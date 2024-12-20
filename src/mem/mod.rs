pub mod addr;
mod guest_page_table;
pub mod page_table;
pub mod pte;
pub mod region;

pub use addr::*;
pub use guest_page_table::*;
pub use page_table::*;
pub use pte::*;
pub use region::*;

use page_table::HYPERVISOR_PAGE_TABLE;
use riscv::register::satp;

pub fn enable_mmu() {
    // host virtual address -> host physical address
    let page_table_root = HYPERVISOR_PAGE_TABLE.lock().root_paddr().as_usize();
    unsafe {
        satp::set(satp::Mode::Sv39, 0, page_table_root >> 12);
        riscv::asm::sfence_vma_all();
    }
}
