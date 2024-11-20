use crate::{
    allocator::frame::PHYS_FRAME_ALLOCATOR, config::PAGE_SIZE_4K, error::HypervisorResult,
    mem::addr::PhysAddr,
};
use spin::Mutex;

use super::{
    addr::VirtAddr,
    pte::{PTEFlags, PageTableEntry},
};

const SV39_TABLE_PTE_COUNT: usize = 512;

pub static HYPERVISOR_PAGE_TABLE: Mutex<PageTable> = Mutex::new(PageTable::empty());

pub fn init_page_table() {
    let page_table = PageTable::try_new().expect("Failed to create page table");
    *HYPERVISOR_PAGE_TABLE.lock() = page_table;
}

pub struct PageTable {
    root_paddr: PhysAddr,
}

impl PageTable {
    pub const fn empty() -> Self {
        Self {
            root_paddr: PhysAddr::new(usize::MAX),
        }
    }

    pub fn try_new() -> HypervisorResult<Self> {
        let root_paddr = PHYS_FRAME_ALLOCATOR.lock().alloc_frames(1)?;
        // mmu should be disabled now
        unsafe { core::ptr::write_bytes(root_paddr.as_usize() as *mut u8, 0, PAGE_SIZE_4K) };
        Ok(Self { root_paddr })
    }

    pub fn map(&self, vaddr: VirtAddr, paddr: PhysAddr, flags: PTEFlags) {
        assert!(vaddr.is_aligned(PAGE_SIZE_4K));
        assert!(paddr.is_aligned(PAGE_SIZE_4K));
    }

    pub fn map_region(&self, vaddr: VirtAddr, paddr: PhysAddr, num_pages: usize, flags: PTEFlags) {
        assert!(vaddr.is_aligned(PAGE_SIZE_4K));
        assert!(paddr.is_aligned(PAGE_SIZE_4K));
        for i in 0..num_pages {
            self.map(vaddr + i * PAGE_SIZE_4K, paddr + i * PAGE_SIZE_4K, flags);
        }
    }
}
