use crate::entity::{Group, QueryMask, StorageMask};
use std::ops::Range;
use std::{cmp, ptr};

/// Holds grouping information about a set of components.
#[derive(Clone, Copy, Debug)]
pub struct GroupInfo<'a> {
    groups: &'a [Group],
    storage_mask: StorageMask,
}

impl<'a> GroupInfo<'a> {
    #[inline]
    #[must_use]
    pub(crate) const unsafe fn new(groups: &'a [Group], storage_mask: StorageMask) -> Self {
        Self {
            groups,
            storage_mask,
        }
    }

    /// Tries to combine two group infos.
    ///
    /// Returns the combined group info or [`None`] if the group infos were incompatibe.
    #[inline]
    #[must_use]
    pub fn combine(&self, other: &Self) -> Option<Self> {
        if !ptr::eq(self.groups.as_ptr(), other.groups.as_ptr()) {
            return None;
        }

        Some(Self {
            groups: cmp::max_by_key(self.groups, other.groups, |g| g.len()),
            storage_mask: self.storage_mask | other.storage_mask,
        })
    }

    /// For complete groups, returns the group range of the components.
    #[inline]
    #[must_use]
    pub fn include_group_range(&self) -> Option<Range<usize>> {
        let group = unsafe { self.groups.last().unwrap_unchecked() };

        let mask = QueryMask {
            include: self.storage_mask,
            exclude: StorageMask::EMPTY,
        };

        (mask == group.metadata.include_mask).then_some(0..group.len)
    }

    /// For complete groups, returns the group range of the components.
    #[inline]
    #[must_use]
    pub fn exclude_group_range(&self, exclude: &GroupInfo) -> Option<Range<usize>> {
        if !ptr::eq(self.groups.as_ptr(), exclude.groups.as_ptr()) {
            return None;
        }

        let groups = cmp::max_by_key(self.groups, exclude.groups, |g| g.len());
        let group = unsafe { groups.last().unwrap_unchecked() };

        let mask = QueryMask {
            include: self.storage_mask,
            exclude: exclude.storage_mask,
        };

        if mask != group.metadata.exclude_mask {
            return None;
        }

        let prev_group = unsafe { groups.get_unchecked(groups.len() - 2) };
        Some(group.len..prev_group.len)
    }
}
