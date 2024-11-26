use bit_field::BitField;

#[derive(Clone, Copy, Debug)]
pub struct Sstatus {
    bits: usize,
}

impl Sstatus {
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
    pub fn spp(&self) -> bool {
        self.bits.get_bit(8)
    }

    #[inline]
    pub fn set_spp(&mut self, val: bool) {
        self.bits.set_bit(8, val);
    }
}

mod private {
    use super::Sstatus;
    use riscv::{read_csr_as, write_csr_as};

    read_csr_as!(Sstatus, 0x100);
    write_csr_as!(Sstatus, 0x100);
}
