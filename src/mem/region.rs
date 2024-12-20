use crate::config::PAGE_SIZE_4K;
use crate::dtb::MachineMeta;
use crate::mem::addr::{align_down, align_up};
use crate::mem::page_table::HYPERVISOR_PAGE_TABLE;
use crate::mem::pte::PTEFlags;
use log::{debug, info};

pub fn map_mmio_regions(meta: &MachineMeta) {
    for virt_dev in meta.virtio.iter() {
        let pte_flags = PTEFlags::V | PTEFlags::R | PTEFlags::W;
        info!(
            "[Hypervisor] map region mmio: [{:#x}, {:#x}) -> [{:#x}, {:#x}) {:?}",
            virt_dev.base_address,
            virt_dev.base_address + virt_dev.size,
            virt_dev.base_address,
            virt_dev.base_address + virt_dev.size,
            pte_flags
        );
        assert_eq!(virt_dev.base_address % PAGE_SIZE_4K, 0);
        assert_eq!(virt_dev.size % PAGE_SIZE_4K, 0);
        HYPERVISOR_PAGE_TABLE
            .lock()
            .map_region(
                virt_dev.base_address.into(),
                virt_dev.base_address.into(),
                virt_dev.size / PAGE_SIZE_4K,
                pte_flags,
            )
            .expect("should work fine");
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
        "[Hypervisor] map region hypervisor .text: [{:#x}, {:#x}) -> [{:#x}, {:#x}) {:?}",
        stext, etext, stext, etext, pte_flags
    );
    HYPERVISOR_PAGE_TABLE
        .lock()
        .map_region(
            stext.into(),
            stext.into(),
            (etext - stext) / PAGE_SIZE_4K,
            pte_flags,
        )
        .expect("should work fine");
    assert_eq!(
        HYPERVISOR_PAGE_TABLE
            .lock()
            .query_page(stext.into())
            .unwrap(),
        (stext.into(), pte_flags)
    );

    let srodata = srodata as usize;
    let erodata = erodata as usize;
    assert_eq!(srodata % PAGE_SIZE_4K, 0);
    assert_eq!(erodata % PAGE_SIZE_4K, 0);
    let pte_flags = PTEFlags::V | PTEFlags::R;
    info!(
        "[Hypervisor] map region hypervisor .rodata: [{:#x}, {:#x}) -> [{:#x}, {:#x}) {:?}",
        srodata, erodata, srodata, erodata, pte_flags
    );
    HYPERVISOR_PAGE_TABLE
        .lock()
        .map_region(
            srodata.into(),
            srodata.into(),
            (erodata - srodata) / PAGE_SIZE_4K,
            pte_flags,
        )
        .expect("should work fine");
    assert_eq!(
        HYPERVISOR_PAGE_TABLE
            .lock()
            .query_page(srodata.into())
            .unwrap(),
        (srodata.into(), pte_flags)
    );

    let sdata = sdata as usize;
    let edata = edata as usize;
    assert_eq!(sdata % PAGE_SIZE_4K, 0);
    assert_eq!(edata % PAGE_SIZE_4K, 0);
    let pte_flags = PTEFlags::V | PTEFlags::R | PTEFlags::W;
    info!(
        "[Hypervisor] map region hypervisor .data: [{:#x}, {:#x}) -> [{:#x}, {:#x}) {:?}",
        sdata, edata, sdata, edata, pte_flags
    );
    HYPERVISOR_PAGE_TABLE
        .lock()
        .map_region(
            sdata.into(),
            sdata.into(),
            (edata - sdata) / PAGE_SIZE_4K,
            pte_flags,
        )
        .expect("should work fine");
    assert_eq!(
        HYPERVISOR_PAGE_TABLE
            .lock()
            .query_page(sdata.into())
            .unwrap(),
        (sdata.into(), pte_flags)
    );

    let sbss = sbss_with_stack as usize;
    let ebss = ebss as usize;
    assert_eq!(sbss % PAGE_SIZE_4K, 0);
    assert_eq!(ebss % PAGE_SIZE_4K, 0);
    let pte_flags = PTEFlags::V | PTEFlags::R | PTEFlags::W;
    info!(
        "[Hypervisor] map region hypervisor .bss: [{:#x}, {:#x}) -> [{:#x}, {:#x}) {:?}",
        sbss, ebss, sbss, ebss, pte_flags
    );
    HYPERVISOR_PAGE_TABLE
        .lock()
        .map_region(
            sbss.into(),
            sbss.into(),
            (ebss - sbss) / PAGE_SIZE_4K,
            pte_flags,
        )
        .expect("should work fine");
    assert_eq!(
        HYPERVISOR_PAGE_TABLE
            .lock()
            .query_page(sbss.into())
            .unwrap(),
        (sbss.into(), pte_flags)
    );
}

pub fn map_free_memory(meta: &MachineMeta) {
    extern "C" {
        fn ehypervisor();
    }
    let free_mem_start = align_up(ehypervisor as usize, PAGE_SIZE_4K);
    let phys_mem_end = meta.phys_mem_start + meta.phys_mem_size;
    let free_mem_end = align_down(phys_mem_end, PAGE_SIZE_4K);
    assert_eq!(free_mem_start % PAGE_SIZE_4K, 0);
    assert_eq!(free_mem_end % PAGE_SIZE_4K, 0);
    let pte_flags = PTEFlags::V | PTEFlags::R | PTEFlags::W;
    info!(
        "[Hypervisor] map region free memory: [{:#x}, {:#x}) -> [{:#x}, {:#x}) {:?}",
        free_mem_start, free_mem_end, free_mem_start, free_mem_end, pte_flags
    );
    HYPERVISOR_PAGE_TABLE
        .lock()
        .map_region(
            free_mem_start.into(),
            free_mem_start.into(),
            (free_mem_end - free_mem_start) / PAGE_SIZE_4K,
            pte_flags,
        )
        .expect("should work fine");

    // test page table
    // free memory should be greater than 4k
    assert_eq!(
        HYPERVISOR_PAGE_TABLE
            .lock()
            .query_page(free_mem_start.into())
            .unwrap(),
        (free_mem_start.into(), pte_flags)
    );
    assert_eq!(
        HYPERVISOR_PAGE_TABLE
            .lock()
            .query_page((free_mem_start + PAGE_SIZE_4K).into())
            .unwrap(),
        ((free_mem_start + PAGE_SIZE_4K).into(), pte_flags)
    );
    assert_eq!(
        HYPERVISOR_PAGE_TABLE
            .lock()
            .query_page((free_mem_start + 2 * PAGE_SIZE_4K).into())
            .unwrap(),
        ((free_mem_start + 2 * PAGE_SIZE_4K).into(), pte_flags)
    );
    assert_eq!(
        HYPERVISOR_PAGE_TABLE
            .lock()
            .translate((free_mem_start + 1).into())
            .unwrap(),
        (free_mem_start + 1).into()
    );
    assert_eq!(
        HYPERVISOR_PAGE_TABLE
            .lock()
            .translate((free_mem_end - 1).into())
            .unwrap(),
        (free_mem_end - 1).into()
    );
}
