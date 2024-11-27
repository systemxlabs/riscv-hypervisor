use log::debug;

use crate::vm::VCpu;

pub fn handle_sbi_call(vcpu: &mut VCpu) {
    let a0 = vcpu.guest_cpu_state.gprs[10];
    let a1 = vcpu.guest_cpu_state.gprs[11];
    let a7 = vcpu.guest_cpu_state.gprs[17];
    debug!(
        "[Hypervisor] VSuperEcall a0: {:#x}, a1: {:#x}, a7: {:#x}",
        a0, a1, a7
    );
    match a7 {
        sbi_spec::legacy::LEGACY_CONSOLE_PUTCHAR => handle_console_putchar(vcpu, a0),
        sbi_spec::legacy::LEGACY_SHUTDOWN => handle_shutdown(vcpu),
        sbi_spec::srst::EID_SRST => handle_reset(vcpu),
        _ => panic!("[Hypervisor] Unsupported SBI call!"),
    }
}

fn handle_console_putchar(vcpu: &mut VCpu, c: usize) {
    let ret = sbi_rt::legacy::console_putchar(c);
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
