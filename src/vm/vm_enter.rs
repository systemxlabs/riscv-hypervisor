use super::{VCpu, VCpuRegs};

#[naked]
#[no_mangle]
pub unsafe extern "C" fn vm_enter(vcpu: &mut VCpu) -> ! {
    core::arch::naked_asm!("sret");
}
