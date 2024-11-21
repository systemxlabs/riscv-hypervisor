use alloc::{string::String, vec::Vec};
use log::debug;
use serde_derive::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct VMConfig {
    pub id: usize,
    pub name: String,
}

pub fn vm_configs() -> Vec<VMConfig> {
    let vm_configs = serde_json::from_str(include_str!("../../vm_configs.json")).unwrap();
    debug!("vm_configs: {:?}", vm_configs);
    vm_configs
}
