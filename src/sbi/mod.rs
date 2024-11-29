use log::debug;

use crate::{csr, vm::VCpu};

pub fn handle_sbi_call(vcpu: &mut VCpu) {
    let a7 = vcpu.guest_cpu_state.gprs[17];
    match a7 {
        sbi_spec::legacy::LEGACY_CONSOLE_PUTCHAR => handle_console_putchar(vcpu),
        sbi_spec::legacy::LEGACY_CONSOLE_GETCHAR => handle_console_getchar(vcpu),
        sbi_spec::legacy::LEGACY_SHUTDOWN => handle_shutdown(vcpu),
        sbi_spec::srst::EID_SRST => handle_reset(vcpu),
        sbi_spec::time::EID_TIME => handle_time(vcpu),
        _ => panic!("[Hypervisor] Unsupported SBI call!"),
    }
}

fn handle_console_putchar(vcpu: &mut VCpu) {
    let a0 = vcpu.guest_cpu_state.gprs[10];
    let ret = sbi_rt::legacy::console_putchar(a0);
    vcpu.guest_cpu_state.gprs[10] = ret;
}

fn handle_console_getchar(vcpu: &mut VCpu) {
    let ret = sbi_rt::legacy::console_getchar();
    vcpu.guest_cpu_state.gprs[10] = ret;
}

fn handle_shutdown(vcpu: &mut VCpu) {
    // TODO
    debug!("[Hypervisor] Shutdown vm!");
    vcpu.guest_cpu_state.gprs[10] = 0;
}

fn handle_reset(vcpu: &mut VCpu) {
    // TODO
    // debug!("[Hypervisor] Reset vm!");
    vcpu.guest_cpu_state.gprs[10] = 0;
}

fn handle_time(vcpu: &mut VCpu) {
    debug!("[Hypervisor] Time!");
    let a0 = vcpu.guest_cpu_state.gprs[10];
    let a6 = vcpu.guest_cpu_state.gprs[16];

    match a6 {
        sbi_spec::time::SET_TIMER => {
            let ret = sbi_rt::set_timer(a0 as u64);

            unsafe {
                riscv::register::sie::set_stimer();
            }
            let mut hvip = csr::Hvip::read();
            hvip.set_vs_timer_interrupt(false);
            hvip.write();

            vcpu.guest_cpu_state.gprs[10] = ret.error;
            vcpu.guest_cpu_state.gprs[11] = ret.value;
        }
        _ => panic!("[Hypervisor] Unsupported TIME SBI call!"),
    }
}
