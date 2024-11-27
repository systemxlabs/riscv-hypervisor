pub const PHYS_MEMORY_BASE: usize = 0x8000_0000;
pub const PHYS_MEMORY_SIZE: usize = 0x4000_0000; // 1G
pub const PHYS_MEMORY_END: usize = PHYS_MEMORY_BASE + PHYS_MEMORY_SIZE;

pub const PAGE_SIZE_4K: usize = 0x1000;
pub const BOOT_STACK_SIZE: usize = 16 * PAGE_SIZE_4K;

pub const PCPU_STACK_SIZE: usize = 4 * PAGE_SIZE_4K;

/// MMIO regions with format (`base_paddr`, `size`).
pub const MMIO_REGIONS: &[(usize, usize)] = &[
    (0x0c00_0000, 0x21_0000),
    (0x1000_0000, 0x1000),
    (0x1000_1000, 0x8000),
    (0x3000_0000, 0x1000_0000),
    (0x4000_0000, 0x4000_0000),
];
