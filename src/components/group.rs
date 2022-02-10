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
/// For the group 1, we get such a `Group` struct:
///
/// ```text
/// storages: A B C D
///           ^   ^   ^
///           |   |   +-- end = begin + 4
///           |   +------ new_begin = begin + 2
///           +---------- begin
/// ```
#[derive(Clone, Copy)]
pub(crate) struct Group {
    begin: usize,
    new_begin: usize,
    end: usize,
    /// Number of entities grouped by this group. Components of grouped entities
    /// are aligned to the left in the storage.
    len: usize,
}

impl Group {
    pub fn new(begin: usize, new_begin: usize, end: usize) -> Self {
        Self { begin, new_begin, end, len: 0 }
    }

    pub fn storage_range(&self) -> Range<usize> {
        self.begin..self.end
    }

    pub fn new_storage_range(&self) -> Range<usize> {
        self.new_begin..self.end
    }

    pub fn include_mask(&self) -> QueryMask {
        QueryMask::new_include_group(self.end - self.begin)
    }

    pub fn exclude_mask(&self) -> QueryMask {
        QueryMask::new_exclude_group(self.new_begin - self.begin, self.end - self.begin)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum GroupStatus {
    Incomplete,
    Ungrouped,
    Grouped,
}

pub(crate) unsafe fn group_family(
    family: &mut [Group],
    storages: &mut [AtomicRefCell<ComponentStorage>],
    entities: impl Iterator<Item = Entity>,
) {
    entities.for_each(|entity| {
        for group in family.iter_mut() {
            let status = get_group_status(
                storages.get_unchecked_mut(group.new_storage_range()),
                group.len,
                entity,
            );

            match status {
                GroupStatus::Grouped => (),
                GroupStatus::Ungrouped => {
                    group_components(
                        storages.get_unchecked_mut(group.storage_range()),
                        &mut group.len,
                        entity,
                    );
                }
                GroupStatus::Incomplete => break,
            }
        }
    });
}

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
                storages.get_unchecked_mut(group.new_storage_range()),
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
            let storages = storages.get_unchecked_mut(group.storage_range());
            ungroup_components(storages, &mut group.len, entity);
        }
    });
}

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

/// Must only be called with ungrouped components.
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

/// Must only be called with grouped components.
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
