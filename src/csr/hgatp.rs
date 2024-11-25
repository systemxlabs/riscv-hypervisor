use bit_field::BitField;

#[derive(Clone, Copy, Debug)]
pub struct Hgatp {
    bits: usize,
}

impl Hgatp {
    #[inline]
    pub fn read() -> Self {
        private::read()
    }
    #[inline]
    pub fn write(&self) {
        private::write(*self);
    }

    /// Guest address translation mode.
    #[inline]
    pub fn mode(&self) -> Mode {
        Mode::from(self.bits.get_bits(60..64))
    }
    #[inline]
    pub fn set_mode(&mut self, val: Mode) {
        self.bits.set_bits(60..64, val as usize);
    }

    /// Virtual machine ID.
    #[inline]
    pub fn vmid(&self) -> usize {
        self.bits.get_bits(44..58)
    }
    #[inline]
    pub fn set_vmid(&mut self, val: usize) {
        self.bits.set_bits(44..58, val);
    }

    /// Physical Page Number for root page table.
    #[inline]
    pub fn ppn(&self) -> usize {
        self.bits.get_bits(0..44)
    }
    #[inline]
    pub fn set_ppn(&mut self, val: usize) {
        self.bits.set_bits(0..44, val);
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(usize)]
pub enum Mode {
    Bare = 0,
    Sv39x4 = 8,
    Sv48x4 = 9,
}
impl From<usize> for Mode {
    fn from(x: usize) -> Self {
        match x {
            0 => Self::Bare,
            8 => Self::Sv39x4,
            9 => Self::Sv48x4,
            _ => unreachable!(),
        }
    }
}

mod private {
    use super::Hgatp;
    use riscv::{read_csr_as, write_csr_as};

    read_csr_as!(Hgatp, 1664);
    write_csr_as!(Hgatp, 1664);
}
