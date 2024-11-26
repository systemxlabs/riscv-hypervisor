use super::VCpu;
use crate::{vcpu_guest_csr_offset, vcpu_hyp_csr_offset};

#[naked]
#[no_mangle]
pub unsafe extern "C" fn _vm_exit() -> ! {
    core::arch::naked_asm!(
        // Pull GuestInfo out of sscratch, swapping with guest's a0
        "csrrw a0, sscratch, a0",

        // Save guest GPRs
        "sd   x1, ({guest_x1})(a0)",
        "sd   x2, ({guest_x2})(a0)",
        "sd   x3, ({guest_x3})(a0)",
        "sd   x4, ({guest_x4})(a0)",
        "sd   x5, ({guest_x5})(a0)",
        "sd   x6, ({guest_x6})(a0)",
        "sd   x7, ({guest_x7})(a0)",
        "sd   x8, ({guest_x8})(a0)",
        "sd   x9, ({guest_x9})(a0)",
        // not store a0
        "sd   x11, ({guest_x11})(a0)",
        "sd   x12, ({guest_x12})(a0)",
        "sd   x13, ({guest_x13})(a0)",
        "sd   x14, ({guest_x14})(a0)",
        "sd   x15, ({guest_x15})(a0)",
        "sd   x16, ({guest_x16})(a0)",
        "sd   x17, ({guest_x17})(a0)",
        "sd   x18, ({guest_x18})(a0)",
        "sd   x19, ({guest_x19})(a0)",
        "sd   x20, ({guest_x20})(a0)",
        "sd   x21, ({guest_x21})(a0)",
        "sd   x22, ({guest_x22})(a0)",
        "sd   x23, ({guest_x23})(a0)",
        "sd   x24, ({guest_x24})(a0)",
        "sd   x25, ({guest_x25})(a0)",
        "sd   x26, ({guest_x26})(a0)",
        "sd   x27, ({guest_x27})(a0)",
        "sd   x28, ({guest_x28})(a0)",
        "sd   x29, ({guest_x29})(a0)",
        "sd   x30, ({guest_x30})(a0)",
        "sd   x31, ({guest_x31})(a0)",

        // Save Guest a0 after recovering from sscratch
        "csrr  t0, sscratch",
        "sd    t0, ({guest_x10})(a0)",

        // Swap in hypervisor CSRs
        "ld    t1, ({hyp_sstatus})(a0)",
        "csrrw t1, sstatus, t1",
        "sd    t1, ({guest_sstatus})(a0)",

        "csrr  t1, hstatus",
        "sd    t1, ({guest_hstatus})(a0)",

        "ld    t1, ({hyp_scounteren})(a0)",
        "csrrw t1, scounteren, t1",
        "sd    t1, ({guest_scounteren})(a0)",

        "ld    t1, ({hyp_stvec})(a0)",
        "csrw  stvec, t1",

        "ld    t1, ({hyp_sscratch})(a0)",
        "csrw  sscratch, t1",

        // Save guest EPC
        "csrr  t1, sepc",
        "sd    t1, ({guest_sepc})(a0)",

        // Restore hypervisor GPRs
        // ra
        "ld   x1, ({hyp_x1})(a0)",
        // sp
        "ld   x2, ({hyp_x2})(a0)",
        // gp
        "ld   x3, ({hyp_x3})(a0)",
        // tp
        "ld   x4, ({hyp_x4})(a0)",
        // s0-s1
        "ld   x8, ({hyp_x8})(a0)",
        "ld   x9, ({hyp_x9})(a0)",
        // a0-a7
        "ld   x10, ({hyp_x10})(a0)",
        "ld   x11, ({hyp_x11})(a0)",
        "ld   x12, ({hyp_x12})(a0)",
        "ld   x13, ({hyp_x13})(a0)",
        "ld   x14, ({hyp_x14})(a0)",
        "ld   x15, ({hyp_x15})(a0)",
        "ld   x16, ({hyp_x16})(a0)",
        "ld   x17, ({hyp_x17})(a0)",
        // s2-s11
        "ld   x18, ({hyp_x18})(a0)",
        "ld   x19, ({hyp_x19})(a0)",
        "ld   x20, ({hyp_x20})(a0)",
        "ld   x21, ({hyp_x21})(a0)",
        "ld   x22, ({hyp_x22})(a0)",
        "ld   x23, ({hyp_x23})(a0)",
        "ld   x24, ({hyp_x24})(a0)",
        "ld   x25, ({hyp_x25})(a0)",
        "ld   x26, ({hyp_x26})(a0)",
        "ld   x27, ({hyp_x27})(a0)",
        "ret",
        guest_x1 = const VCpu::guest_gpr_offset(1),
        guest_x2 = const VCpu::guest_gpr_offset(2),
        guest_x3 = const VCpu::guest_gpr_offset(3),
        guest_x4 = const VCpu::guest_gpr_offset(4),
        guest_x5 = const VCpu::guest_gpr_offset(5),
        guest_x6 = const VCpu::guest_gpr_offset(6),
        guest_x7 = const VCpu::guest_gpr_offset(7),
        guest_x8 = const VCpu::guest_gpr_offset(8),
        guest_x9 = const VCpu::guest_gpr_offset(9),
        guest_x10 = const VCpu::guest_gpr_offset(10),
        guest_x11 = const VCpu::guest_gpr_offset(11),
        guest_x12 = const VCpu::guest_gpr_offset(12),
        guest_x13 = const VCpu::guest_gpr_offset(13),
        guest_x14 = const VCpu::guest_gpr_offset(14),
        guest_x15 = const VCpu::guest_gpr_offset(15),
        guest_x16 = const VCpu::guest_gpr_offset(16),
        guest_x17 = const VCpu::guest_gpr_offset(17),
        guest_x18 = const VCpu::guest_gpr_offset(18),
        guest_x19 = const VCpu::guest_gpr_offset(19),
        guest_x20 = const VCpu::guest_gpr_offset(20),
        guest_x21 = const VCpu::guest_gpr_offset(21),
        guest_x22 = const VCpu::guest_gpr_offset(22),
        guest_x23 = const VCpu::guest_gpr_offset(23),
        guest_x24 = const VCpu::guest_gpr_offset(24),
        guest_x25 = const VCpu::guest_gpr_offset(25),
        guest_x26 = const VCpu::guest_gpr_offset(26),
        guest_x27 = const VCpu::guest_gpr_offset(27),
        guest_x28 = const VCpu::guest_gpr_offset(28),
        guest_x29 = const VCpu::guest_gpr_offset(29),
        guest_x30 = const VCpu::guest_gpr_offset(30),
        guest_x31 = const VCpu::guest_gpr_offset(31),

        guest_sstatus = const vcpu_guest_csr_offset!(sstatus),
        guest_hstatus = const vcpu_guest_csr_offset!(hstatus),
        guest_scounteren = const vcpu_guest_csr_offset!(scounteren),
        guest_sepc = const vcpu_guest_csr_offset!(sepc),
        hyp_x1 = const VCpu::hyp_gpr_offset(1),
        hyp_x2 = const VCpu::hyp_gpr_offset(2),
        hyp_x3 = const VCpu::hyp_gpr_offset(3),
        hyp_x4 = const VCpu::hyp_gpr_offset(4),
        hyp_x8 = const VCpu::hyp_gpr_offset(8),
        hyp_x9 = const VCpu::hyp_gpr_offset(9),
        hyp_x10 = const VCpu::hyp_gpr_offset(10),
        hyp_x11 = const VCpu::hyp_gpr_offset(11),
        hyp_x12 = const VCpu::hyp_gpr_offset(12),
        hyp_x13 = const VCpu::hyp_gpr_offset(13),
        hyp_x14 = const VCpu::hyp_gpr_offset(14),
        hyp_x15 = const VCpu::hyp_gpr_offset(15),
        hyp_x16 = const VCpu::hyp_gpr_offset(16),
        hyp_x17 = const VCpu::hyp_gpr_offset(17),
        hyp_x18 = const VCpu::hyp_gpr_offset(18),
        hyp_x19 = const VCpu::hyp_gpr_offset(19),
        hyp_x20 = const VCpu::hyp_gpr_offset(20),
        hyp_x21 = const VCpu::hyp_gpr_offset(21),
        hyp_x22 = const VCpu::hyp_gpr_offset(22),
        hyp_x23 = const VCpu::hyp_gpr_offset(23),
        hyp_x24 = const VCpu::hyp_gpr_offset(24),
        hyp_x25 = const VCpu::hyp_gpr_offset(25),
        hyp_x26 = const VCpu::hyp_gpr_offset(26),
        hyp_x27 = const VCpu::hyp_gpr_offset(27),

        hyp_sstatus = const vcpu_hyp_csr_offset!(sstatus),
        hyp_scounteren = const vcpu_hyp_csr_offset!(scounteren),
        hyp_stvec = const vcpu_hyp_csr_offset!(stvec),
        hyp_sscratch = const vcpu_hyp_csr_offset!(sscratch),
    );
}
