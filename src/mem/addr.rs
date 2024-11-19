use core::fmt;

/// Align address upwards.
///
/// Returns the smallest `x` with alignment `align` so that `x >= addr`.
///
/// The alignment must be a power of two.
#[inline]
pub const fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

/// Align address downwards.
///
/// Returns the greatest `x` with alignment `align` so that `x <= addr`.
///
/// The alignment must be a power of two.
#[inline]
pub const fn align_down(addr: usize, align: usize) -> usize {
    addr & !(align - 1)
}

/// A physical memory address.
///
/// It's a wrapper type around an `usize`.
#[repr(transparent)]
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysAddr(usize);

impl fmt::Debug for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("PA:{:#x}", self.0))
    }
}

impl PhysAddr {
    /// Converts the address to an `usize`.
    #[inline]
    pub const fn as_usize(self) -> usize {
        self.0
    }

    /// Aligns the address upwards to the given alignment.
    ///
    /// See the [`align_up`] function for more information.
    #[inline]
    pub fn align_up<U>(self, align: U) -> Self
        where
            U: Into<usize>,
    {
        Self(align_up(self.0, align.into()))
    }

    /// Aligns the address downwards to the given alignment.
    ///
    /// See the [`align_down`] function for more information.
    #[inline]
    pub fn align_down<U>(self, align: U) -> Self
        where
            U: Into<usize>,
    {
        Self(align_down(self.0, align.into()))
    }
}

impl From<usize> for PhysAddr {
    #[inline]
    fn from(addr: usize) -> Self {
        Self(addr)
    }
}