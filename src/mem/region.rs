use crate::config::{PAGE_SIZE_4K, PHYS_MEMORY_END};
use crate::mem::addr::{align_down, align_up, PhysAddr};
use crate::mem::page_table::HYPERVISOR_PAGE_TABLE;
use crate::mem::pte::PTEFlags;
use alloc::vec::Vec;
use log::info;

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
        fn ehypervisor();
    }
    let start = PhysAddr::from(ehypervisor as usize).align_up(PAGE_SIZE_4K);
    let end = PhysAddr::from(PHYS_MEMORY_END).align_down(PAGE_SIZE_4K);

    MemRegion {
        paddr: start,
        size: end.as_usize() - start.as_usize(),
        flags: MemRegionFlags::FREE | MemRegionFlags::READ | MemRegionFlags::WRITE,
        name: "free memory",
    }
}

pub fn map_hypervisor_image() {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss_with_stack();
        fn ebss();
    }
    let stext = stext as usize;
    let etext = etext as usize;
    assert_eq!(stext % PAGE_SIZE_4K, 0);
    assert_eq!(etext % PAGE_SIZE_4K, 0);
    let pte_flags = PTEFlags::V | PTEFlags::R | PTEFlags::X;
    info!(
        "map region .text: [{:#x}, {:#x}) -> [{:#x}, {:#x}) {:?}",
        stext, etext, stext, etext, pte_flags
    );
    HYPERVISOR_PAGE_TABLE.lock().map_region(
        stext.into(),
        stext.into(),
        (etext - stext) / PAGE_SIZE_4K,
        pte_flags,
    );

    let srodata = srodata as usize;
    let erodata = erodata as usize;
    assert_eq!(srodata % PAGE_SIZE_4K, 0);
    assert_eq!(erodata % PAGE_SIZE_4K, 0);
    let pte_flags = PTEFlags::V | PTEFlags::R;
    info!(
        "map region .rodata: [{:#x}, {:#x}) -> [{:#x}, {:#x}) {:?}",
        srodata, erodata, srodata, erodata, pte_flags
    );
    HYPERVISOR_PAGE_TABLE.lock().map_region(
        srodata.into(),
        srodata.into(),
        (erodata - srodata) / PAGE_SIZE_4K,
        pte_flags,
    );

    let sdata = sdata as usize;
    let edata = edata as usize;
    assert_eq!(sdata % PAGE_SIZE_4K, 0);
    assert_eq!(edata % PAGE_SIZE_4K, 0);
    let pte_flags = PTEFlags::V | PTEFlags::R | PTEFlags::W;
    info!(
        "map region .data: [{:#x}, {:#x}) -> [{:#x}, {:#x}) {:?}",
        sdata, edata, sdata, edata, pte_flags
    );
    HYPERVISOR_PAGE_TABLE.lock().map_region(
        sdata.into(),
        sdata.into(),
        (edata - sdata) / PAGE_SIZE_4K,
        pte_flags,
    );

    let sbss = sbss_with_stack as usize;
    let ebss = ebss as usize;
    assert_eq!(sbss % PAGE_SIZE_4K, 0);
    assert_eq!(ebss % PAGE_SIZE_4K, 0);
    let pte_flags = PTEFlags::V | PTEFlags::R | PTEFlags::W;
    info!(
        "map region .bss: [{:#x}, {:#x}) -> [{:#x}, {:#x}) {:?}",
        sbss, ebss, sbss, ebss, pte_flags
    );
    HYPERVISOR_PAGE_TABLE.lock().map_region(
        sbss.into(),
        sbss.into(),
        (ebss - sbss) / PAGE_SIZE_4K,
        pte_flags,
    );
}

pub fn map_free_memory() {
    extern "C" {
        fn ehypervisor();
    }
    let free_mem_start = align_up(ehypervisor as usize, PAGE_SIZE_4K);
    let free_mem_end = align_down(PHYS_MEMORY_END, PAGE_SIZE_4K);
    assert_eq!(free_mem_start % PAGE_SIZE_4K, 0);
    assert_eq!(free_mem_end % PAGE_SIZE_4K, 0);
    let pte_flags = PTEFlags::V | PTEFlags::R | PTEFlags::W;
    info!(
        "map region free memory: [{:#x}, {:#x}) -> [{:#x}, {:#x}) {:?}",
        free_mem_start, free_mem_end, free_mem_start, free_mem_end, pte_flags
    );
    HYPERVISOR_PAGE_TABLE.lock().map_region(
        free_mem_start.into(),
        free_mem_start.into(),
        (free_mem_end - free_mem_start) / PAGE_SIZE_4K,
        pte_flags,
    );
}

pub fn map_heap_memory() {
    extern "C" {
        fn ehypervisor();
    }
    let free_mem_start = align_up(ehypervisor as usize, PAGE_SIZE_4K);
    let free_mem_end = align_down(PHYS_MEMORY_END, PAGE_SIZE_4K);
    assert_eq!(free_mem_start % PAGE_SIZE_4K, 0);
    assert_eq!(free_mem_end % PAGE_SIZE_4K, 0);
    let pte_flags = PTEFlags::V | PTEFlags::R | PTEFlags::W;

    let heap_start = align_up(PHYS_MEMORY_END, PAGE_SIZE_4K);
    let heap_end = heap_start + (free_mem_end - free_mem_start);
    assert_eq!(heap_start % PAGE_SIZE_4K, 0);
    assert_eq!(heap_end % PAGE_SIZE_4K, 0);

    assert_eq!(free_mem_end - free_mem_start, heap_end - heap_start);

    info!(
        "map region heap: [{:#x}, {:#x}) -> [{:#x}, {:#x}) {:?}",
        free_mem_start, free_mem_end, heap_start, heap_end, pte_flags
    );
}
