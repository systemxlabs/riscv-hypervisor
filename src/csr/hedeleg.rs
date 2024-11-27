use bit_field::BitField;

#[derive(Copy, Clone, Debug)]
pub struct Hedeleg {
    bits: usize,
}

impl Hedeleg {
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

    #[inline]
    pub fn inst_addr_misalign(&self) -> bool {
        self.bits.get_bit(0)
    }
    #[inline]
    pub fn set_inst_addr_misalign(&mut self, val: bool) {
        self.bits.set_bit(0, val);
    }

    #[inline]
    pub fn inst_access_fault(&self) -> bool {
        self.bits.get_bit(1)
    }
    #[inline]
    pub fn set_inst_access_fault(&mut self, val: bool) {
        self.bits.set_bit(1, val);
    }

    #[inline]
    pub fn illegal_inst(&self) -> bool {
        self.bits.get_bit(2)
    }
    #[inline]
    pub fn set_illegal_inst(&mut self, val: bool) {
        self.bits.set_bit(2, val);
    }

    #[inline]
    pub fn env_call_from_u_or_vu(&self) -> bool {
        self.bits.get_bit(8)
    }
    #[inline]
    pub fn set_env_call_from_u_or_vu(&mut self, val: bool) {
        self.bits.set_bit(8, val);
    }

    #[inline]
    pub fn inst_page_fault(&self) -> bool {
        self.bits.get_bit(12)
    }
    #[inline]
    pub fn set_inst_page_fault(&mut self, val: bool) {
        self.bits.set_bit(12, val);
    }

    #[inline]
    pub fn load_page_fault(&self) -> bool {
        self.bits.get_bit(13)
    }
    #[inline]
    pub fn set_load_page_fault(&mut self, val: bool) {
        self.bits.set_bit(13, val);
    }

    #[inline]
    pub fn store_page_fault(&self) -> bool {
        self.bits.get_bit(15)
    }
    #[inline]
    pub fn set_store_page_fault(&mut self, val: bool) {
        self.bits.set_bit(15, val);
    }
}

mod private {
    use super::Hedeleg;
    use riscv::{read_csr_as, write_csr_as};

    read_csr_as!(Hedeleg, 1538);
    write_csr_as!(Hedeleg, 1538);
}
