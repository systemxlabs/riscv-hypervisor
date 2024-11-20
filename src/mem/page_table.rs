use crate::{allocator::frame::PHYS_FRAME_ALLOCATOR, config::PAGE_SIZE_4K, error::HypervisorResult, mem::addr::PhysAddr};

use super::{addr::VirtAddr, pte::{PTEFlags, PageTableEntry}};

const SV39_TABLE_PTE_COUNT: usize = 512;

pub struct PageTable {
    root_paddr: PhysAddr,
}

impl PageTable {
    pub fn try_new() -> HypervisorResult<Self> {
        let root_paddr = PHYS_FRAME_ALLOCATOR.lock().alloc_frames(1)?;
        // mmu should be disabled now
        unsafe { core::ptr::write_bytes(root_paddr.as_usize() as *mut u8, 0, PAGE_SIZE_4K) };
        Ok(Self { root_paddr })
    }

    pub fn map(&self, vaddr: VirtAddr, paddr: VirtAddr, flags: PTEFlags) {
        todo!()
    }
}