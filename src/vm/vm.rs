use core::sync::atomic::AtomicBool;

use crate::error::HypervisorResult;
use alloc::vec::Vec;
use spin::lock_api::Mutex;

use crate::mem::PageTable;
use crate::vm::{config, kernel_image, VCpu, VMConfig};

pub static GLOBAL_VMS: Mutex<Vec<VM>> = Mutex::new(Vec::new());

pub fn init_vms() {
    let vm_configs = config::vm_configs();
    for vm_config in vm_configs {
        let vm = VM::new(vm_config).expect("Failed to create VM");
        GLOBAL_VMS.lock().push(vm);
    }
}

pub struct VM {
    running: AtomicBool,
    vcpus: Vec<VCpu>,
    page_table: PageTable,
    kernel_image: &'static [u8],
    memory_limit: usize,
}

impl VM {
    pub fn new(vm_config: VMConfig) -> HypervisorResult<Self> {
        let kernel_image = kernel_image(vm_config.kernel.as_str());
        let page_table = PageTable::try_new()?;
        Ok(Self {
            running: AtomicBool::new(false),
            vcpus: Vec::new(),
            page_table,
            kernel_image,
            memory_limit: vm_config.memory,
        })
    }
}
