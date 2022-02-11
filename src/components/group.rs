use crate::components::{GroupMask, QueryMask};
use crate::storage::{ComponentStorage, Entity};
use atomic_refcell::AtomicRefCell;
use std::ops::Range;

/// Example:
///
/// Say we have a family made of two groups:
/// - Group 0: (A, B)
/// - Group 1: (A, B, C, D)
///
/// For 'Group 1', we get such a `Group` struct:
///
/// ```text
/// storages: A B C D
///           ^   ^   ^
///           |   |   +-- end = start + 4
///           |   +------ new_start = start + 2
///           +---------- start
/// ```
#[derive(Clone, Copy, Debug)]
pub(crate) struct GroupMetadata {
    /// The index of the storage at which the group starts.
    start: usize,
    /// The index of the storage at which the group's first new storage starts.
    new_start: usize,
    /// The index of the last storage in the group plus one.
    end: usize,
}

impl GroupMetadata {
    #[inline]
    pub fn storage_range(&self) -> Range<usize> {
        self.start..self.end
    }

    #[inline]
    pub fn new_storage_range(&self) -> Range<usize> {
        self.new_start..self.end
    }

    #[inline]
    pub fn include_mask(&self) -> QueryMask {
        QueryMask::new_include_group(self.end - self.start)
    }

    #[inline]
    pub fn exclude_mask(&self) -> QueryMask {
        QueryMask::new_exclude_group(self.new_start - self.start, self.end - self.start)
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Group {
    metadata: GroupMetadata,
    /// The number of components grouped by this group.
    len: usize,
}

impl Group {
    pub fn new(start: usize, new_start: usize, end: usize) -> Self {
        Self { metadata: GroupMetadata { start, new_start, end }, len: 0 }
    }

    #[inline]
    pub fn metadata(&self) -> &GroupMetadata {
        &self.metadata
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum GroupStatus {
    /// The storages are missing components.
    Incomplete,
    /// The storages contains all components but they are not grouped.
    Ungrouped,
    /// The storages contains all components and they are grouped.
    Grouped,
}

/// # Safety
/// The group family and the storages must be valid.
pub(crate) unsafe fn group_family(
    family: &mut [Group],
    storages: &mut [AtomicRefCell<ComponentStorage>],
    entities: impl Iterator<Item = Entity>,
) {
    entities.for_each(|entity| {
        for group in family.iter_mut() {
            let status = get_group_status(
                storages.get_unchecked_mut(group.metadata.new_storage_range()),
                group.len,
                entity,
            );

            match status {
                GroupStatus::Grouped => (),
                GroupStatus::Ungrouped => {
                    group_components(
                        storages.get_unchecked_mut(group.metadata.storage_range()),
                        &mut group.len,
                        entity,
                    );
                }
                GroupStatus::Incomplete => break,
            }
        }
    });
}

/// # Safety
/// The group family and the storages must be valid.
pub(crate) unsafe fn ungroup_family(
    family: &mut [Group],
    storages: &mut [AtomicRefCell<ComponentStorage>],
    group_mask: GroupMask,
    entities: impl Iterator<Item = Entity>,
) {
    entities.into_iter().for_each(|entity| {
        let mut ungroup_start = 0_usize;
        let mut ungroup_len = 0_usize;

        for (i, group) in family.iter().enumerate() {
            let status = get_group_status(
                storages.get_unchecked_mut(group.metadata.new_storage_range()),
                group.len,
                entity,
            );

            match status {
                GroupStatus::Grouped => {
                    if ungroup_len == 0 {
                        ungroup_start = i;
                    }

                    ungroup_len += 1;
                }
                GroupStatus::Ungrouped | GroupStatus::Incomplete => break,
            }
        }

        let ungroup_range = ungroup_start..(ungroup_start + ungroup_len);

        for i in ungroup_range.rev().take_while(|i| (group_mask & (1 << i)) != 0) {
            let group = family.get_unchecked_mut(i);
            let storages = storages.get_unchecked_mut(group.metadata.storage_range());
            ungroup_components(storages, &mut group.len, entity);
        }
    });
}

/// # Safety
/// The storage slice must be non-empty.
unsafe fn get_group_status(
    storages: &mut [AtomicRefCell<ComponentStorage>],
    group_len: usize,
    entity: Entity,
) -> GroupStatus {
    let (first, others) = storages.split_first_mut().unwrap_unchecked();

    let status = match first.get_mut().get_index(entity) {
        Some(index) => {
            if index < group_len {
                GroupStatus::Grouped
            } else {
                GroupStatus::Ungrouped
            }
        }
        None => return GroupStatus::Incomplete,
    };

    if others.iter_mut().all(|storage| storage.get_mut().contains(entity)) {
        status
    } else {
        GroupStatus::Incomplete
    }
}

/// # Safety
/// The components of the given entity must be ungrouped and the storages and length of the group
/// must be valid.
unsafe fn group_components(
    storages: &mut [AtomicRefCell<ComponentStorage>],
    group_len: &mut usize,
    entity: Entity,
) {
    let swap_index = *group_len;

    for storage in storages.iter_mut().map(AtomicRefCell::get_mut) {
        let index = storage.get_index(entity).unwrap_unchecked();
        storage.swap_unchecked(index, swap_index);
    }

    *group_len += 1;
}

/// # Safety
/// The components of the given entity must be grouped and the storages and length of the group must
/// be valid.
unsafe fn ungroup_components(
    storages: &mut [AtomicRefCell<ComponentStorage>],
    group_len: &mut usize,
    entity: Entity,
) {
    *group_len -= 1;

    for storage in storages.iter_mut().map(AtomicRefCell::get_mut) {
        let index = storage.get_index(entity).unwrap_unchecked();
        storage.swap_unchecked(index, *group_len);
    }
}