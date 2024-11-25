use crate::allocator::PHYS_FRAME_ALLOCATOR;
use crate::config::PAGE_SIZE_4K;
use crate::error::HypervisorResult;
use alloc::vec::Vec;
use spin::lock_api::Mutex;

use crate::mem::{align_up, GuestPageTable, PTEFlags};
use crate::vm::{config, kernel_image, VCpu, VMConfig};

pub static GLOBAL_VMS: Mutex<Vec<VM>> = Mutex::new(Vec::new());

pub fn init_vms() {
    let vm_configs = config::vm_configs();
    for vm_config in vm_configs {
        let vm = VM::new(vm_config).expect("Failed to create VM");
        GLOBAL_VMS.lock().push(vm);
    }
}

pub struct VM {
    vcpus: Vec<VCpu>,
    guest_page_table: GuestPageTable,
    kernel_image: &'static [u8],
    memory_limit: usize,
}

impl VM {
    pub fn new(vm_config: VMConfig) -> HypervisorResult<Self> {
        let kernel_image = kernel_image(vm_config.kernel.as_str());
        let guest_page_table = init_guest_page_table(&vm_config)?;
        // TODO vcpu
        Ok(Self {
            vcpus: Vec::new(),
            guest_page_table,
            kernel_image,
            memory_limit: vm_config.memory_limit,
        })
    }

    pub fn boot(&self) -> ! {
        // TODO
        unreachable!()
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
