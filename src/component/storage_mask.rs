use crate::component::MAX_GROUP_ARITY;
use core::num::NonZeroU16;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(crate) struct StorageMask(u16);

impl StorageMask {
    pub const EMPTY: Self = Self(0);

    #[inline]
    #[must_use]
    pub fn from_to(from: usize, to: usize) -> Self {
        assert!(from < MAX_GROUP_ARITY);
        assert!(to < MAX_GROUP_ARITY);
        assert!(from <= to);

        Self(((1 << (to - from)) - 1) << from)
    }
}

impl From<NonZeroStorageMask> for StorageMask {
    #[inline]
    fn from(mask: NonZeroStorageMask) -> Self {
        Self(mask.0.get())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct NonZeroStorageMask(NonZeroU16);

impl NonZeroStorageMask {
    #[inline]
    #[must_use]
    pub fn single(index: usize) -> Self {
        assert!(index < MAX_GROUP_ARITY);
        Self(NonZeroU16::new(1 << index).unwrap())
    }
}

macro_rules! impl_common {
    ($Ty:ty) => {
        impl ::core::ops::BitOr for $Ty {
            type Output = Self;

            #[inline]
            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl ::core::ops::BitOrAssign for $Ty {
            #[inline]
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }

        impl ::core::fmt::Debug for $Ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::write!(f, "{:0>16b}", self.0)
            }
        }
    };
}

impl_common!(StorageMask);
impl_common!(NonZeroStorageMask);
