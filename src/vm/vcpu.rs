pub struct VCpu {
    vcpu_id: usize,
    state: VCpuState,
    regs: VCpuRegisters,
}

/// The state of a virtual CPU.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCpuState {
    /// An invalid state.
    Invalid = 0,
    /// The vcpu is created but not initialized yet.
    Created = 1,
    /// The vcpu is already initialized and can be bound to a physical CPU.
    Free = 2,
    /// The vcpu is bound to a physical CPU and ready to run.
    Ready = 3,
    /// The vcpu is bound to a physical CPU and running.
    Running = 4,
    /// The vcpu is blocked.
    Blocked = 5,
}

pub struct VCpuRegisters {}

#[derive(Default)]
#[repr(C)]
pub struct GeneralPurposeRegisters([usize; 32]);

/// Guest GPR and CSR state which must be saved/restored when exiting/entering virtualization.
#[derive(Default)]
#[repr(C)]
struct GuestCpuState {
    gprs: GeneralPurposeRegisters,
    sstatus: usize,
    hstatus: usize,
    scounteren: usize,
    sepc: usize,
}
