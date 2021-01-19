use crate::storage::{AbstractSparseSetView, AbstractSparseSetViewMut, Entity};
use std::hint::unreachable_unchecked;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GroupStatus {
    MissingComponents,
    Ungrouped,
    Grouped,
}

pub fn get_group_status<'a, I>(mut sets_iter: I, group_len: usize, entity: Entity) -> GroupStatus
where
    I: Iterator<Item = AbstractSparseSetView<'a>>,
{
    let status = match sets_iter.next() {
        Some(set) => match set.get_index_entity(entity) {
            Some(index_entity) => {
                if index_entity.index() < group_len {
                    GroupStatus::Grouped
                } else {
                    GroupStatus::Ungrouped
                }
            }
            None => return GroupStatus::MissingComponents,
        },
        None => return GroupStatus::Grouped,
    };

    while let Some(set) = sets_iter.next() {
        if !set.contains(entity) {
            return GroupStatus::MissingComponents;
        }
    }

    status
}

pub unsafe fn group_components<'a, I>(sets_iter: I, group_len: &mut usize, entity: Entity)
where
    I: Iterator<Item = AbstractSparseSetViewMut<'a>>,
{
    for mut set in sets_iter {
        let index = match set.get_index_entity(entity) {
            Some(index_entity) => index_entity.index(),
            None => unreachable_unchecked(),
        };

        set.swap(index, *group_len);
    }

    *group_len += 1;
}

pub unsafe fn ungroup_components<'a, I>(sets_iter: I, group_len: &mut usize, entity: Entity)
where
    I: Iterator<Item = AbstractSparseSetViewMut<'a>>,
{
    if *group_len > 0 {
        let last_index = *group_len - 1;

        for mut set in sets_iter {
            let index = match set.get_index_entity(entity) {
                Some(index_entity) => index_entity.index(),
                None => unreachable_unchecked(),
            };

            set.swap(index, last_index);
        }

        *group_len -= 1;
    }
}
