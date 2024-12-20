#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(alloc_error_handler)]

extern crate alloc;

mod allocator;
mod config;
mod console;
mod csr;
mod dtb;
mod error;
mod lang_items;
mod logging;
mod mem;
mod pcpu;
mod sbi;
mod trap;
mod vm;

use crate::config::BOOT_STACK_SIZE;
use log::{debug, info};
use vm::GLOBAL_VMS;

#[link_section = ".bss.stack"]
static BOOT_STACK: [u8; BOOT_STACK_SIZE] = [0u8; BOOT_STACK_SIZE];

#[link_section = ".text.entry"]
#[export_name = "_start"]
#[naked]
pub unsafe extern "C" fn start() -> ! {
    // PC = 0x8020_0000
    // a0 = hartid
    // a1 = dtb
    core::arch::naked_asm!(
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
    trap::set_hypervisor_trap_entry();
    info!("[Hypervisor] Hello, world!");
    info!("[HyperVisor] hart_id: {}, dtb: {:#x}", hart_id, dtb);

    // detect extension
    if sbi_rt::probe_extension(sbi_rt::Hsm).is_unavailable() {
        panic!("no HSM extension exist on current SBI environment");
    }

    let machine_meta = dtb::MachineMeta::parse(dtb);
    info!("[HyperVisor] machine meta: {:#x?}", machine_meta);

    allocator::init_frame_allocator(&machine_meta);
    allocator::init_heap_allocator();

    mem::init_hypervisor_page_table(&machine_meta);
    mem::enable_mmu();
    allocator::heap_test();

    pcpu::init_pcpus(hart_id, &machine_meta);

    vm::init_vms(&machine_meta);
    vm::bind_vcpus();

    csr::init_csrs();

    let pcpu = pcpu::this_cpu();
    pcpu.run();

    info!("[HyperVisor] exited");
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
