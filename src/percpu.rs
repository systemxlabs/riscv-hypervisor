use log::debug;
use spin::Once;

use crate::{
    allocator::PHYS_FRAME_ALLOCATOR,
    config::{PAGE_SIZE_4K, PERCPU_STACK_SIZE},
    error::HypervisorResult,
    mem::{PhysAddr, VirtAddr},
};

pub struct Percpu {
    pub hart_id: usize,
    pub stack_top: PhysAddr,
}

static PERCPU_BASE: Once<PhysAddr> = Once::new();

pub fn init_percpus(boot_hart_id: usize) {
    assert_eq!(boot_hart_id, 0);
    // TODO: get cpu info by device tree
    let cpu_nums: usize = 1;
    let percpu_size = core::mem::size_of::<Percpu>();
    debug!("percpu size: {:#x}", percpu_size);
    let percpu_base = PHYS_FRAME_ALLOCATOR
        .lock()
        .alloc_frames((percpu_size + PAGE_SIZE_4K - 1) / PAGE_SIZE_4K)
        .expect("Failed to alloc percpu pages");
    PERCPU_BASE.call_once(|| percpu_base);
    for cpu_id in 0..cpu_nums {
        let stack_top = PHYS_FRAME_ALLOCATOR
            .lock()
            .alloc_frames((PERCPU_STACK_SIZE + PAGE_SIZE_4K - 1) / PAGE_SIZE_4K)
            .expect("Failed to alloc percpu stack");
        let percpu = Percpu {
            hart_id: cpu_id,
            stack_top: stack_top,
        };
        let percpu_addr = percpu_base.as_usize() + cpu_id * percpu_size;
        unsafe {
            core::ptr::write_volatile(percpu_addr as *mut Percpu, percpu);
        }
    }

    // Initialize TP register and set this CPU online to be consistent with secondary CPUs.
    setup_this_cpu(boot_hart_id);
}

/// Initializes the TP pointer to point to PerCpu data.
pub fn setup_this_cpu(hart_id: usize) {
    // Load TP with address of pur PerCpu struct.
    let tp = percpu_ptr(hart_id) as usize;
    unsafe {
        // Safe since we're the only users of TP.
        core::arch::asm!("mv tp, {rs}", rs = in(reg) tp)
    };
}

/// Returns this CPU's `PerCpu` structure.
pub fn this_cpu() -> &'static mut Percpu {
    // Make sure PerCpu has been set up.
    assert!(PERCPU_BASE.get().is_some());
    let tp: u64;
    unsafe { core::arch::asm!("mv {rd}, tp", rd = out(reg) tp) };
    let percpu_ptr = tp as *mut Percpu;
    let percpu = unsafe { percpu_ptr.as_mut().unwrap() };
    percpu
}

fn percpu_ptr(cpu_id: usize) -> *const Percpu {
    let pcpu_addr = PERCPU_BASE.get().unwrap().as_usize() + cpu_id * core::mem::size_of::<Percpu>();
    pcpu_addr as *const Percpu
}
