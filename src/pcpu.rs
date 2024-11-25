use alloc::vec::Vec;
use log::{debug, info};
use spin::{Mutex, Once};

use crate::{
    allocator::PHYS_FRAME_ALLOCATOR,
    config::{PAGE_SIZE_4K, PCPU_STACK_SIZE},
    error::HypervisorResult,
    mem::{HostPhysAddr, HostVirtAddr},
};

pub static GLOBAL_PCPUS: Once<Vec<PCpu>> = Once::new();

#[derive(Debug)]
pub struct PCpu {
    pub hart_id: usize,
    pub stack_top: HostPhysAddr,
    pub vcpus: Mutex<Vec<usize>>,
}

pub fn init_pcpus(boot_hart_id: usize) {
    assert_eq!(boot_hart_id, 0);
    // TODO: get cpu info by device tree
    let cpu_nums: usize = 1;
    let mut pcpus = Vec::new();
    for cpu_id in 0..cpu_nums {
        let stack_top = PHYS_FRAME_ALLOCATOR
            .lock()
            .alloc_frames(
                (PCPU_STACK_SIZE + PAGE_SIZE_4K - 1) / PAGE_SIZE_4K,
                PAGE_SIZE_4K,
            )
            .expect("Failed to alloc pcpu stack");
        let pcpu = PCpu {
            hart_id: cpu_id,
            stack_top,
            vcpus: Mutex::new(Vec::new()),
        };
        info!("[Hypervisor] init pcpu: {:?}", pcpu);
        pcpus.push(pcpu);
    }
    GLOBAL_PCPUS.call_once(|| pcpus);

    // Initialize TP register and set this CPU online to be consistent with secondary CPUs.
    setup_this_cpu(boot_hart_id);
    info!("[Hypervisor] this cpu: {:?}", this_cpu());
}

pub fn setup_this_cpu(hart_id: usize) {
    unsafe {
        // Safe since we're the only users of TP.
        core::arch::asm!("mv tp, {rs}", rs = in(reg) hart_id)
    };
}

/// Returns this CPU's `PCpu` structure.
pub fn this_cpu() -> &'static PCpu {
    let tp: u64;
    unsafe { core::arch::asm!("mv {rd}, tp", rd = out(reg) tp) };
    unsafe { GLOBAL_PCPUS.get_unchecked().get_unchecked(tp as usize) }
}

pub fn run_vcpus() -> ! {
    let pcpu = this_cpu();
    loop {}
}
