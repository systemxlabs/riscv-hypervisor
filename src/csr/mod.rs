mod hedeleg;
mod hgatp;
mod hideleg;
mod hstatus;
pub mod htinst;
pub mod htval;
mod scause;
mod sstatus;
mod vsstatus;

pub use hedeleg::*;
pub use hgatp::*;
pub use hideleg::*;
pub use hstatus::*;
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
}
