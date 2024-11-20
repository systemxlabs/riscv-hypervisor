use crate::mem::addr::PhysAddr;

pub struct PageTable {
    root_paddr: PhysAddr,
}
