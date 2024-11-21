#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(alloc_error_handler)]

extern crate alloc;

mod allocator;
mod config;
mod console;
mod error;
mod lang_items;
mod logging;
mod mem;

use core::arch::naked_asm;

use crate::config::BOOT_STACK_SIZE;
use crate::mem::page_table::init_page_table;
use crate::mem::region::{map_free_memory, map_hypervisor_image};
use alloc::vec::Vec;
use allocator::{frame::init_frame_allocator, heap::init_heap_allocator};
use log::info;
use mem::init_mmu;

#[link_section = ".bss.stack"]
static BOOT_STACK: [u8; BOOT_STACK_SIZE] = [0u8; BOOT_STACK_SIZE];

#[link_section = ".text.entry"]
#[export_name = "_start"]
#[naked]
pub unsafe extern "C" fn start() -> ! {
    // PC = 0x8020_0000
    // a0 = hartid
    // a1 = dtb
    naked_asm!(
        "la sp, {boot_stack}",  // load addr of the symbol `BOOT_STACK` to sp
        "li t0, {boot_stack_size}",  // load immediate `BOOT_STACK_SIZE` to t0
        "add sp, sp, t0",  // setup boot stack
        "call hmain",
        boot_stack = sym BOOT_STACK,
        boot_stack_size = const BOOT_STACK_SIZE,
    )
}

#[no_mangle]
pub fn hmain(hart_id: usize, dtb: usize) -> ! {
    clear_bss();
    logging::init();
    info!("[Hypervisor] Hello, world!");
    info!("[HyperVisor] hart_id: {}, dtb: {:#x}", hart_id, dtb);

    // detect extension
    if sbi_rt::probe_extension(sbi_rt::Hsm).is_unavailable() {
        panic!("no HSM extension exist on current SBI environment");
    }

    // init frame
    init_frame_allocator();

    // init heap
    init_heap_allocator();

    // init page table
    init_page_table();
    map_hypervisor_image();
    map_free_memory();

    // enable mmu
    init_mmu();

    let mut v = Vec::new();
    for i in 0..5000 {
        v.push(i);
    }
    assert_eq!(v.len(), 5000);

    sbi_rt::system_reset(sbi_rt::Shutdown, sbi_rt::NoReason);
    unreachable!()
}

/// clear BSS segment
pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
