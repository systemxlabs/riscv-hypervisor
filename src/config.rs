pub const PHYS_MEMORY_BASE: usize = 0x8000_0000;
pub const PHYS_MEMORY_SIZE: usize = 0x800_0000;  // 128 MB
pub const PHYS_MEMORY_END: usize = PHYS_MEMORY_BASE + PHYS_MEMORY_SIZE;

pub const BOOT_STACK_SIZE: usize = 16 * 1000;
