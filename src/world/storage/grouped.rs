use crate::data::{GroupInfo, WorldId};
use crate::storage::{AbstractSparseSet, AbstractSparseSetViewMut, Entity};
use crate::world::WorldLayout;
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use std::any::TypeId;
use std::cell::UnsafeCell;
use std::collections::{HashMap, HashSet};
use std::hint::unreachable_unchecked;

pub(crate) struct GroupedComponents {
    groups: Vec<Group>,
    info: HashMap<TypeId, ComponentInfo>,
    world_id: WorldId,
}

unsafe impl Send for GroupedComponents {}
unsafe impl Sync for GroupedComponents {}

impl GroupedComponents {
    pub fn new(world_layout: &WorldLayout) -> Self {
        let mut groups = Vec::<Group>::new();
        let mut info = HashMap::<TypeId, ComponentInfo>::new();

        for group_layout in world_layout.group_layouts() {
            let mut sparse_sets = Vec::<AtomicRefCell<Box<dyn AbstractSparseSet>>>::new();
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
                subgroups: UnsafeCell::new(subgroups),
            });
        }

        Self {
            world_id: WorldId::new(),
            groups,
            info,
        }
    }

    pub fn clear(&mut self) {
        for group in self.groups.iter_mut() {
            for sparse_set in group.sparse_sets.iter_mut() {
                sparse_set.get_mut().clear();
            }

            for subgroup in group.subgroups.get_mut().iter_mut() {
                subgroup.clear();
            }
        }
    }

    pub fn contains(&self, type_id: &TypeId) -> bool {
        self.info.contains_key(type_id)
    }

    pub fn get_group(&self, type_id: &TypeId) -> Option<GroupInfo> {
        self.info.get(type_id).map(|info| unsafe {
            GroupInfo::new(
                self.world_id,
                info.group_index,
                info.subgroup_index,
                self.groups
                    .get_unchecked(info.group_index)
                    .subgroups
                    .get()
                    .as_ref()
                    .unwrap()
                    .get_unchecked(info.subgroup_index)
                    .len(),
            )
        })
    }

    pub fn get_group_index(&self, type_id: &TypeId) -> Option<usize> {
        self.info.get(type_id).map(|info| info.group_index)
    }

    pub unsafe fn get_group_set(&mut self, group_indexes: &HashSet<usize>) -> GroupSetViewMut {
        let mut groups = Vec::<GroupViewMut>::with_capacity(group_indexes.len());

        for &i in group_indexes.iter() {
            groups.push(self.groups[i].to_view_mut());
        }

        GroupSetViewMut::new(groups)
    }

    pub unsafe fn get_full_group_set(&mut self) -> GroupSetViewMut {
        GroupSetViewMut::new(
            self.groups
                .iter()
                .map(|group| group.to_view_mut())
                .collect(),
        )
    }

    pub fn borrow_abstract(&self, type_id: &TypeId) -> Option<AtomicRef<dyn AbstractSparseSet>> {
        self.info.get(type_id).map(|info| unsafe {
            AtomicRef::map(
                self.groups
                    .get_unchecked(info.group_index)
                    .sparse_sets
                    .get_unchecked(info.sparse_set_index)
                    .borrow(),
                |sparse_set| Box::as_ref(sparse_set),
            )
        })
    }

    pub unsafe fn borrow_abstract_mut(
        &self,
        type_id: &TypeId,
    ) -> Option<AtomicRefMut<dyn AbstractSparseSet>> {
        self.info.get(type_id).map(|info| {
            AtomicRefMut::map(
                self.groups
                    .get_unchecked(info.group_index)
                    .sparse_sets
                    .get_unchecked(info.sparse_set_index)
                    .borrow_mut(),
                |sparse_set| Box::as_mut(sparse_set),
            )
        })
    }

    pub unsafe fn iter_sparse_sets_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut dyn AbstractSparseSet> {
        self.groups.iter_mut().flat_map(|group| {
            group
                .sparse_sets
                .iter_mut()
                .map(|sparse_set| Box::as_mut(sparse_set.get_mut()))
        })
    }
}

struct Group {
    sparse_sets: Vec<AtomicRefCell<Box<dyn AbstractSparseSet>>>,
    subgroups: UnsafeCell<Vec<Subgroup>>,
}

impl Group {
    unsafe fn to_view_mut(&self) -> GroupViewMut {
        let sparse_sets = self
            .sparse_sets
            .iter()
            .map(|sparse_set| sparse_set.as_ptr().as_mut().unwrap().as_abstract_view_mut())
            .collect::<Vec<_>>();

        let subgroups = self.subgroups.get().as_mut().unwrap();

        GroupViewMut::new(sparse_sets, subgroups)
    }
}

#[derive(Copy, Clone)]
struct ComponentInfo {
    group_index: usize,
    sparse_set_index: usize,
    subgroup_index: usize,
}

struct GroupViewMut<'a> {
    sparse_sets: Vec<AbstractSparseSetViewMut<'a>>,
    subgroups: &'a mut [Subgroup],
}

impl<'a> GroupViewMut<'a> {
    unsafe fn new(
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
    fn new(groups: Vec<GroupViewMut<'a>>) -> Self {
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
struct Subgroup {
    arity: usize,
    len: usize,
}

impl Subgroup {
    fn with_arity(arity: usize) -> Self {
        Self { arity, len: 0 }
    }

    fn clear(&mut self) {
        self.len = 0;
    }

    fn len(&self) -> usize {
        self.len
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum GroupStatus {
    MissingComponents,
    Ungrouped,
    Grouped,
}

fn get_group_status<'a>(
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

unsafe fn group_components<'a>(
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

unsafe fn ungroup_components<'a>(
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
