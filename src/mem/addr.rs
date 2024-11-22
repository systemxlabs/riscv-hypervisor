use core::fmt;
use core::ops::{Add, AddAssign, Sub, SubAssign};

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

/// Returns the offset of the address within the alignment.
///
/// Equivalent to `addr % align`, but the alignment must be a power of two.
#[inline]
pub const fn align_offset(addr: usize, align: usize) -> usize {
    addr & (align - 1)
}

/// Checks whether the address has the demanded alignment.
///
/// Equivalent to `addr % align == 0`, but the alignment must be a power of two.
#[inline]
pub const fn is_aligned(addr: usize, align: usize) -> bool {
    align_offset(addr, align) == 0
}

macro_rules! impl_common_addr_methods {
    ($t: ty) => {
        impl $t {
            pub const fn new(addr: usize) -> Self {
                Self(addr)
            }

            #[inline]
            pub const fn as_usize(self) -> usize {
                self.0
            }

            #[inline]
            pub fn is_aligned<U>(self, align: U) -> bool
            where
                U: Into<usize>,
            {
                is_aligned(self.0, align.into())
            }

            #[inline]
            pub fn align_up<U>(self, align: U) -> Self
            where
                U: Into<usize>,
            {
                Self(align_up(self.0, align.into()))
            }

            #[inline]
            pub fn align_down<U>(self, align: U) -> Self
            where
                U: Into<usize>,
            {
                Self(align_down(self.0, align.into()))
            }
        }

        impl From<usize> for $t {
            #[inline]
            fn from(addr: usize) -> Self {
                Self(addr)
            }
        }

        impl Add<usize> for $t {
            type Output = Self;
            #[inline]
            fn add(self, rhs: usize) -> Self {
                Self(self.0 + rhs)
            }
        }

        impl AddAssign<usize> for $t {
            #[inline]
            fn add_assign(&mut self, rhs: usize) {
                *self = *self + rhs;
            }
        }

        impl Sub<usize> for $t {
            type Output = Self;
            #[inline]
            fn sub(self, rhs: usize) -> Self {
                Self(self.0 - rhs)
            }
        }

        impl SubAssign<usize> for $t {
            #[inline]
            fn sub_assign(&mut self, rhs: usize) {
                *self = *self - rhs;
            }
        }
    };
}

impl_common_addr_methods!(HostPhysAddr);
impl_common_addr_methods!(HostVirtAddr);

#[repr(transparent)]
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct HostPhysAddr(usize);

impl fmt::Debug for HostPhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("HPA:{:#x}", self.0))
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct HostVirtAddr(usize);

impl fmt::Debug for HostVirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("HVA:{:#x}", self.0))
    }
}
