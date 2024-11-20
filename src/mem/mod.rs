pub mod addr;
pub mod page_table;
pub mod pte;
pub mod region;

use page_table::PageTable;
use spin::Lazy;

// pub static HYPERVISOR_PAGE_TABLE: Lazy<PageTable> = Lazy::new();