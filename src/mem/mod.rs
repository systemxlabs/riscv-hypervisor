use alloc::vec::Vec;
use crate::config::{PAGE_SIZE_4K, PHYS_MEMORY_END};
use crate::mem::addr::PhysAddr;

mod addr;

bitflags::bitflags! {
    /// The flags of a physical memory region.
    pub struct MemRegionFlags: usize {
        /// Readable.
        const READ          = 1 << 0;
        /// Writable.
        const WRITE         = 1 << 1;
        /// Executable.
        const EXECUTE       = 1 << 2;
        /// Device memory.
        const DEVICE        = 1 << 4;
        /// Reserved memory, do not use for allocation.
        const RESERVED      = 1 << 5;
        /// Free memory for allocation.
        const FREE          = 1 << 6;
    }
}

/// A physical memory region.
#[derive(Debug)]
pub struct MemRegion {
    /// The start physical address of the region.
    pub paddr: PhysAddr,
    /// The size in bytes of the region.
    pub size: usize,
    /// The region flags, see [`MemRegionFlags`].
    pub flags: MemRegionFlags,
    /// The region name, used for identification.
    pub name: &'static str,
}

pub fn all_mem_regions() -> Vec<MemRegion> {
    todo!()
}

pub fn free_mem_region() -> MemRegion {
    extern "C" {
        fn ekernel();
    }
    let start = PhysAddr::from(ekernel as usize).align_up(PAGE_SIZE_4K);
    let end = PhysAddr::from(PHYS_MEMORY_END).align_down(PAGE_SIZE_4K);

    MemRegion {
        paddr: start,
        size: end.as_usize() - start.as_usize(),
        flags: MemRegionFlags::FREE | MemRegionFlags::READ | MemRegionFlags::WRITE,
        name: "free memory",
    }
}