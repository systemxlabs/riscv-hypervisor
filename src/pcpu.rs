use alloc::vec::Vec;
use log::{debug, info};
use riscv::register::sstatus;
use spin::{Mutex, Once};

use crate::{
    allocator::PHYS_FRAME_ALLOCATOR,
    config::{PAGE_SIZE_4K, PCPU_STACK_SIZE},
    csr,
    error::HypervisorResult,
    mem::{HostPhysAddr, HostVirtAddr},
    sbi,
    vm::{VCpu, _vm_entry, GLOBAL_VMS},
};

pub static GLOBAL_PCPUS: Once<Vec<PCpu>> = Once::new();

#[derive(Debug)]
pub struct PCpu {
    pub hart_id: usize,
    pub stack_top: HostPhysAddr,
    // vec of (vm_id, vcpu_id)
    pub vcpus: Mutex<Vec<(usize, usize)>>,
}

impl PCpu {
    pub fn run(&self) {
        let (vm_id, vcpu_id) = self.vcpus.lock()[0];
        let vm = unsafe { GLOBAL_VMS.get_unchecked().get_unchecked(vm_id) };
        let mut vcpu = unsafe { vm.vcpus.get_unchecked(vcpu_id).lock() };

        let hstatus = csr::Hstatus::read();
        vcpu.guest_cpu_state.hstatus = hstatus.bits();

        let mut sstatus = csr::Sstatus::read();
        sstatus.set_spp(true);
        vcpu.guest_cpu_state.sstatus = sstatus.bits();

        vcpu.guest_cpu_state.sepc = vm.entry.as_usize();

        let gpt_root = vm.guest_page_table.root_paddr().as_usize();
        let mut hgatp = csr::Hgatp::read();
        hgatp.set_mode(csr::Mode::Sv39x4);
        hgatp.set_ppn(gpt_root >> 12);
        hgatp.write();

        unsafe {
            core::arch::asm!("hfence.gvma");
        }

        info!("[Hypervisor] run vcpu: {:?}", vcpu_id);
        while !run_vcpu(&mut vcpu) {}
    }
}

fn run_vcpu(vcpu: &mut VCpu) -> bool {
    unsafe {
        _vm_entry(vcpu);
    }

    vmexit_handler(vcpu)
}

fn vmexit_handler(vcpu: &mut VCpu) -> bool {
    let scause = csr::Scause::read();
    debug!("[Hypervisor] scause: {:?}", scause.cause());
    let stval = riscv::register::stval::read();
    debug!("[Hypervisor] stval: {:#x}", stval);

    match scause.cause() {
        csr::Trap::Exception(csr::Exception::VirtualSupervisorEnvCall) => {
            info!(
                "VirtualSupervisorEnvCall: stval: {:#x}, sepc: {:#x}, htval: {:#x}, htinst: {:#x}",
                riscv::register::stval::read(),
                vcpu.guest_cpu_state.sepc,
                csr::htval::read(),
                csr::htinst::read(),
            );
            let a7 = vcpu.guest_cpu_state.gprs[17];
            sbi::handle_sbi_call(vcpu);
            vcpu.guest_cpu_state.sepc += 4;
            if a7 == 8 {
                info!("[Hypervisor] Shutdown vm normally!");
                return true;
            }
            return false;
        }
        csr::Trap::Exception(csr::Exception::LoadGuestPageFault) => {
            info!(
                "LoadGuestPageFault: stval: {:#x}, sepc: {:#x}, htval: {:#x}, htinst: {:#x}",
                riscv::register::stval::read(),
                vcpu.guest_cpu_state.sepc,
                csr::htval::read(),
                csr::htinst::read(),
            );
        }
        _ => {
            panic!("Unknown trap: {:?}", scause.cause());
        }
    }
    true
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
