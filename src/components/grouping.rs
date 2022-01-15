use crate::storage::{ComponentStorage, Entity};
use atomic_refcell::AtomicRefCell;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub(crate) enum GroupStatus {
    MissingComponents,
    Ungrouped,
    Grouped,
}

/// The storage slice must be non-empty.
pub(crate) unsafe fn get_group_status(
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
        None => return GroupStatus::MissingComponents,
    };

    if others.iter_mut().all(|storage| storage.get_mut().contains(entity)) {
        status
    } else {
        GroupStatus::MissingComponents
    }
}

/// Must only be called with ungrouped components.
pub(crate) unsafe fn group_components(
    storages: &mut [AtomicRefCell<ComponentStorage>],
    group_len: &mut usize,
    entity: Entity,
) {
    let swap_index = *group_len;

    for storage in storages.iter_mut().map(|storage| storage.get_mut()) {
        let index = storage.get_index(entity).unwrap_unchecked();
        storage.swap_unchecked(index, swap_index);
    }

    *group_len += 1;
}

/// Must only be called with grouped components.
pub(crate) unsafe fn ungroup_components(
    storages: &mut [AtomicRefCell<ComponentStorage>],
    group_len: &mut usize,
    entity: Entity,
) {
    if *group_len > 0 {
        let swap_index = *group_len - 1;

        for storage in storages.iter_mut().map(|storage| storage.get_mut()) {
            let index = storage.get_index(entity).unwrap_unchecked();
            storage.swap_unchecked(index, swap_index);
        }

        *group_len -= 1;
    }
}
