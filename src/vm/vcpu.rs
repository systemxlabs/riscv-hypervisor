#[repr(C)]
pub struct VCpu {
    vcpu_id: usize,
    pcpu_regs: PCpuRegs,
    vcpu_regs: VCpuRegs,
}

#[repr(C)]
pub struct VCpuRegs {
    grs: GeneralRegs,
    sstatus: usize,
    scounteren: usize,
    sepc: usize,
}

#[derive(Default)]
#[repr(C)]
pub struct GeneralRegs([usize; 32]);

#[repr(C)]
pub struct PCpuRegs {
    grs: GeneralRegs,
    sstatus: usize,
    scounteren: usize,
    sepc: usize,
}
