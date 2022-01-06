use crate::components::{Group, QueryMask, StorageMask};
use std::marker::PhantomData;
use std::ops::Range;
use std::ptr::NonNull;

#[derive(Clone)]
pub struct GroupInfo<'a> {
    group: NonNull<Group>,
    offset: usize,
    mask: StorageMask,
    _phantom: PhantomData<&'a [Group]>,
}

impl<'a> GroupInfo<'a> {
    pub(crate) unsafe fn new(group: NonNull<Group>, offset: usize, mask: StorageMask) -> Self {
        Self { group, offset, mask, _phantom: PhantomData }
    }

    pub(crate) fn combine(self, info: Self) -> Option<Self> {
        if self.group != info.group {
            return None;
        }

        Some(Self {
            group: self.group,
            offset: self.offset.max(info.offset),
            mask: self.mask | info.mask,
            _phantom: PhantomData,
        })
    }
}

pub fn group_range(include: GroupInfo, exclude: Option<GroupInfo>) -> Option<Range<usize>> {
    let (mask, offset) = match exclude {
        Some(exclude) => {
            if include.group != exclude.group {
                return None;
            }

            (QueryMask::new(include.mask, exclude.mask), include.offset.max(exclude.offset))
        }
        None => (QueryMask::new(include.mask, StorageMask::default()), include.offset),
    };

    unsafe {
        let group = *include.group.as_ptr().add(offset);

        if mask == group.include_mask() {
            Some(0..group.len())
        } else if mask == group.exclude_mask() {
            let prev_group = *include.group.as_ptr().add(offset - 1);
            Some(group.len()..prev_group.len())
        } else {
            None
        }
    }
}
