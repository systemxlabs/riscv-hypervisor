use core::sync::atomic::AtomicUsize;

use crate::allocator::PHYS_FRAME_ALLOCATOR;
use crate::config::PAGE_SIZE_4K;
use crate::dtb::MachineMeta;
use crate::error::HypervisorResult;
use crate::pcpu::GLOBAL_PCPUS;
use alloc::vec::Vec;
use log::debug;
use spin::{Mutex, Once};

use crate::mem::{align_down, align_up, GuestPageTable, GuestPhysAddr, HostPhysAddr, PTEFlags};
use crate::vm::{self, kernel_image, vconfig, VMConfig};

use super::VCpu;

pub static GLOBAL_VMS: Once<Vec<VM>> = Once::new();
pub static VM_ID_GENERATOR: AtomicUsize = AtomicUsize::new(0);

pub fn init_vms(meta: &MachineMeta) {
    let vm_configs = vconfig::vm_configs();
    let mut vms = Vec::new();
    let vm_config = vm_configs.get(0).unwrap();
    let vm = VM::new(vm_config.clone(), meta).expect("Failed to create VM");
    vms.push(vm);
    // for vm_config in vm_configs {
    //     let vm = VM::new(vm_config, meta).expect("Failed to create VM");
    //     vms.push(vm);
    // }
    GLOBAL_VMS.call_once(|| vms);
}

pub fn bind_vcpus() {
    let num_pcpu = unsafe { GLOBAL_PCPUS.get_unchecked().len() };
    unsafe {
        let mut idx = 0;
        for vm in GLOBAL_VMS.get_unchecked() {
            for vcpu in vm.vcpus.iter() {
                let vcpu_id = vcpu.lock().vcpu_id;
                let pcpu_id = idx % num_pcpu;
                bind_vcpu_to_pcpu(vm.vm_id, vcpu_id, pcpu_id);
                idx += 1;
            }
        }
    }
}

fn bind_vcpu_to_pcpu(vm_id: usize, vcpu_id: usize, pcpu_id: usize) {
    unsafe {
        let pcpu = GLOBAL_PCPUS.get_unchecked().get_unchecked(pcpu_id);
        let mut vcpus = pcpu.vcpus.lock();
        assert!(!vcpus.contains(&(vm_id, vcpu_id)));
        vcpus.push((vm_id, vcpu_id));
        debug!(
            "[Hypervisor] bind vm {} vcpu {} to pcpu {}",
            vm_id, vcpu_id, pcpu_id
        );
    }
}

pub struct VM {
    pub vm_id: usize,
    pub vcpus: Vec<Mutex<VCpu>>,
    pub guest_page_table: GuestPageTable,
    pub kernel_image: &'static [u8],
    pub memory_limit: usize,
    pub entry: GuestPhysAddr,
}

impl VM {
    pub fn new(vm_config: VMConfig, meta: &MachineMeta) -> HypervisorResult<Self> {
        let kernel_image = kernel_image(vm_config.kernel.as_str());
        let guest_page_table = init_guest_page_table(&vm_config, meta)?;
        let mut vcpus = Vec::new();
        for _ in 0..vm_config.num_vcpu {
            vcpus.push(Mutex::new(VCpu::new()));
        }
        Ok(Self {
            vm_id: VM_ID_GENERATOR.fetch_add(1, core::sync::atomic::Ordering::SeqCst),
            vcpus,
            guest_page_table,
            kernel_image,
            memory_limit: vm_config.memory_limit,
            entry: vm_config.entry.into(),
        })
    }
}

pub fn init_guest_page_table(
    vm_config: &VMConfig,
    meta: &MachineMeta,
) -> HypervisorResult<GuestPageTable> {
    let mut guest_page_table = GuestPageTable::try_new()?;

    let guest_memory_base = align_down(vm_config.entry, PAGE_SIZE_4K);
    let guest_memory_size = align_up(vm_config.memory_limit, PAGE_SIZE_4K);
    let guest_memory_pages = guest_memory_size / PAGE_SIZE_4K;
    PHYS_FRAME_ALLOCATOR
        .lock()
        .alloc_range(guest_memory_base.into(), guest_memory_pages);
    let pte_flags = PTEFlags::R | PTEFlags::W | PTEFlags::X | PTEFlags::V | PTEFlags::U;
    guest_page_table.map_region(
        guest_memory_base.into(),
        guest_memory_base.into(),
        guest_memory_pages,
        pte_flags,
    )?;

    // copy kernel image to guest memory
    let kernel_entry: GuestPhysAddr = vm_config.entry.into();
    let kernel_entry_paddr = guest_page_table.translate(kernel_entry)?;
    let kernel_image = kernel_image(vm_config.kernel.as_str());
    unsafe {
        core::ptr::copy_nonoverlapping(
            kernel_image.as_ptr(),
            kernel_entry_paddr.as_usize() as *mut u8,
            kernel_image.len(),
        );
    }

    // map mmio
    for virt_dev in meta.virtio.iter() {
        let pte_flags = PTEFlags::V | PTEFlags::R | PTEFlags::W | PTEFlags::U | PTEFlags::X;
        guest_page_table.map_region(
            virt_dev.base_address.into(),
            virt_dev.base_address.into(),
            virt_dev.size / PAGE_SIZE_4K,
            pte_flags,
        )?;
    }

    let guest_memory_base_paddr: HostPhysAddr = guest_memory_base.into();
    assert_eq!(
        guest_page_table
            .query_page(guest_memory_base.into())
            .unwrap(),
        (guest_memory_base_paddr, pte_flags)
    );
    if guest_memory_size > PAGE_SIZE_4K {
        assert_eq!(
            guest_page_table
                .query_page((guest_memory_base + PAGE_SIZE_4K).into())
                .unwrap(),
            (guest_memory_base_paddr + PAGE_SIZE_4K, pte_flags)
        );
    }
    assert_eq!(
        guest_page_table
            .translate((guest_memory_base + 1).into())
            .unwrap(),
        guest_memory_base_paddr + 1
    );
    assert_eq!(
        guest_page_table
            .translate((guest_memory_base + guest_memory_size - 1).into())
            .unwrap(),
        guest_memory_base_paddr + guest_memory_size - 1
    );

    Ok(guest_page_table)
}
