use crate::components::{Group, QueryMask, StorageMask};
use std::cmp;
use std::ops::Range;

/// Tracks the group to which one or more component views belong to.
#[derive(Clone)]
pub struct GroupInfo<'a> {
    /// The groups to which this component view belongs.
    groups: &'a [Group],
    /// Bitmask for storage indexes in the group family.
    storage_mask: StorageMask,
}

impl<'a> GroupInfo<'a> {
    pub(crate) unsafe fn new(groups: &'a [Group], storage_mask: StorageMask) -> Self {
        debug_assert!(!groups.is_empty());

        Self {
            groups,
            storage_mask,
        }
    }

    pub(crate) fn combine(self, include: Self) -> Option<Self> {
        if self.groups.as_ptr() != include.groups.as_ptr() {
            return None;
        }

        Some(Self {
            groups: cmp::max_by_key(self.groups, include.groups, |groups| groups.len()),
            storage_mask: self.storage_mask | include.storage_mask,
        })
    }

    pub(crate) fn group_len(&self) -> Option<usize> {
        let group = unsafe { self.groups.last().unwrap_unchecked() };
        let mask = QueryMask::new(self.storage_mask, StorageMask::NONE);

        (mask == group.metadata().include_mask()).then(|| group.len())
    }

    pub(crate) fn exclude_group_range(&self, exclude: &GroupInfo) -> Option<Range<usize>> {
        if self.groups.as_ptr() != exclude.groups.as_ptr() {
            return None;
        }

        let mask = QueryMask::new(self.storage_mask, exclude.storage_mask);
        let groups = cmp::max_by_key(self.groups, exclude.groups, |groups| groups.len());

        unsafe {
            let group = groups.last().unwrap_unchecked();

            if mask != group.metadata().exclude_mask() {
                return None;
            }

            let prev_group = groups.get_unchecked(groups.len() - 2);
            Some(group.len()..prev_group.len())
        }
    }
}
