use bit_field::BitField;

#[derive(Clone, Copy, Debug)]
pub struct Hstatus {
    bits: usize,
}

impl Hstatus {
    #[inline]
    pub fn read() -> Self {
        private::read()
    }
    #[inline]
    pub fn write(&self) {
        private::write(*self);
    }

    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Supervisor Previous Virtualization mode.
    #[inline]
    pub fn spv(&self) -> bool {
        self.bits.get_bit(7)
    }
    #[inline]
    pub fn set_spv(&mut self, val: bool) {
        self.bits.set_bit(7, val);
    }

    /// Supervisor Previous Virtual Privilege.
    #[inline]
    pub fn spvp(&self) -> bool {
        self.bits.get_bit(8)
    }
    #[inline]
    pub fn set_spvp(&mut self, val: bool) {
        self.bits.set_bit(8, val);
    }

    /// Guest Virtual Address.
    #[inline]
    pub fn gva(&self) -> bool {
        self.bits.get_bit(6)
    }
    #[inline]
    pub fn set_gva(&mut self, val: bool) {
        self.bits.set_bit(6, val);
    }
}

mod private {
    use super::Hstatus;
    use riscv::{read_csr_as, write_csr_as};

    read_csr_as!(Hstatus, 1536);
    write_csr_as!(Hstatus, 1536);
}
