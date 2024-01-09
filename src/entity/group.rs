use crate::entity::{ComponentSparseSet, Entity, GroupMask};
use atomic_refcell::AtomicRefCell;
use std::ops::Range;

#[derive(Clone, Copy, Debug)]
pub struct Group {
    pub metadata: GroupMetadata,
    pub len: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct GroupMetadata {
    pub storage_start: usize,
    pub new_storage_start: usize,
    pub storage_end: usize,
    pub skip_mask: GroupMask,
}

impl GroupMetadata {
    #[inline]
    #[must_use]
    pub fn storage_range(&self) -> Range<usize> {
        self.storage_start..self.storage_end
    }

    #[inline]
    #[must_use]
    pub fn new_storage_range(&self) -> Range<usize> {
        self.new_storage_start..self.storage_end
    }
}

pub(crate) unsafe fn group(
    components: &mut [AtomicRefCell<ComponentSparseSet>],
    groups: &mut [Group],
    group_mask: GroupMask,
    entity: Entity,
) {
    let mut group_index_iter = group_mask.iter_bit_indexes();

    while let Some(group_index) = group_index_iter.next() {
        let group = groups.get_unchecked_mut(group_index as usize);

        let status = get_group_status(
            &mut components[group.metadata.new_storage_range()],
            group.len,
            entity,
        );

        match status {
            GroupStatus::Incomplete => {
                group_index_iter.0 &= group.metadata.skip_mask.0;
            }
            GroupStatus::Ungrouped => {
                group_components(
                    &mut components[group.metadata.storage_range()],
                    &mut group.len,
                    entity,
                );
            }
            GroupStatus::Grouped => (),
        }
    }
}

pub(crate) unsafe fn ungroup(
    components: &mut [AtomicRefCell<ComponentSparseSet>],
    groups: &mut [Group],
    group_mask: GroupMask,
    entity: Entity,
) {
    for group_index in group_mask.iter_bit_indexes().rev() {
        let group = groups.get_unchecked_mut(group_index as usize);

        let status = get_group_status(
            &mut components[group.metadata.new_storage_range()],
            group.len,
            entity,
        );

        if status == GroupStatus::Grouped {
            ungroup_components(
                &mut components[group.metadata.storage_range()],
                &mut group.len,
                entity,
            );
        }
    }
}

pub(crate) unsafe fn ungroup_all(
    components: &mut [AtomicRefCell<ComponentSparseSet>],
    groups: &mut [Group],
    entity: Entity,
) {
    for group in groups.iter_mut().rev() {
        let status = get_group_status(
            &mut components[group.metadata.new_storage_range()],
            group.len,
            entity,
        );

        if status == GroupStatus::Grouped {
            ungroup_components(
                &mut components[group.metadata.storage_range()],
                &mut group.len,
                entity,
            );
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum GroupStatus {
    Incomplete,
    Ungrouped,
    Grouped,
}

unsafe fn get_group_status(
    components: &mut [AtomicRefCell<ComponentSparseSet>],
    group_len: usize,
    entity: Entity,
) -> GroupStatus {
    let (first, others) = components.split_first_mut().unwrap_unchecked();

    let status = match first.get_mut().sparse().get(entity) {
        Some(dense_entity) => {
            if dense_entity.dense() < group_len {
                GroupStatus::Grouped
            } else {
                GroupStatus::Ungrouped
            }
        }
        None => return GroupStatus::Incomplete,
    };

    let sparse = entity.sparse();

    if others
        .iter_mut()
        .all(|sparse_set| sparse_set.get_mut().sparse().contains_sparse(sparse))
    {
        status
    } else {
        GroupStatus::Incomplete
    }
}

#[inline]
unsafe fn group_components(
    components: &mut [AtomicRefCell<ComponentSparseSet>],
    group_len: &mut usize,
    entity: Entity,
) {
    let swap_index = *group_len;
    let sparse = entity.sparse();

    components
        .iter_mut()
        .map(AtomicRefCell::get_mut)
        .for_each(|sparse_set| {
            let dense = sparse_set.sparse().get_sparse_unchecked(sparse);

            if dense != swap_index {
                sparse_set.swap(dense, swap_index);
            }
        });

    *group_len += 1;
}

#[inline]
unsafe fn ungroup_components(
    components: &mut [AtomicRefCell<ComponentSparseSet>],
    group_len: &mut usize,
    entity: Entity,
) {
    *group_len -= 1;
    let swap_index = *group_len;
    let sparse = entity.sparse();

    components
        .iter_mut()
        .map(AtomicRefCell::get_mut)
        .for_each(|sparse_set| {
            let dense = sparse_set.sparse().get_sparse_unchecked(sparse);

            if dense != swap_index {
                sparse_set.swap(dense, swap_index);
            }
        });
}
