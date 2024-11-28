use bit_field::BitField;

#[derive(Copy, Clone, Debug)]
pub struct Hcounteren {
    bits: usize,
}

impl Hcounteren {
    #[inline]
    pub fn read() -> Self {
        private::read()
    }
    #[inline]
    pub fn write(&self) {
        private::write(*self);
    }

    #[inline]
    pub fn from_bits(x: usize) -> Self {
        return Hcounteren { bits: x };
    }

    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    #[inline]
    pub fn cycle(&self) -> bool {
        self.bits.get_bit(0)
    }
    #[inline]
    pub fn set_cycle(&mut self, val: bool) {
        self.bits.set_bit(0, val);
    }

    #[inline]
    pub fn time(&self) -> bool {
        self.bits.get_bit(1)
    }
    #[inline]
    pub fn set_time(&mut self, val: bool) {
        self.bits.set_bit(1, val);
    }

    #[inline]
    pub fn instret(&self) -> bool {
        self.bits.get_bit(2)
    }
    #[inline]
    pub fn set_instret(&mut self, val: bool) {
        self.bits.set_bit(2, val);
    }
}

mod private {
    use super::Hcounteren;
    use riscv::{read_csr_as, write_csr_as};

    read_csr_as!(Hcounteren, 1542);
    write_csr_as!(Hcounteren, 1542);
}
