mod hcounteren;
mod hedeleg;
mod hgatp;
mod hideleg;
mod hstatus;
pub mod htinst;
pub mod htval;
mod hvip;
mod scause;
mod sstatus;
mod vsstatus;

pub use hcounteren::*;
pub use hedeleg::*;
pub use hgatp::*;
pub use hideleg::*;
pub use hstatus::*;
pub use hvip::*;
pub use scause::*;
pub use sstatus::*;
pub use vsstatus::*;

use log::debug;

pub fn init_csrs() {
    let mut hstatus = Hstatus::read();
    hstatus.set_spv(true);
    hstatus.set_spvp(true);
    hstatus.write();
    debug!("[HyperVisor] hstatus: {:?}", Hstatus::read());

    let mut hedeleg = Hedeleg::read();
    hedeleg.set_env_call_from_u_or_vu(true);
    hedeleg.set_load_page_fault(true);
    hedeleg.set_store_page_fault(true);
    hedeleg.set_illegal_inst(true);
    hedeleg.set_inst_access_fault(true);
    hedeleg.write();
    debug!("[HyperVisor] hedeleg: {:?}", Hedeleg::read());

    let mut hideleg = Hideleg::read();
    hideleg.set_vs_timer_interrupt(true);
    hideleg.set_vs_external_interrupt(true);
    hideleg.write();
    debug!("[HyperVisor] hideleg: {:?}", Hideleg::read());

    let hcounteren = Hcounteren::from_bits(0xffff_ffff);
    hcounteren.write();
    debug!("[HyperVisor] hcounteren: {:?}", Hcounteren::read());

    let mut hvip = Hvip::read();
    hvip.set_vs_external_interrupt(false);
    hvip.set_vs_software_interrupt(false);
    hvip.set_vs_timer_interrupt(false);
    hvip.write();
    debug!("[Hypervisor] hvip: {:?}", Hvip::read());

    unsafe {
        //     riscv::register::sie::set_sext();
        //     riscv::register::sie::set_ssoft();
        riscv::register::sie::set_stimer();
        //     debug!("[Hypervisor] sie: {:?}", riscv::register::sie::read());
    }
}
