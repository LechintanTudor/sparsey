use std::ops::{BitOr, BitOrAssign};

const U16_BITS: usize = u16::BITS as usize;
const U32_BITS: usize = u32::BITS as usize;

#[derive(Clone, Debug)]
pub struct MaskIndexIter {
    mask: u32,
    offset: u32,
}

impl MaskIndexIter {
    #[inline]
    fn new(mask: u32) -> MaskIndexIter {
        MaskIndexIter { mask, offset: 0 }
    }
}

impl Iterator for MaskIndexIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let trailing_zeros = self.mask.trailing_zeros();

        if trailing_zeros == u32::BITS {
            return None;
        }

        self.mask >>= trailing_zeros + 1;
        self.offset += trailing_zeros;

        let index = self.offset as usize;
        self.offset += 1;

        Some(index)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct FamilyMask(u32);

impl FamilyMask {
    pub const NONE: Self = Self(0);
    pub const ALL: Self = Self(u32::MAX);

    #[inline]
    pub fn from_family_index(index: usize) -> Self {
        assert!(index < U32_BITS);
        Self(1 << index)
    }

    #[inline]
    pub fn contains_index(self, index: usize) -> bool {
        (self.0 & (1 << index)) != 0
    }

    #[inline]
    pub fn iter_indexes(self) -> MaskIndexIter {
        MaskIndexIter::new(self.0)
    }

    #[inline]
    pub fn get(self) -> u32 {
        self.0
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
pub struct GroupMask(u32);

impl GroupMask {
    pub const NONE: Self = Self(0);
    pub const ALL: Self = Self(u32::MAX);

    pub fn new(group_index: usize, group_arity: usize, family_arity: usize) -> Self {
        assert!(group_index < U32_BITS);
        assert!(group_arity < U32_BITS);
        assert!(family_arity < U32_BITS);
        assert!(group_arity <= family_arity);

        let mut mask = 0;

        for i in 0..(family_arity - group_arity + 1) {
            mask |= 1 << i;
        }

        Self(mask << group_index)
    }

    #[inline]
    pub fn contains_index(self, index: usize) -> bool {
        (self.0 & (1 << index)) != 0
    }

    #[inline]
    pub fn iter_indexes(self) -> MaskIndexIter {
        MaskIndexIter::new(self.0)
    }

    #[inline]
    pub fn get(self) -> u32 {
        self.0
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
pub struct StorageMask(u16);

impl StorageMask {
    pub const NONE: Self = Self(0);
    pub const ALL: Self = Self(u16::MAX);

    #[inline]
    pub fn from_storage_index(index: usize) -> Self {
        assert!(index < U16_BITS);
        Self(1 << index)
    }

    #[inline]
    pub fn get(self) -> u16 {
        self.0
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

// #[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
// pub struct QueryMask {
//     include: StorageMask,
//     exclude: StorageMask,
// }

// impl QueryMask {
//     #[inline]
//     pub const fn new(include: StorageMask, exclude: StorageMask) -> Self {
//         Self { include, exclude }
//     }

//     pub fn for_include_group(group_arity: usize) -> Self {
//         todo!()
//     }

//     pub fn for_exclude_group(prev_group_arity: usize, group_arity: usize) -> Self {
//         todo!()
//     }
// }

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub(crate) struct QueryMask {
    include: u16,
    exclude: u16,
}

impl QueryMask {
    pub const fn new(include: u16, exclude: u16) -> Self {
        Self { include, exclude }
    }

    pub const fn new_include_group(arity: usize) -> Self {
        Self { include: (1 << arity) - 1, exclude: 0 }
    }

    pub const fn new_exclude_group(prev_arity: usize, arity: usize) -> Self {
        if prev_arity != 0 {
            let exclude_count = arity - prev_arity;

            Self {
                include: (1 << prev_arity) - 1,
                exclude: ((1 << exclude_count) - 1) << prev_arity,
            }
        } else {
            Self::new(0, 0)
        }
    }
}
