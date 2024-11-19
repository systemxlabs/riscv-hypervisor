#![no_std]
#![no_main]
#![feature(naked_functions)]

mod console;
mod lang_items;
mod logging;

use core::arch::{global_asm, naked_asm};

use log::info;

global_asm!(include_str!("entry.asm"));

/// clear BSS segment
pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]
pub fn hmain() -> ! {
    clear_bss();
    logging::init();
    info!("[Hypervisor] Hello, world!");
    sbi_rt::system_reset(sbi_rt::Shutdown, sbi_rt::NoReason);
    unreachable!()
}
