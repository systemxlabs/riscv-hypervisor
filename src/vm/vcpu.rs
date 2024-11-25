use core::sync::atomic::AtomicUsize;

static VCPU_ID_GENERATOR: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
#[repr(C)]
pub struct VCpu {
    pub vcpu_id: usize,
    vcpu_regs: VCpuRegs,
}

impl VCpu {
    pub fn new() -> Self {
        Self {
            vcpu_id: VCPU_ID_GENERATOR.fetch_add(1, core::sync::atomic::Ordering::SeqCst),
            vcpu_regs: VCpuRegs::default(),
        }
    }
}

#[derive(Default, Debug)]
#[repr(C)]
pub struct VCpuRegs {
    grs: GeneralRegs,
    sstatus: usize,
    scounteren: usize,
    sepc: usize,
}

#[derive(Default, Debug)]
#[repr(C)]
pub struct GeneralRegs([usize; 32]);

#[derive(Default, Debug)]
#[repr(C)]
pub struct PCpuRegs {
    grs: GeneralRegs,
    sstatus: usize,
    scounteren: usize,
    sepc: usize,
}
