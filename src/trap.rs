use log::info;

use crate::config::PAGE_SIZE_4K;

// TODO this is a temporary solution
static TRAP_STACK: [u8; 10 * PAGE_SIZE_4K] = [0u8; 10 * PAGE_SIZE_4K];

pub fn set_hypervisor_trap_entry() {
    unsafe {
        riscv::register::stvec::write(
            _trap_vector_base as usize,
            riscv::register::stvec::TrapMode::Direct,
        );
        riscv::register::sscratch::write(TRAP_STACK.as_ptr() as usize + TRAP_STACK.len());
    }
}

#[naked]
#[no_mangle]
pub unsafe extern "C" fn _trap_vector_base() -> ! {
    core::arch::naked_asm!(
        "csrrw sp, sscratch, sp",
        "addi sp, sp, -{trapframe_size}",
        // save general registers
        "sd ra, 0*8(sp)",
        "sd t0, 4*8(sp)",
        "sd t1, 5*8(sp)",
        "sd t2, 6*8(sp)",
        "sd s0, 7*8(sp)",
        "sd s1, 8*8(sp)",
        "sd a0, 9*8(sp)",
        "sd a1, 10*8(sp)",
        "sd a2, 11*8(sp)",
        "sd a3, 12*8(sp)",
        "sd a4, 13*8(sp)",
        "sd a5, 14*8(sp)",
        "sd a6, 15*8(sp)",
        "sd a7, 16*8(sp)",
        "sd s2, 17*8(sp)",
        "sd s3, 18*8(sp)",
        "sd s4, 19*8(sp)",
        "sd s5, 20*8(sp)",
        "sd s6, 21*8(sp)",
        "sd s7, 22*8(sp)",
        "sd s8, 23*8(sp)",
        "sd s9, 24*8(sp)",
        "sd s10, 25*8(sp)",
        "sd s11, 26*8(sp)",
        "sd t3, 27*8(sp)",
        "sd t4, 28*8(sp)",
        "sd t5, 29*8(sp)",
        "sd t6, 30*8(sp)",

        // save supervisor registers
        "csrr    t0, sepc",
        "csrr    t1, sstatus",
        "sd     t0, 31*8(sp)",
        "sd     t1, 32*8(sp)",

        // call trap_handler
        "mv a0, sp",
        "call trap_handler",

        // restore supervisor registers
        "ld     t0, 31*8(sp)",
        "ld     t1, 32*8(sp)",
        "csrw    sepc, t0",
        "csrw    sstatus, t1",

        // restore general registers
        "ld ra, 0*8(sp)",
        "ld t0, 4*8(sp)",
        "ld t1, 5*8(sp)",
        "ld t2, 6*8(sp)",
        "ld s0, 7*8(sp)",
        "ld s1, 8*8(sp)",
        "ld a0, 9*8(sp)",
        "ld a1, 10*8(sp)",
        "ld a2, 11*8(sp)",
        "ld a3, 12*8(sp)",
        "ld a4, 13*8(sp)",
        "ld a5, 14*8(sp)",
        "ld a6, 15*8(sp)",
        "ld a7, 16*8(sp)",
        "ld s2, 17*8(sp)",
        "ld s3, 18*8(sp)",
        "ld s4, 19*8(sp)",
        "ld s5, 20*8(sp)",
        "ld s6, 21*8(sp)",
        "ld s7, 22*8(sp)",
        "ld s8, 23*8(sp)",
        "ld s9, 24*8(sp)",
        "ld s10, 25*8(sp)",
        "ld s11, 26*8(sp)",
        "ld t3, 27*8(sp)",
        "ld t4, 28*8(sp)",
        "ld t5, 29*8(sp)",
        "ld t6, 30*8(sp)",

        "addi sp, sp, {trapframe_size}",
        "sret",
        trapframe_size = const core::mem::size_of::<TrapFrame>(),
    );
}

#[no_mangle]
pub fn trap_handler(tf: &mut TrapFrame) {
    panic!("trap_handler");
}

/// General registers of RISC-V.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct GeneralRegisters {
    pub ra: usize,
    pub sp: usize,
    pub gp: usize, // only valid for user traps
    pub tp: usize, // only valid for user traps
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub s0: usize,
    pub s1: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
}

/// Saved registers when a trap (interrupt or exception) occurs.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct TrapFrame {
    /// All general registers.
    pub regs: GeneralRegisters,
    /// Supervisor Exception Program Counter.
    pub sepc: usize,
    /// Supervisor Status Register.
    pub sstatus: usize,
}
