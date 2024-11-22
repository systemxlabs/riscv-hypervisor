use crate::{
    allocator::frame::PHYS_FRAME_ALLOCATOR,
    config::PAGE_SIZE_4K,
    error::{HypervisorError, HypervisorResult},
    mem::addr::HostPhysAddr,
};
use alloc::vec;
use alloc::vec::Vec;
use spin::Mutex;

use super::{
    addr::HostVirtAddr,
    pte::{PTEFlags, PageTableEntry},
};

const SV39_TABLE_PTE_COUNT: usize = 512;

pub static HYPERVISOR_PAGE_TABLE: Mutex<PageTable> = Mutex::new(PageTable::empty());

pub fn init_page_table() {
    let page_table = PageTable::try_new().expect("Failed to create page table");
    *HYPERVISOR_PAGE_TABLE.lock() = page_table;
}

pub struct PageTable {
    root_paddr: HostPhysAddr,
    intrm_tables: Vec<HostPhysAddr>,
}

impl PageTable {
    pub const fn empty() -> Self {
        Self {
            root_paddr: HostPhysAddr::new(usize::MAX),
            intrm_tables: Vec::new(),
        }
    }

    pub fn try_new() -> HypervisorResult<Self> {
        let root_paddr = PHYS_FRAME_ALLOCATOR.lock().alloc_frames(1)?;
        // mmu should be disabled now
        unsafe { core::ptr::write_bytes(root_paddr.as_usize() as *mut u8, 0, PAGE_SIZE_4K) };
        Ok(Self {
            root_paddr,
            intrm_tables: vec![root_paddr],
        })
    }

    pub fn root_paddr(&self) -> HostPhysAddr {
        self.root_paddr
    }

    pub fn map(
        &mut self,
        vaddr: HostVirtAddr,
        paddr: HostPhysAddr,
        flags: PTEFlags,
    ) -> HypervisorResult<()> {
        assert!(vaddr.is_aligned(PAGE_SIZE_4K));
        assert!(paddr.is_aligned(PAGE_SIZE_4K));
        let pte = self.get_entry_mut_or_create(vaddr)?;
        if pte.is_unused() {
            *pte = PageTableEntry::new(paddr, flags);
            Ok(())
        } else {
            Err(HypervisorError::AlreadyMapped)
        }
    }

    pub fn map_region(
        &mut self,
        vaddr: HostVirtAddr,
        paddr: HostPhysAddr,
        num_pages: usize,
        flags: PTEFlags,
    ) -> HypervisorResult<()> {
        assert!(vaddr.is_aligned(PAGE_SIZE_4K));
        assert!(paddr.is_aligned(PAGE_SIZE_4K));
        for i in 0..num_pages {
            self.map(vaddr + i * PAGE_SIZE_4K, paddr + i * PAGE_SIZE_4K, flags)?;
        }
        Ok(())
    }

    pub fn query(&self, vaddr: HostVirtAddr) -> HypervisorResult<(HostPhysAddr, PTEFlags)> {
        let pte = self.get_entry_mut(vaddr)?;
        Ok((pte.paddr(), pte.flags()))
    }

    fn table_of<'a>(&self, paddr: HostPhysAddr) -> &'a [PageTableEntry] {
        let ptr = paddr.as_usize() as _;
        // as we did identical mapping, so vaddr = paddr
        unsafe { core::slice::from_raw_parts(ptr, SV39_TABLE_PTE_COUNT) }
    }

    fn table_of_mut<'a>(&self, paddr: HostPhysAddr) -> &'a mut [PageTableEntry] {
        let ptr = paddr.as_usize() as _;
        // as we did identical mapping, so vaddr = paddr
        unsafe { core::slice::from_raw_parts_mut(ptr, SV39_TABLE_PTE_COUNT) }
    }

    fn next_table_mut<'a>(
        &self,
        entry: &PageTableEntry,
    ) -> HypervisorResult<&'a mut [PageTableEntry]> {
        if entry.is_valid() {
            Ok(self.table_of_mut(entry.paddr()))
        } else {
            Err(HypervisorError::NotMapped)
        }
    }

    fn next_table_mut_or_create<'a>(
        &mut self,
        entry: &mut PageTableEntry,
    ) -> HypervisorResult<&'a mut [PageTableEntry]> {
        if entry.is_unused() {
            let paddr = PHYS_FRAME_ALLOCATOR.lock().alloc_frames(1)?;
            self.intrm_tables.push(paddr);
            *entry = PageTableEntry::new(paddr, PTEFlags::V);
            Ok(self.table_of_mut(paddr))
        } else {
            self.next_table_mut(entry)
        }
    }

    fn get_entry_mut_or_create(
        &mut self,
        vaddr: HostVirtAddr,
    ) -> HypervisorResult<&mut PageTableEntry> {
        let table1 = self.table_of_mut(self.root_paddr);
        let table1_pte_index = (vaddr.as_usize() >> (12 + 18)) & (SV39_TABLE_PTE_COUNT - 1);
        let table1_pte = &mut table1[table1_pte_index];

        let table2 = self.next_table_mut_or_create(table1_pte)?;
        let table2_pte_index = (vaddr.as_usize() >> (12 + 9)) & (SV39_TABLE_PTE_COUNT - 1);
        let table2_pte = &mut table2[table2_pte_index];

        let table3 = self.next_table_mut_or_create(table2_pte)?;
        let table3_pte_index = (vaddr.as_usize() >> 12) & (SV39_TABLE_PTE_COUNT - 1);
        let table3_pte = &mut table3[table3_pte_index];

        Ok(table3_pte)
    }

    fn get_entry_mut(&self, vaddr: HostVirtAddr) -> HypervisorResult<&mut PageTableEntry> {
        let table1 = self.table_of_mut(self.root_paddr);
        let table1_pte_index = (vaddr.as_usize() >> (12 + 18)) & (SV39_TABLE_PTE_COUNT - 1);
        let table1_pte = &mut table1[table1_pte_index];

        let table2 = self.next_table_mut(table1_pte)?;
        let table2_pte_index = (vaddr.as_usize() >> (12 + 9)) & (SV39_TABLE_PTE_COUNT - 1);
        let table2_pte = &mut table2[table2_pte_index];

        let table3 = self.next_table_mut(table2_pte)?;
        let table3_pte_index = (vaddr.as_usize() >> 12) & (SV39_TABLE_PTE_COUNT - 1);
        let table3_pte = &mut table3[table3_pte_index];

        Ok(table3_pte)
    }
}

impl Drop for PageTable {
    fn drop(&mut self) {
        for paddr in self.intrm_tables.iter() {
            PHYS_FRAME_ALLOCATOR.lock().dealloc_frames(*paddr, 1);
        }
    }
}
