use core::{mem::offset_of, sync::atomic::AtomicUsize};

static VCPU_ID_GENERATOR: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
#[repr(C)]
pub struct VCpu {
    pub vcpu_id: usize,
    pub hyp_cpu_state: HypervisorCpuState,
    pub guest_cpu_state: GuestCpuState,
}

impl VCpu {
    pub fn new() -> Self {
        Self {
            vcpu_id: VCPU_ID_GENERATOR.fetch_add(1, core::sync::atomic::Ordering::SeqCst),
            hyp_cpu_state: HypervisorCpuState::default(),
            guest_cpu_state: GuestCpuState::default(),
        }
    }

    pub const fn hyp_gpr_offset(index: usize) -> usize {
        assert!(index < 32);
        offset_of!(VCpu, hyp_cpu_state) + offset_of!(HypervisorCpuState, gprs) + index * 8
    }

    pub const fn guest_gpr_offset(index: usize) -> usize {
        assert!(index < 32);
        offset_of!(VCpu, guest_cpu_state) + offset_of!(GuestCpuState, gprs) + index * 8
    }
}

#[macro_export]
macro_rules! vcpu_hyp_csr_offset {
    ($reg:tt) => {
        core::mem::offset_of!(VCpu, hyp_cpu_state)
            + core::mem::offset_of!(crate::vm::HypervisorCpuState, $reg)
    };
}
#[macro_export]
macro_rules! vcpu_guest_csr_offset {
    ($reg:tt) => {
        core::mem::offset_of!(VCpu, guest_cpu_state)
            + core::mem::offset_of!(crate::vm::GuestCpuState, $reg)
    };
}

#[derive(Default, Debug)]
#[repr(C)]
pub struct GeneralPurposeRegs([usize; 32]);

#[derive(Default, Debug)]
#[repr(C)]
pub struct GuestCpuState {
    pub gprs: GeneralPurposeRegs,
    pub sstatus: usize,
    pub hstatus: usize,
    pub scounteren: usize,
    pub sepc: usize,
}

#[derive(Default, Debug)]
#[repr(C)]
pub struct HypervisorCpuState {
    pub gprs: GeneralPurposeRegs,
    pub sstatus: usize,
    pub scounteren: usize,
    pub stvec: usize,
    pub sscratch: usize,
}
