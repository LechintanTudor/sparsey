use std::ops::{BitOr, BitOrAssign};

const STORAGE_MASK_ARITY: usize = u16::BITS as usize;
const GROUP_MASK_ARITY: usize = u32::BITS as usize;

fn first_n_bits_u16(n: usize) -> u16 {
    assert!(n <= u16::BITS as usize);
    (0..n).fold(0, |m, i| m | 1 << i)
}

fn first_n_bits_u32(n: usize) -> u32 {
    assert!(n <= u32::BITS as usize);
    (0..n).fold(0, |m, i| m | 1 << i)
}

#[derive(Clone, Debug)]
pub(crate) struct BitIndexIter {
    value: u32,
}

impl Iterator for BitIndexIter {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.value == 0 {
            return None;
        }

        let trailing_zeros = self.value.trailing_zeros();
        self.value &= !(1 << trailing_zeros);
        Some(trailing_zeros as usize)
    }
}

impl From<u32> for BitIndexIter {
    #[inline]
    fn from(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub(crate) struct FamilyMask(u32);

impl FamilyMask {
    pub const NONE: Self = Self(0);

    #[inline]
    pub fn from_family_index(index: usize) -> Self {
        assert!(index < GROUP_MASK_ARITY);
        Self(1 << index)
    }

    #[inline]
    pub fn iter_bit_indexes(self) -> BitIndexIter {
        BitIndexIter::from(self.0)
    }
}

impl From<u32> for FamilyMask {
    #[inline]
    fn from(mask: u32) -> Self {
        Self(mask)
    }
}

impl BitOr for FamilyMask {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for FamilyMask {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub(crate) struct GroupMask(u32);

impl GroupMask {
    pub const NONE: Self = Self(0);
    pub const ALL: Self = Self(u32::MAX);

    pub fn new(group_index: usize, group_arity: usize, family_arity: usize) -> Self {
        assert!(group_index < GROUP_MASK_ARITY);
        assert!(group_arity < GROUP_MASK_ARITY);
        assert!(family_arity < GROUP_MASK_ARITY);
        assert!(group_arity <= family_arity);

        Self(first_n_bits_u32(family_arity - group_arity + 1) << group_index)
    }

    #[inline]
    pub fn contains_index(self, index: usize) -> bool {
        (self.0 & (1 << index)) != 0
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
        self.0 |= rhs.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub(crate) struct StorageMask(u16);

impl StorageMask {
    pub const NONE: Self = Self(0);

    #[inline]
    pub fn from_storage_index(index: usize) -> Self {
        assert!(index < STORAGE_MASK_ARITY);
        Self(1 << index)
    }
}

impl From<u16> for StorageMask {
    #[inline]
    fn from(mask: u16) -> Self {
        Self(mask)
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
        self.0 |= rhs.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub(crate) struct QueryMask {
    include: StorageMask,
    exclude: StorageMask,
}

impl QueryMask {
    pub const NONE: Self = Self {
        include: StorageMask(0),
        exclude: StorageMask(0),
    };

    #[inline]
    pub const fn new(include: StorageMask, exclude: StorageMask) -> Self {
        Self { include, exclude }
    }

    pub fn for_include_group(group_arity: usize) -> Self {
        assert!(group_arity <= STORAGE_MASK_ARITY);

        Self {
            include: StorageMask(first_n_bits_u16(group_arity)),
            exclude: StorageMask::NONE,
        }
    }

    pub fn for_exclude_group(prev_group_arity: usize, group_arity: usize) -> Self {
        assert!(prev_group_arity <= STORAGE_MASK_ARITY);
        assert!(group_arity <= STORAGE_MASK_ARITY);
        assert!(prev_group_arity < group_arity);

        if prev_group_arity == 0 {
            return Self::NONE;
        }

        let exclude_count = group_arity - prev_group_arity;

        Self {
            include: StorageMask(first_n_bits_u16(prev_group_arity)),
            exclude: StorageMask(first_n_bits_u16(exclude_count) << prev_group_arity),
        }
    }
}
