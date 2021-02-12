use crate::storage::{AbstractSparseSetViewMut, Entity};
use std::hint::unreachable_unchecked;

pub struct GroupViewMut<'a> {
    sparse_sets: Vec<AbstractSparseSetViewMut<'a>>,
    subgroups: &'a mut [Subgroup],
}

impl<'a> GroupViewMut<'a> {
    pub unsafe fn new(
        sparse_sets: Vec<AbstractSparseSetViewMut<'a>>,
        subgroups: &'a mut [Subgroup],
    ) -> Self {
        Self {
            sparse_sets,
            subgroups,
        }
    }
}

pub struct GroupSetViewMut<'a> {
    groups: Vec<GroupViewMut<'a>>,
}

impl<'a> GroupSetViewMut<'a> {
    pub fn new(groups: Vec<GroupViewMut<'a>>) -> Self {
        Self { groups }
    }
}

impl<'a> GroupSetViewMut<'a> {
    pub fn group_components(&mut self, entity: Entity) {
        for group in self.groups.iter_mut() {
            let mut previous_arity = 0_usize;

            for subgroup in group.subgroups.iter_mut() {
                let status = get_group_status(
                    &group.sparse_sets[previous_arity..subgroup.arity],
                    subgroup.len,
                    entity,
                );

                match status {
                    GroupStatus::Grouped => (),
                    GroupStatus::Ungrouped => unsafe {
                        group_components(
                            &mut group.sparse_sets[..subgroup.arity],
                            &mut subgroup.len,
                            entity,
                        );
                    },
                    GroupStatus::MissingComponents => break,
                }

                previous_arity = subgroup.arity;
            }
        }
    }

    pub fn ungroup_components(&mut self, entity: Entity) {
        for group in self.groups.iter_mut() {
            let mut previous_arity = 0_usize;
            let mut ungroup_start = 0_usize;
            let mut ungroup_len = 0_usize;

            for (i, subgroup) in group.subgroups.iter_mut().enumerate() {
                let status = get_group_status(
                    &group.sparse_sets[previous_arity..subgroup.arity],
                    subgroup.len,
                    entity,
                );

                match status {
                    GroupStatus::Grouped => {
                        if ungroup_len == 0 {
                            ungroup_start = i;
                        }

                        ungroup_len += 1;
                    }
                    GroupStatus::Ungrouped => break,
                    GroupStatus::MissingComponents => break,
                }

                previous_arity = subgroup.arity;
            }

            let ungroup_range = ungroup_start..(ungroup_start + ungroup_len);

            for subgroup in (&mut group.subgroups[ungroup_range]).iter_mut().rev() {
                unsafe {
                    ungroup_components(
                        &mut group.sparse_sets[..subgroup.arity],
                        &mut subgroup.len,
                        entity,
                    );
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Subgroup {
    arity: usize,
    len: usize,
}

impl Subgroup {
    pub fn with_arity(arity: usize) -> Self {
        Self { arity, len: 0 }
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GroupStatus {
    MissingComponents,
    Ungrouped,
    Grouped,
}

pub fn get_group_status<'a>(
    sparse_sets: &[AbstractSparseSetViewMut<'a>],
    group_len: usize,
    entity: Entity,
) -> GroupStatus {
    match sparse_sets.split_first() {
        Some((first, others)) => {
            let status = match first.get_index_entity(entity) {
                Some(index_entity) => {
                    if index_entity.index() < group_len {
                        GroupStatus::Grouped
                    } else {
                        GroupStatus::Ungrouped
                    }
                }
                None => return GroupStatus::MissingComponents,
            };

            if others.iter().all(|sparse_set| sparse_set.contains(entity)) {
                status
            } else {
                GroupStatus::MissingComponents
            }
        }
        None => GroupStatus::Grouped,
    }
}

pub unsafe fn group_components<'a>(
    sparse_sets: &mut [AbstractSparseSetViewMut<'a>],
    group_len: &mut usize,
    entity: Entity,
) {
    for sparse_set in sparse_sets.iter_mut() {
        let index = match sparse_set.get_index_entity(entity) {
            Some(index_entity) => index_entity.index(),
            None => unreachable_unchecked(),
        };

        sparse_set.swap(index, *group_len);
    }

    *group_len += 1;
}

pub unsafe fn ungroup_components<'a>(
    sparse_sets: &mut [AbstractSparseSetViewMut<'a>],
    group_len: &mut usize,
    entity: Entity,
) {
    if *group_len > 0 {
        let last_index = *group_len - 1;

        for sparse_set in sparse_sets.iter_mut() {
            let index = match sparse_set.get_index_entity(entity) {
                Some(index_entity) => index_entity.index(),
                None => unreachable_unchecked(),
            };

            sparse_set.swap(index, last_index);
        }

        *group_len -= 1;
    }
}
