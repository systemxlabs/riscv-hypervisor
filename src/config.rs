pub const PHYS_MEMORY_BASE: usize = 0x8000_0000;
pub const PHYS_MEMORY_SIZE: usize = 0x4000_0000; // 1G
pub const PHYS_MEMORY_END: usize = PHYS_MEMORY_BASE + PHYS_MEMORY_SIZE;

pub const PAGE_SIZE_4K: usize = 0x1000;
pub const BOOT_STACK_SIZE: usize = 1000 * PAGE_SIZE_4K;

pub const PCPU_STACK_SIZE: usize = 4 * PAGE_SIZE_4K;
