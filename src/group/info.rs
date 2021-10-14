use crate::components::Group;
use crate::group::{QueryMask, StorageMask};
use std::marker::PhantomData;
use std::ops::Range;
use std::ptr;
use std::ptr::NonNull;

/// Tracks the group to which a `ComponentStorage` belongs.
#[derive(Clone, Copy)]
pub struct GroupInfo<'a> {
    family: NonNull<Group>,
    offset: usize,
    storage_mask: StorageMask,
    _phantom: PhantomData<&'a [Group]>,
}

impl<'a> GroupInfo<'a> {
    /// # Safety
    /// `family` must point to a slice with lifetime `'a`, indexable by
    /// `offset`.
    pub(crate) unsafe fn new(
        family: NonNull<Group>,
        offset: usize,
        storage_mask: StorageMask,
    ) -> Self {
        Self {
            family,
            offset,
            storage_mask,
            _phantom: PhantomData,
        }
    }
}

/// Tracks the group to which multiple `ComponentStorage`s belong.
#[derive(Copy, Clone, Default)]
pub struct CombinedGroupInfo<'a> {
    family: Option<NonNull<Group>>,
    max_offset: usize,
    storage_mask: StorageMask,
    _phantom: PhantomData<&'a [Group]>,
}

impl<'a> CombinedGroupInfo<'a> {
    pub(crate) fn combine(self, info: GroupInfo<'a>) -> Option<Self> {
        match self.family {
            Some(group_family) => {
                ptr::eq(group_family.as_ptr(), info.family.as_ptr()).then(|| CombinedGroupInfo {
                    family: Some(group_family),
                    max_offset: self.max_offset.max(info.offset),
                    storage_mask: self.storage_mask | info.storage_mask,
                    _phantom: PhantomData,
                })
            }
            None => Some(CombinedGroupInfo {
                family: Some(info.family),
                max_offset: info.offset,
                storage_mask: info.storage_mask,
                _phantom: PhantomData,
            }),
        }
    }
}

fn common_family(
    base_family: Option<NonNull<Group>>,
    include_family: Option<NonNull<Group>>,
    exclude_family: Option<NonNull<Group>>,
) -> Option<NonNull<Group>> {
    let mut common_family = base_family;

    for family in [include_family, exclude_family].iter().flatten() {
        match common_family {
            Some(common_family) => {
                if !ptr::eq(family.as_ptr(), common_family.as_ptr()) {
                    return None;
                }
            }
            None => common_family = Some(*family),
        }
    }

    common_family
}

/// Returns the range of elements the storages have in common if the
/// `CombinedGroupInfo`s form a group.
pub(crate) fn group_range(
    base: CombinedGroupInfo,
    include: CombinedGroupInfo,
    exclude: CombinedGroupInfo,
) -> Option<Range<usize>> {
    let family = common_family(base.family, include.family, exclude.family)?;

    let max_offset = base
        .max_offset
        .max(include.max_offset)
        .max(exclude.max_offset);

    let query_mask = QueryMask::new(
        base.storage_mask | include.storage_mask,
        exclude.storage_mask,
    );

    let group = unsafe { &*family.as_ptr().add(max_offset) };

    if query_mask == group.include_mask() {
        Some(0..group.len())
    } else if query_mask == group.exclude_mask() {
        let prev_group = unsafe { &*family.as_ptr().add(max_offset - 1) };
        Some(group.len()..prev_group.len())
    } else {
        None
    }
}
