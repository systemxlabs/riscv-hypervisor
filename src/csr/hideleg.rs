use bit_field::BitField;

#[derive(Copy, Clone, Debug)]
pub struct Hideleg {
    bits: usize,
}

impl Hideleg {
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

    /// Timer Interrupt
    #[inline]
    pub fn vs_timer_interrupt(&self) -> bool {
        self.bits.get_bit(6)
    }
    #[inline]
    pub fn set_vs_timer_interrupt(&mut self, val: bool) {
        self.bits.set_bit(6, val);
    }

    /// External Interrupt
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
    use super::Hideleg;
    use riscv::{read_csr_as, write_csr_as};

    read_csr_as!(Hideleg, 1539);
    write_csr_as!(Hideleg, 1539);
}
