mod config;
mod vcpu;

pub use config::*;
pub use vcpu::*;

use core::sync::atomic::AtomicBool;

use alloc::vec::Vec;
use vcpu::VCpu;

use crate::mem::PageTable;

pub struct VM {
    running: AtomicBool,
    vcpus: Vec<VCpu>,
    page_table: PageTable,
}
