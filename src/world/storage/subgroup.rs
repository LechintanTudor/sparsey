use std::ops::{BitOr, BitOrAssign};
use std::ptr;

#[derive(Copy, Clone)]
pub struct Subgroup {
    arity: usize,
    pub(crate) len: usize,
}

impl Subgroup {
    pub fn with_arity(arity: usize) -> Self {
        Self { arity, len: 0 }
    }

    pub fn arity(&self) -> usize {
        self.arity
    }

    pub fn mask(&self) -> SubgroupMask {
        SubgroupMask::from_arity(self.arity)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct SubgroupMask(u32);

impl SubgroupMask {
    pub fn from_arity(arity: usize) -> Self {
        Self((1 << arity) - 1)
    }

    pub fn from_index(index: usize) -> Self {
        Self(1 << index)
    }
}

impl BitOr for SubgroupMask {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        Self(self.0 | other.0)
    }
}

impl BitOrAssign for SubgroupMask {
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0
    }
}

#[derive(Copy, Clone)]
pub struct SubgroupInfo<'a> {
    subgroups: &'a [Subgroup],
    subgroup_index: usize,
    sparse_set_index: usize,
}

impl<'a> SubgroupInfo<'a> {
    pub fn new(subgroups: &'a [Subgroup], subgroup_index: usize, sparse_set_index: usize) -> Self {
        Self {
            subgroups,
            subgroup_index,
            sparse_set_index,
        }
    }

    pub fn has_same_group(&self, other: &SubgroupInfo) -> bool {
        ptr::eq(self.subgroups, other.subgroups)
    }

    pub fn subgroups(&self) -> &[Subgroup] {
        self.subgroups
    }

    pub fn subgroup_index(&self) -> usize {
        self.subgroup_index
    }

    pub fn mask(&self) -> SubgroupMask {
        SubgroupMask::from_index(self.sparse_set_index)
    }
}

pub(crate) fn get_subgroup_len(subgroup_infos: &[SubgroupInfo]) -> Option<usize> {
    let (first, others) = subgroup_infos.split_first()?;
    let subgroups = first.subgroups();

    let mut subgroup_index = first.subgroup_index();
    let mut subgroup_mask = first.mask();

    for other in others {
        if !first.has_same_group(other) {
            return None;
        }

        subgroup_index = subgroup_index.max(other.subgroup_index());
        subgroup_mask |= other.mask();
    }

    let subgroup = unsafe { subgroups.get_unchecked(subgroup_index) };
    (subgroup.mask() == subgroup_mask).then(|| subgroup.len)
}
