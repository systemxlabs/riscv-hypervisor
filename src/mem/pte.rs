use crate::mem::addr::HostPhysAddr;

bitflags::bitflags! {
    /// Page-table entry flags.
    pub struct PTEFlags: usize {
        /// Whether the PTE is valid.
        const V =   1 << 0;
        /// Whether the page is readable.
        const R =   1 << 1;
        /// Whether the page is writable.
        const W =   1 << 2;
        /// Whether the page is executable.
        const X =   1 << 3;
        /// Whether the page is accessible to user mode.
        const U =   1 << 4;
        /// Designates a global mapping.
        const G =   1 << 5;
        /// Indicates the virtual page has been read, written, or fetched from
        /// since the last time the A bit was cleared.
        const A =   1 << 6;
        /// Indicates the virtual page has been written since the last time the
        /// D bit was cleared.
        const D =   1 << 7;
    }
}

pub struct PageTableEntry(u64);

impl PageTableEntry {
    const PHYS_ADDR_MASK: u64 = (1 << 54) - (1 << 10); // bits 10..54

    pub fn new(paddr: HostPhysAddr, flags: PTEFlags) -> Self {
        Self(flags.bits() as u64 | ((paddr.as_usize() >> 2) as u64 & Self::PHYS_ADDR_MASK))
    }
    pub fn empty() -> Self {
        PageTableEntry(0)
    }
    pub fn ppn(&self) -> HostPhysAddr {
        HostPhysAddr::from(((self.0 & Self::PHYS_ADDR_MASK) << 2) as usize)
    }
    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits_truncate(self.0 as usize)
    }
    pub fn is_valid(&self) -> bool {
        (self.flags() & PTEFlags::V) != PTEFlags::empty()
    }
    pub fn readable(&self) -> bool {
        (self.flags() & PTEFlags::R) != PTEFlags::empty()
    }
    pub fn writable(&self) -> bool {
        (self.flags() & PTEFlags::W) != PTEFlags::empty()
    }
    pub fn executable(&self) -> bool {
        (self.flags() & PTEFlags::X) != PTEFlags::empty()
    }
    pub fn is_user(&self) -> bool {
        (self.flags() & PTEFlags::U) != PTEFlags::empty()
    }
    pub fn is_global(&self) -> bool {
        (self.flags() & PTEFlags::G) != PTEFlags::empty()
    }
    pub fn dirty(&self) -> bool {
        (self.flags() & PTEFlags::D) != PTEFlags::empty()
    }
    pub fn accessed(&self) -> bool {
        (self.flags() & PTEFlags::A) != PTEFlags::empty()
    }
    pub fn is_unused(&self) -> bool {
        self.0 == 0
    }
}
