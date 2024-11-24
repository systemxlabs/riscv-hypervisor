#[derive(Copy, Clone, Debug)]
pub struct Vsstatus {
    bits: usize,
}

impl Vsstatus {
    #[inline]
    pub fn read() -> Self {
        private::read()
    }
    #[inline]
    pub fn write(&self) {
        private::write(*self);
    }
}

mod private {
    use super::Vsstatus;
    use riscv::{read_csr_as, write_csr_as};

    read_csr_as!(Vsstatus, 1536);
    write_csr_as!(Vsstatus, 1536);
}
