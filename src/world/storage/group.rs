use std::ops::{BitOr, BitOrAssign};
use std::ptr;

/// Holds information about the layout in the `World`
/// of the component storage from which it was created.
#[derive(Copy, Clone)]
pub struct GroupInfo<'a> {
    groups: &'a [Group],
    group_index: usize,
    sparse_set_index: usize,
}

impl<'a> GroupInfo<'a> {
    pub(crate) fn new(groups: &'a [Group], group_index: usize, sparse_set_index: usize) -> Self {
        Self {
            groups,
            group_index,
            sparse_set_index,
        }
    }

    pub(crate) fn has_same_group_set(&self, other: &GroupInfo) -> bool {
        ptr::eq(self.groups, other.groups)
    }

    pub(crate) fn group_index(&self) -> usize {
        self.group_index
    }

    pub(crate) fn mask(&self) -> GroupMask {
        GroupMask::from_index(self.sparse_set_index)
    }

    pub(crate) fn groups(&self) -> &[Group] {
        self.groups
    }
}

#[derive(Copy, Clone)]
pub(crate) struct Group {
    arity: usize,
    pub(crate) len: usize,
}

impl Group {
    pub fn with_arity(arity: usize) -> Self {
        Self { arity, len: 0 }
    }

    pub fn arity(&self) -> usize {
        self.arity
    }

    pub fn mask(&self) -> GroupMask {
        GroupMask::from_arity(self.arity)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub(crate) struct GroupMask(u32);

impl GroupMask {
    pub fn from_arity(arity: usize) -> Self {
        Self((1 << arity) - 1)
    }

    pub fn from_index(index: usize) -> Self {
        Self(1 << index)
    }
}

impl BitOr for GroupMask {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        Self(self.0 | other.0)
    }
}

impl BitOrAssign for GroupMask {
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0
    }
}

pub(crate) fn get_group_len(group_infos: &[GroupInfo]) -> Option<usize> {
    let (first, others) = group_infos.split_first()?;
    let groups = first.groups();

    let mut group_index = first.group_index();
    let mut group_mask = first.mask();

    for other in others {
        if !first.has_same_group_set(other) {
            return None;
        }

        group_index = group_index.max(other.group_index());
        group_mask |= other.mask();
    }

    let group = unsafe { groups.get_unchecked(group_index) };
    (group.mask() == group_mask).then(|| group.len)
}