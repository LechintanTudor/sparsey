use crate::data::{AtomicRef, AtomicRefCell, AtomicRefMut, Entity, TypeErasedSparseSet};
use crate::world::WorldLayout;
use std::any::TypeId;
use std::collections::HashMap;
use std::hint::unreachable_unchecked;

pub(crate) struct GroupedComponents {
    groups: Vec<Group>,
    info: HashMap<TypeId, ComponentInfo>,
}

impl GroupedComponents {
    pub fn new(world_layout: &WorldLayout) -> Self {
        let mut groups = Vec::<Group>::new();
        let mut info = HashMap::<TypeId, ComponentInfo>::new();

        for group_layout in world_layout.group_layouts() {
            let mut sparse_sets = Vec::<AtomicRefCell<TypeErasedSparseSet>>::new();
            let mut subgroups = Vec::<Subgroup>::new();

            let components = group_layout.components();
            let mut previous_arity = 0_usize;

            for (subgroup_index, &arity) in group_layout.subgroup_arities().iter().enumerate() {
                for component in &components[previous_arity..arity] {
                    info.insert(
                        component.component_type_id(),
                        ComponentInfo {
                            group_index: groups.len(),
                            sparse_set_index: sparse_sets.len(),
                            subgroup_index,
                        },
                    );

                    sparse_sets.push(AtomicRefCell::new(component.create_sparse_set()));
                }

                subgroups.push(Subgroup::with_arity(arity));
                previous_arity = arity;
            }

            groups.push(Group {
                sparse_sets,
                subgroups,
            });
        }

        Self { groups, info }
    }

    pub fn clear(&mut self) {
        for group in self.groups.iter_mut() {
            for sparse_set in group.sparse_sets.iter_mut() {
                sparse_set.get_mut().clear();
            }

            for subgroup in group.subgroups.iter_mut() {
                subgroup.len = 0;
            }
        }
    }

    pub fn contains(&self, type_id: &TypeId) -> bool {
        self.info.contains_key(type_id)
    }

    pub unsafe fn group_components(&mut self, group_index: usize, entity: Entity) {
        let (sparse_sets, subgroups) = {
            let group = &mut self.groups[group_index];
            (
                group.sparse_sets.as_mut_slice(),
                group.subgroups.as_mut_slice(),
            )
        };

        let mut previous_arity = 0_usize;

        for subgroup in subgroups.iter_mut() {
            let status = get_group_status(
                &mut sparse_sets[previous_arity..subgroup.arity],
                subgroup.len,
                entity,
            );

            match status {
                GroupStatus::Grouped => (),
                GroupStatus::Ungrouped => {
                    group_components(
                        &mut sparse_sets[..subgroup.arity],
                        &mut subgroup.len,
                        entity,
                    );
                }
                GroupStatus::MissingComponents => break,
            }

            previous_arity = subgroup.arity;
        }
    }

    pub unsafe fn ungroup_components(&mut self, group_index: usize, entity: Entity) {
        let (sparse_sets, subgroups) = {
            let group = &mut self.groups[group_index];
            (
                group.sparse_sets.as_mut_slice(),
                group.subgroups.as_mut_slice(),
            )
        };

        let mut previous_arity = 0_usize;
        let mut ungroup_start = 0_usize;
        let mut ungroup_len = 0_usize;

        for (i, subgroup) in subgroups.iter_mut().enumerate() {
            let status = get_group_status(
                &mut sparse_sets[previous_arity..subgroup.arity],
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

        for subgroup in (&mut subgroups[ungroup_range]).iter_mut().rev() {
            ungroup_components(
                &mut sparse_sets[..subgroup.arity],
                &mut subgroup.len,
                entity,
            );
        }
    }

    pub fn group_count(&self) -> usize {
        self.groups.len()
    }

    pub fn get_group_index(&self, type_id: &TypeId) -> Option<usize> {
        self.info.get(type_id).map(|info| info.group_index)
    }

    pub fn get_group_len_ref(&self, type_id: &TypeId) -> Option<&usize> {
        self.info.get(type_id).map(|info| unsafe {
            &self
                .groups
                .get_unchecked(info.group_index)
                .subgroups
                .get_unchecked(info.subgroup_index)
                .len
        })
    }

    pub fn borrow(&self, type_id: &TypeId) -> Option<AtomicRef<TypeErasedSparseSet>> {
        self.info.get(type_id).map(|info| unsafe {
            self.groups
                .get_unchecked(info.group_index)
                .sparse_sets
                .get_unchecked(info.sparse_set_index)
                .borrow()
        })
    }

    pub unsafe fn borrow_mut(&self, type_id: &TypeId) -> Option<AtomicRefMut<TypeErasedSparseSet>> {
        self.info.get(type_id).map(|info| {
            self.groups
                .get_unchecked(info.group_index)
                .sparse_sets
                .get_unchecked(info.sparse_set_index)
                .borrow_mut()
        })
    }

    pub unsafe fn iter_sparse_sets_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut TypeErasedSparseSet> {
        self.groups.iter_mut().flat_map(|group| {
            group
                .sparse_sets
                .iter_mut()
                .map(|sparse_set| sparse_set.get_mut())
        })
    }
}

struct Group {
    sparse_sets: Vec<AtomicRefCell<TypeErasedSparseSet>>,
    subgroups: Vec<Subgroup>,
}

#[derive(Copy, Clone)]
struct Subgroup {
    arity: usize,
    len: usize,
}

impl Subgroup {
    fn with_arity(arity: usize) -> Self {
        Self { arity, len: 0 }
    }
}

#[derive(Copy, Clone)]
struct ComponentInfo {
    group_index: usize,
    sparse_set_index: usize,
    subgroup_index: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum GroupStatus {
    MissingComponents,
    Ungrouped,
    Grouped,
}

fn get_group_status(
    sparse_sets: &mut [AtomicRefCell<TypeErasedSparseSet>],
    group_len: usize,
    entity: Entity,
) -> GroupStatus {
    match sparse_sets.split_first_mut() {
        Some((first, others)) => {
            let status = match first.get_mut().get_index_entity(entity) {
                Some(index_entity) => {
                    if index_entity.index() < group_len {
                        GroupStatus::Grouped
                    } else {
                        GroupStatus::Ungrouped
                    }
                }
                None => return GroupStatus::MissingComponents,
            };

            if others
                .iter_mut()
                .all(|sparse_set| sparse_set.get_mut().contains(entity))
            {
                status
            } else {
                GroupStatus::MissingComponents
            }
        }
        None => GroupStatus::Grouped,
    }
}

unsafe fn group_components(
    sparse_sets: &mut [AtomicRefCell<TypeErasedSparseSet>],
    group_len: &mut usize,
    entity: Entity,
) {
    for sparse_set in sparse_sets
        .iter_mut()
        .map(|sparse_set| sparse_set.get_mut())
    {
        let index = match sparse_set.get_index_entity(entity) {
            Some(index_entity) => index_entity.index(),
            None => unreachable_unchecked(),
        };

        sparse_set.swap(index, *group_len);
    }

    *group_len += 1;
}

unsafe fn ungroup_components(
    sparse_sets: &mut [AtomicRefCell<TypeErasedSparseSet>],
    group_len: &mut usize,
    entity: Entity,
) {
    if *group_len > 0 {
        let last_index = *group_len - 1;

        for sparse_set in sparse_sets
            .iter_mut()
            .map(|sparse_set| sparse_set.get_mut())
        {
            let index = match sparse_set.get_index_entity(entity) {
                Some(index_entity) => index_entity.index(),
                None => unreachable_unchecked(),
            };

            sparse_set.swap(index, last_index);
        }

        *group_len -= 1;
    }
}
