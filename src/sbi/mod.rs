use log::info;
use sbi_rt::SbiRet;

use crate::vm::VCpu;

pub fn handle_sbi_call(vcpu: &mut VCpu) -> SbiRet {
    let a0 = vcpu.guest_cpu_state.gprs[10];
    let a1 = vcpu.guest_cpu_state.gprs[11];
    let a7 = vcpu.guest_cpu_state.gprs[17];
    info!(
        "[Hypervisor] VSuperEcall a0: {:#x}, a1: {:#x}, a7: {:#x}",
        a0, a1, a7
    );
    match a7 {
        1 => handle_console_putchar(a0),
        8 => handle_reset(),
        _ => SbiRet::not_supported(),
    }
}

fn handle_console_putchar(c: usize) -> SbiRet {
    sbi_rt::legacy::console_putchar(c);
    SbiRet::success(0)
}

fn handle_reset() -> SbiRet {
    // TODO
    info!("[Hypervisor] Reset vm!");
    SbiRet::success(0)
}
