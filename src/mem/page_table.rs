use core::sync::atomic::AtomicBool;

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
    map_free_memory, map_hypervisor_image,
    pte::{PTEFlags, PageTableEntry},
};

const SV39_TABLE_PTE_COUNT: usize = 512;

pub static HYPERVISOR_PAGE_TABLE: Mutex<PageTable> = Mutex::new(PageTable::empty());
pub static HYPERVISOR_PAGE_TABLE_INITED: AtomicBool = AtomicBool::new(false);

pub fn init_hypervisor_page_table() {
    let page_table = PageTable::try_new().expect("Failed to create page table");
    *HYPERVISOR_PAGE_TABLE.lock() = page_table;
    map_hypervisor_image();
    map_free_memory();
    HYPERVISOR_PAGE_TABLE_INITED.store(true, core::sync::atomic::Ordering::SeqCst);
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
        let pte = self.get_entry_mut(vaddr, true)?;
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

    pub fn query_page(&mut self, vpn: HostVirtAddr) -> HypervisorResult<(HostPhysAddr, PTEFlags)> {
        assert_eq!(vpn.as_usize() & (PAGE_SIZE_4K - 1), 0);
        let pte = self.get_entry_mut(vpn, false)?;
        Ok((pte.ppn(), pte.flags()))
    }

    pub fn translate(&mut self, vaddr: HostVirtAddr) -> HypervisorResult<HostPhysAddr> {
        let pte = self.get_entry_mut(vaddr, false)?;
        if pte.is_valid() {
            let offset = vaddr.as_usize() & (PAGE_SIZE_4K - 1);
            let paddr = pte.ppn().as_usize() + offset;
            Ok(paddr.into())
        } else {
            Err(HypervisorError::NotMapped)
        }
    }

    fn table_of_mut<'a>(&self, paddr: HostPhysAddr) -> &'a mut [PageTableEntry] {
        let ptr = paddr.as_usize() as _;
        // as we did identical mapping, so vaddr = paddr
        unsafe { core::slice::from_raw_parts_mut(ptr, SV39_TABLE_PTE_COUNT) }
    }

    fn next_table_mut<'a>(
        &mut self,
        entry: &mut PageTableEntry,
        create_if_absent: bool,
    ) -> HypervisorResult<&'a mut [PageTableEntry]> {
        if entry.is_unused() && create_if_absent {
            let paddr = PHYS_FRAME_ALLOCATOR.lock().alloc_frames(1)?;
            self.intrm_tables.push(paddr);
            *entry = PageTableEntry::new(paddr, PTEFlags::V);
        }
        if entry.is_valid() {
            Ok(self.table_of_mut(entry.ppn()))
        } else {
            Err(HypervisorError::NotMapped)
        }
    }

    fn get_entry_mut(
        &mut self,
        vaddr: HostVirtAddr,
        create_if_absent: bool,
    ) -> HypervisorResult<&mut PageTableEntry> {
        let table1 = self.table_of_mut(self.root_paddr);
        let table1_pte_index = (vaddr.as_usize() >> (12 + 18)) & (SV39_TABLE_PTE_COUNT - 1);
        let table1_pte = &mut table1[table1_pte_index];

        let table2 = self.next_table_mut(table1_pte, create_if_absent)?;
        let table2_pte_index = (vaddr.as_usize() >> (12 + 9)) & (SV39_TABLE_PTE_COUNT - 1);
        let table2_pte = &mut table2[table2_pte_index];

        let table3 = self.next_table_mut(table2_pte, create_if_absent)?;
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
