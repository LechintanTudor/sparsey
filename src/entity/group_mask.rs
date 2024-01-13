use crate::entity::{MAX_GROUP_ARITY, MAX_GROUP_COUNT};
use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GroupMask(pub u64);

impl GroupMask {
    #[inline]
    #[must_use]
    pub const fn single(index: usize) -> Self {
        assert!(index < MAX_GROUP_COUNT);
        Self(1 << index)
    }

    #[inline]
    #[must_use]
    pub const fn from_to(from: usize, to: usize) -> Self {
        assert!(from < MAX_GROUP_COUNT);
        assert!(to < MAX_GROUP_COUNT);
        assert!(from <= to);

        Self(((1 << (to - from)) - 1) << from)
    }

    #[inline]
    #[must_use]
    pub const fn skip_from_to(from: usize, to: usize) -> Self {
        assert!(from < MAX_GROUP_COUNT);
        assert!(to < MAX_GROUP_COUNT);
        assert!(from <= to);

        Self(!(((1 << (to - from)) - 1) << from))
    }

    #[inline]
    pub const fn iter_bit_indexes(&self) -> BitIndexIter {
        BitIndexIter(self.0)
    }
}

impl BitAnd for GroupMask {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for GroupMask {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for GroupMask {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for GroupMask {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl fmt::Debug for GroupMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>64b}", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct StorageMask(pub u16);

impl StorageMask {
    #[inline]
    #[must_use]
    pub const fn single(index: usize) -> Self {
        assert!(index < MAX_GROUP_ARITY);
        Self(1 << index)
    }

    #[inline]
    #[must_use]
    pub const fn from_to(from: usize, to: usize) -> Self {
        assert!(from < MAX_GROUP_ARITY);
        assert!(to < MAX_GROUP_ARITY);
        assert!(from <= to);

        Self(((1 << (to - from)) - 1) << from)
    }
}

impl BitOr for StorageMask {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for StorageMask {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl fmt::Debug for StorageMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>16b}", self.0)
    }
}

#[repr(align(4))]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub struct QueryMask {
    pub include: StorageMask,
    pub exclude: StorageMask,
}

impl QueryMask {
    #[inline]
    #[must_use]
    pub const fn include(arity: usize) -> Self {
        Self {
            include: StorageMask::from_to(0, arity),
            exclude: StorageMask(0),
        }
    }

    #[inline]
    #[must_use]
    pub const fn exclude(prev_arity: usize, arity: usize) -> Self {
        Self {
            include: StorageMask::from_to(0, prev_arity),
            exclude: StorageMask::from_to(prev_arity, arity),
        }
    }
}

#[must_use]
#[derive(Clone, Debug)]
pub struct BitIndexIter(pub u64);

impl Iterator for BitIndexIter {
    type Item = u32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }

        let trailing_zeros = self.0.trailing_zeros();
        self.0 &= !(1 << trailing_zeros);
        Some(trailing_zeros)
    }
}

impl DoubleEndedIterator for BitIndexIter {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }

        let index = 63 - self.0.leading_zeros();
        self.0 &= !(1 << index);
        Some(index)
    }
}
