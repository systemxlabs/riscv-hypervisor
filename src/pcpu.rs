use log::debug;
use spin::Once;

use crate::{
    allocator::PHYS_FRAME_ALLOCATOR,
    config::{PAGE_SIZE_4K, PCPU_STACK_SIZE},
    error::HypervisorResult,
    mem::{HostPhysAddr, HostVirtAddr},
};

pub struct PCpu {
    pub hart_id: usize,
    pub stack_top: HostPhysAddr,
}

static PCPU_BASE: Once<HostPhysAddr> = Once::new();

pub fn init_pcpus(boot_hart_id: usize) {
    assert_eq!(boot_hart_id, 0);
    // TODO: get cpu info by device tree
    let cpu_nums: usize = 1;
    let pcpu_size = size_of::<PCpu>();
    debug!("pcpu size: {:#x}", pcpu_size);
    let pcpu_base = PHYS_FRAME_ALLOCATOR
        .lock()
        .alloc_frames((pcpu_size + PAGE_SIZE_4K - 1) / PAGE_SIZE_4K)
        .expect("Failed to alloc pcpu pages");
    PCPU_BASE.call_once(|| pcpu_base);
    for cpu_id in 0..cpu_nums {
        let stack_top = PHYS_FRAME_ALLOCATOR
            .lock()
            .alloc_frames((PCPU_STACK_SIZE + PAGE_SIZE_4K - 1) / PAGE_SIZE_4K)
            .expect("Failed to alloc pcpu stack");
        let pcpu = PCpu {
            hart_id: cpu_id,
            stack_top,
        };
        let pcpu_addr = pcpu_base.as_usize() + cpu_id * pcpu_size;
        unsafe {
            core::ptr::write_volatile(pcpu_addr as *mut PCpu, pcpu);
        }
    }

    // Initialize TP register and set this CPU online to be consistent with secondary CPUs.
    setup_this_cpu(boot_hart_id);
}

/// Initializes the TP pointer to point to PCpu data.
pub fn setup_this_cpu(hart_id: usize) {
    // Load TP with address of pur PCpu struct.
    let tp = pcpu_ptr(hart_id) as usize;
    unsafe {
        // Safe since we're the only users of TP.
        core::arch::asm!("mv tp, {rs}", rs = in(reg) tp)
    };
}

/// Returns this CPU's `PCpu` structure.
pub fn this_cpu() -> &'static mut PCpu {
    // Make sure PCpu has been set up.
    assert!(PCPU_BASE.get().is_some());
    let tp: u64;
    unsafe { core::arch::asm!("mv {rd}, tp", rd = out(reg) tp) };
    let pcpu_ptr = tp as *mut PCpu;
    let pcpu = unsafe { pcpu_ptr.as_mut().unwrap() };
    pcpu
}

fn pcpu_ptr(cpu_id: usize) -> *const PCpu {
    let pcpu_addr = PCPU_BASE.get().unwrap().as_usize() + cpu_id * size_of::<PCpu>();
    pcpu_addr as *const PCpu
}
