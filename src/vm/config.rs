use crate::vm::config;
use alloc::{string::String, vec::Vec};
use log::{debug, info};
use serde_derive::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct VMConfig {
    pub name: String,
    pub kernel: String,
    pub memory_limit: usize,
    pub num_vcpu: usize,
}

pub fn vm_configs() -> Vec<VMConfig> {
    let vm_configs = serde_json::from_str(include_str!("../../vm_configs.json")).unwrap();
    info!("[Hypervisor] Parsed VM configs: {:?}", vm_configs);

    debug!(
        "Guest rCore-tutorial-v3 bin: [{:#x}, {:#x}) {}bytes",
        GUEST_RCORE_TUTORIAL_V3_BIN.as_ptr() as usize,
        GUEST_RCORE_TUTORIAL_V3_BIN.as_ptr() as usize + GUEST_RCORE_TUTORIAL_V3_BIN.len(),
        GUEST_RCORE_TUTORIAL_V3_BIN.len()
    );
    vm_configs
}

static GUEST_RCORE_TUTORIAL_V3_BIN: [u8; include_bytes!("../../guests/rCore-Tutorial-v3/os.bin")
    .len()] = *include_bytes!("../../guests/rCore-Tutorial-v3/os.bin");

static GUEST_HELLO_WORLD_BIN: [u8; include_bytes!("../../guests/hello-world/hello-world.bin")
    .len()] = *include_bytes!("../../guests/hello-world/hello-world.bin");

pub fn kernel_image(kernel: &str) -> &'static [u8] {
    if kernel.eq_ignore_ascii_case("rCore-Tutorial-v3") {
        return GUEST_RCORE_TUTORIAL_V3_BIN.as_ref();
    }
    if kernel.eq_ignore_ascii_case("hello-world") {
        return GUEST_HELLO_WORLD_BIN.as_ref();
    }
    panic!("Unsupported kernel: {}", kernel)
}
