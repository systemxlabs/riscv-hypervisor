use page_table::HYPERVISOR_PAGE_TABLE;
use riscv::register::satp;

pub mod addr;
pub mod page_table;
pub mod pte;
pub mod region;

pub fn init_mmu() {
    let page_table_root = HYPERVISOR_PAGE_TABLE.lock().root_paddr().as_usize();
    unsafe {
        satp::set(satp::Mode::Sv39, 0, page_table_root >> 12);
        riscv::asm::sfence_vma_all();
    }
}
