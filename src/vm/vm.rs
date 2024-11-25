use core::sync::atomic::AtomicUsize;

use crate::allocator::PHYS_FRAME_ALLOCATOR;
use crate::config::PAGE_SIZE_4K;
use crate::error::HypervisorResult;
use crate::pcpu::GLOBAL_PCPUS;
use alloc::vec::Vec;
use log::debug;
use spin::Once;

use crate::mem::{align_up, GuestPageTable, PTEFlags};
use crate::vm::{config, kernel_image, VMConfig};

use super::VCpu;

pub static GLOBAL_VMS: Once<Vec<VM>> = Once::new();
pub static VM_ID_GENERATOR: AtomicUsize = AtomicUsize::new(0);

pub fn init_vms() {
    let vm_configs = config::vm_configs();
    let mut vms = Vec::new();
    for vm_config in vm_configs {
        let vm = VM::new(vm_config).expect("Failed to create VM");
        vms.push(vm);
    }
    GLOBAL_VMS.call_once(|| vms);
}

pub fn bind_vcpus() {
    let num_pcpu = unsafe { GLOBAL_PCPUS.get_unchecked().len() };
    unsafe {
        let mut idx = 0;
        for vm in GLOBAL_VMS.get_unchecked() {
            for vcpu_id in vm.vcpus.iter().map(|vcpu| vcpu.vcpu_id) {
                let pcpu_id = idx % num_pcpu;
                bind_vcpu_to_pcpu(vcpu_id, pcpu_id);
                idx += 1;
            }
        }
    }
}

fn bind_vcpu_to_pcpu(vcpu_id: usize, pcpu_id: usize) {
    unsafe {
        let pcpu = GLOBAL_PCPUS.get_unchecked().get_unchecked(pcpu_id);
        pcpu.vcpus.lock().push(vcpu_id);
        debug!("[Hypervisor] bind vcpu {} to pcpu {}", vcpu_id, pcpu_id);
    }
}

pub struct VM {
    vm_id: usize,
    vcpus: Vec<VCpu>,
    guest_page_table: GuestPageTable,
    kernel_image: &'static [u8],
    memory_limit: usize,
}

impl VM {
    pub fn new(vm_config: VMConfig) -> HypervisorResult<Self> {
        let kernel_image = kernel_image(vm_config.kernel.as_str());
        let guest_page_table = init_guest_page_table(&vm_config)?;
        let mut vcpus = Vec::new();
        for _ in 0..vm_config.num_vcpu {
            vcpus.push(VCpu::new());
        }
        Ok(Self {
            vm_id: VM_ID_GENERATOR.fetch_add(1, core::sync::atomic::Ordering::SeqCst),
            vcpus,
            guest_page_table,
            kernel_image,
            memory_limit: vm_config.memory_limit,
        })
    }
}

pub fn init_guest_page_table(vm_config: &VMConfig) -> HypervisorResult<GuestPageTable> {
    let mut guest_page_table = GuestPageTable::try_new()?;
    let memory_size = align_up(vm_config.memory_limit, PAGE_SIZE_4K);
    let paddr = PHYS_FRAME_ALLOCATOR
        .lock()
        .alloc_frames(memory_size / PAGE_SIZE_4K, PAGE_SIZE_4K)?;
    let pte_flags = PTEFlags::R | PTEFlags::W | PTEFlags::X | PTEFlags::V | PTEFlags::U;
    guest_page_table.map_region(0.into(), paddr, memory_size / PAGE_SIZE_4K, pte_flags)?;

    assert_eq!(
        guest_page_table.query_page(0.into()).unwrap(),
        (paddr, pte_flags)
    );
    if memory_size > PAGE_SIZE_4K {
        assert_eq!(
            guest_page_table
                .query_page((0 + PAGE_SIZE_4K).into())
                .unwrap(),
            (paddr + PAGE_SIZE_4K, pte_flags)
        );
    }
    assert_eq!(guest_page_table.translate(1.into()).unwrap(), paddr + 1);
    assert_eq!(
        guest_page_table
            .translate((memory_size - 1).into())
            .unwrap(),
        paddr + memory_size - 1
    );

    Ok(guest_page_table)
}
