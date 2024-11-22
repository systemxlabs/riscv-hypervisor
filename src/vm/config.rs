use alloc::{string::String, vec::Vec};
use log::debug;
use serde_derive::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct VMConfig {
    pub id: usize,
    pub name: String,
    pub kernel: String,
}

pub fn vm_configs() -> Vec<VMConfig> {
    let vm_configs = serde_json::from_str(include_str!("../../vm_configs.json")).unwrap();
    debug!("vm_configs: {:?}", vm_configs);

    debug!(
        "Guest rCore-tutorial-v3 bin: [{:#x}, {}) {}bytes",
        GUEST_RCORE_TUTORIAL_V3_BIN.as_ptr() as usize,
        GUEST_RCORE_TUTORIAL_V3_BIN.as_ptr() as usize + GUEST_RCORE_TUTORIAL_V3_BIN.len(),
        GUEST_RCORE_TUTORIAL_V3_BIN.len()
    );
    vm_configs
}

static GUEST_RCORE_TUTORIAL_V3_BIN: [u8; include_bytes!("../../guests/rCore-Tutorial-v3/os.bin")
    .len()] = *include_bytes!("../../guests/rCore-Tutorial-v3/os.bin");
