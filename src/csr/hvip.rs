use bit_field::BitField;

#[derive(Copy, Clone, Debug)]
pub struct Hvip {
    bits: usize,
}

impl Hvip {
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
        return Self { bits: x };
    }

    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    #[inline]
    pub fn vs_software_interrupt(&self) -> bool {
        self.bits.get_bit(2)
    }
    #[inline]
    pub fn set_vs_software_interrupt(&mut self, val: bool) {
        self.bits.set_bit(2, val);
    }

    #[inline]
    pub fn vs_timer_interrupt(&self) -> bool {
        self.bits.get_bit(6)
    }
    #[inline]
    pub fn set_vs_timer_interrupt(&mut self, val: bool) {
        self.bits.set_bit(6, val);
    }

    #[inline]
    pub fn vs_external_interrupt(&self) -> bool {
        self.bits.get_bit(10)
    }
    #[inline]
    pub fn set_vs_external_interrupt(&mut self, val: bool) {
        self.bits.set_bit(10, val);
    }
}

mod private {
    use super::Hvip;
    use riscv::{read_csr_as, write_csr_as};

    read_csr_as!(Hvip, 1605);
    write_csr_as!(Hvip, 1605);
}
