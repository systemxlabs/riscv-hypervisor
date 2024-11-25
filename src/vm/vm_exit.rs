#[naked]
#[no_mangle]
pub unsafe extern "C" fn vm_exit() -> ! {
    core::arch::naked_asm!("sret");
}
