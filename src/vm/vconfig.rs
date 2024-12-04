use alloc::{string::String, vec::Vec};
use log::{debug, info};
use serde_derive::Deserialize;

#[derive(Debug, Clone)]
pub struct VMConfig {
    pub name: String,
    pub kernel: String,
    pub memory_limit: usize,
    pub num_vcpu: usize,
    pub entry: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VMJsonConfig {
    pub name: String,
    pub kernel: String,
    pub memory_limit: String,
    pub num_vcpu: usize,
    pub entry: String,
}

pub fn vm_configs() -> Vec<VMConfig> {
    let vm_json_configs: Vec<VMJsonConfig> =
        serde_json::from_str(include_str!("../../vm_configs.json")).unwrap();
    let mut vm_configs = Vec::new();
    for vm_json_config in vm_json_configs {
        let entry_str = vm_json_config
            .entry
            .trim_start_matches("0x")
            .trim_start_matches("0X")
            .replace("_", "");
        let entry = usize::from_str_radix(&entry_str, 16).unwrap();

        let memory_limit = parse_memory_limit(&vm_json_config.memory_limit);

        vm_configs.push(VMConfig {
            name: vm_json_config.name,
            kernel: vm_json_config.kernel,
            memory_limit,
            num_vcpu: vm_json_config.num_vcpu,
            entry,
        });
    }
    info!("[Hypervisor] Parsed VM configs: {:#x?}", vm_configs);

    debug!(
        "Guest rCore-tutorial-v3 bin: [{:#x}, {:#x}) {}bytes",
        GUEST_RCORE_TUTORIAL_V3_BIN.as_ptr() as usize,
        GUEST_RCORE_TUTORIAL_V3_BIN.as_ptr() as usize + GUEST_RCORE_TUTORIAL_V3_BIN.len(),
        GUEST_RCORE_TUTORIAL_V3_BIN.len()
    );
    vm_configs
}

fn parse_memory_limit(size_str: &str) -> usize {
    let clean_str = size_str.trim().to_uppercase();

    if clean_str.ends_with("M") {
        clean_str.trim_end_matches("M").parse::<usize>().unwrap() * 1024 * 1024
    } else if clean_str.ends_with("G") {
        clean_str.trim_end_matches("G").parse::<usize>().unwrap() * 1024 * 1024 * 1024
    } else {
        panic!("Unsupported memory limit format: {}", size_str);
    }
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
