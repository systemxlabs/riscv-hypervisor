use alloc::vec::Vec;
use log::{debug, info};
use serde_derive::Deserialize;

#[derive(Debug, Clone)]
pub struct VMConfig {
    pub name: &'static str,
    pub kernel: &'static str,
    pub memory_limit: usize,
    pub num_vcpu: usize,
    pub entry: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VMJsonConfig {
    pub name: &'static str,
    pub kernel: &'static str,
    pub memory_limit: &'static str,
    pub num_vcpu: usize,
    pub entry: &'static str,
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

static GUEST_HELLO_WORLD_BIN: [u8; include_bytes!("../../guests/hello-world/hello-world.bin")
    .len()] = *include_bytes!("../../guests/hello-world/hello-world.bin");

static GUEST_RCORE_TUTORIAL_V3_CH1_BIN: [u8; include_bytes!(
    "../../guests/rCore-Tutorial-v3/os-ch1.bin"
)
.len()] = *include_bytes!("../../guests/rCore-Tutorial-v3/os-ch1.bin");
static GUEST_RCORE_TUTORIAL_V3_CH2_BIN: [u8; include_bytes!(
    "../../guests/rCore-Tutorial-v3/os-ch2.bin"
)
.len()] = *include_bytes!("../../guests/rCore-Tutorial-v3/os-ch2.bin");
static GUEST_RCORE_TUTORIAL_V3_CH3_BIN: [u8; include_bytes!(
    "../../guests/rCore-Tutorial-v3/os-ch3.bin"
)
.len()] = *include_bytes!("../../guests/rCore-Tutorial-v3/os-ch3.bin");
static GUEST_RCORE_TUTORIAL_V3_CH4_BIN: [u8; include_bytes!(
    "../../guests/rCore-Tutorial-v3/os-ch4.bin"
)
.len()] = *include_bytes!("../../guests/rCore-Tutorial-v3/os-ch4.bin");
static GUEST_RCORE_TUTORIAL_V3_CH5_BIN: [u8; include_bytes!(
    "../../guests/rCore-Tutorial-v3/os-ch5.bin"
)
.len()] = *include_bytes!("../../guests/rCore-Tutorial-v3/os-ch5.bin");
static GUEST_RCORE_TUTORIAL_V3_CH6_BIN: [u8; include_bytes!(
    "../../guests/rCore-Tutorial-v3/os-ch6.bin"
)
.len()] = *include_bytes!("../../guests/rCore-Tutorial-v3/os-ch6.bin");
static GUEST_RCORE_TUTORIAL_V3_CH7_BIN: [u8; include_bytes!(
    "../../guests/rCore-Tutorial-v3/os-ch7.bin"
)
.len()] = *include_bytes!("../../guests/rCore-Tutorial-v3/os-ch7.bin");

pub fn kernel_image(kernel: &str) -> &'static [u8] {
    if kernel.eq_ignore_ascii_case("hello-world") {
        return GUEST_HELLO_WORLD_BIN.as_ref();
    }
    if kernel.eq_ignore_ascii_case("rCore-Tutorial-v3-ch1") {
        return GUEST_RCORE_TUTORIAL_V3_CH1_BIN.as_ref();
    }
    if kernel.eq_ignore_ascii_case("rCore-Tutorial-v3-ch2") {
        return GUEST_RCORE_TUTORIAL_V3_CH2_BIN.as_ref();
    }
    if kernel.eq_ignore_ascii_case("rCore-Tutorial-v3-ch3") {
        return GUEST_RCORE_TUTORIAL_V3_CH3_BIN.as_ref();
    }
    if kernel.eq_ignore_ascii_case("rCore-Tutorial-v3-ch4") {
        return GUEST_RCORE_TUTORIAL_V3_CH4_BIN.as_ref();
    }
    if kernel.eq_ignore_ascii_case("rCore-Tutorial-v3-ch5") {
        return GUEST_RCORE_TUTORIAL_V3_CH5_BIN.as_ref();
    }
    if kernel.eq_ignore_ascii_case("rCore-Tutorial-v3-ch6") {
        return GUEST_RCORE_TUTORIAL_V3_CH6_BIN.as_ref();
    }
    if kernel.eq_ignore_ascii_case("rCore-Tutorial-v3-ch7") {
        return GUEST_RCORE_TUTORIAL_V3_CH7_BIN.as_ref();
    }
    panic!("Unsupported kernel: {}", kernel)
}
