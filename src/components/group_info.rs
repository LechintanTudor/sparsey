use crate::components::{Group, QueryMask, StorageMask};
use std::marker::PhantomData;
use std::ops::Range;
use std::ptr::NonNull;

/// Tracks the group to which one or more component storages belong to.
#[derive(Clone, Copy)]
pub struct GroupInfo<'a> {
    family: NonNull<Group>,
    group_offset: usize,
    storage_mask: StorageMask,
    _phantom: PhantomData<&'a [Group]>,
}

unsafe impl Send for GroupInfo<'_> {}
unsafe impl Sync for GroupInfo<'_> {}

impl<'a> GroupInfo<'a> {
    pub(crate) unsafe fn new(
        family: NonNull<Group>,
        group_offset: usize,
        storage_mask: StorageMask,
    ) -> Self {
        Self { family, group_offset, storage_mask, _phantom: PhantomData }
    }

    pub(crate) fn combine(self, info: Self) -> Option<Self> {
        if self.family != info.family {
            return None;
        }

        Some(Self {
            family: self.family,
            group_offset: self.group_offset.max(info.group_offset),
            storage_mask: self.storage_mask | info.storage_mask,
            _phantom: PhantomData,
        })
    }

    pub(crate) fn group_len(&self) -> Option<usize> {
        let group = unsafe { *self.family.as_ptr().add(self.group_offset) };
        let mask = QueryMask::new(self.storage_mask, 0);

        (mask == group.metadata().include_mask()).then(|| group.len())
    }

    pub(crate) fn exclude_group_range(&self, exclude: &GroupInfo) -> Option<Range<usize>> {
        if self.family != exclude.family {
            return None;
        }

        let mask = QueryMask::new(self.storage_mask, exclude.storage_mask);
        let group_offset = self.group_offset.max(exclude.group_offset);

        unsafe {
            let group = *self.family.as_ptr().add(group_offset);

            if mask == group.metadata().exclude_mask() {
                let prev_group = *self.family.as_ptr().add(group_offset - 1);
                Some(group.len()..prev_group.len())
            } else {
                None
            }
        }
    }
}
