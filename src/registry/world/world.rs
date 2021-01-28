use crate::group::{Group, WorldLayoutDescriptor};
use crate::registry::world::group;
use crate::registry::world::group::GroupStatus;
use crate::registry::{
    Comp, CompMut, Component, ComponentSet, GroupedComponents, UngroupedComponents,
};
use crate::storage::{Entity, EntityStorage, SparseSet};
use atomic_refcell::AtomicRefMut;
use std::any::TypeId;
use std::collections::HashSet;
use std::num::NonZeroU64;
use std::sync::atomic::{AtomicU64, Ordering};

static CURRENT_WORLD_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct WorldId(NonZeroU64);

pub struct World {
    entities: EntityStorage,
    components: UngroupedComponents,
    grouped_components: GroupedComponents,
    id: WorldId,
}

impl World {
    pub fn new<L>() -> Self
    where
        L: WorldLayoutDescriptor,
    {
        Self {
            entities: Default::default(),
            components: Default::default(),
            grouped_components: GroupedComponents::new(L::world_layout()),
            id: WorldId(NonZeroU64::new(CURRENT_WORLD_ID.fetch_add(1, Ordering::Relaxed)).unwrap()),
        }
    }

    pub fn id(&self) -> WorldId {
        self.id
    }

    pub fn register<C>(&mut self)
    where
        C: Component,
    {
        if !self.grouped_components.contains(TypeId::of::<C>()) {
            self.components.register::<C>();
        }
    }

    pub fn maintain(&mut self) {
        self.entities.maintain();
        self.components.maintain();
        self.grouped_components.maintain();
    }

    pub(crate) fn borrow_comp<T>(&self) -> Option<Comp<T>>
    where
        T: Component,
    {
        match self.grouped_components.borrow::<T>() {
            Some(set) => unsafe {
                Some(Comp::grouped(
                    set,
                    Group::new(
                        self.id,
                        self.grouped_components
                            .get_group_info(TypeId::of::<T>())
                            .unwrap(),
                    ),
                ))
            },
            None => Some(Comp::ungrouped(self.components.borrow::<T>()?)),
        }
    }

    pub(crate) fn borrow_comp_mut<T>(&self) -> Option<CompMut<T>>
    where
        T: Component,
    {
        match self.grouped_components.borrow_mut::<T>() {
            Some(set) => unsafe {
                Some(CompMut::grouped(
                    set,
                    Group::new(
                        self.id,
                        self.grouped_components
                            .get_group_info(TypeId::of::<T>())
                            .unwrap(),
                    ),
                ))
            },
            None => Some(CompMut::ungrouped(self.components.borrow_mut::<T>()?)),
        }
    }

    pub(crate) unsafe fn borrow_sparse_set_mut<T>(&self) -> Option<AtomicRefMut<SparseSet<T>>>
    where
        T: Component,
    {
        match self.grouped_components.borrow_mut::<T>() {
            Some(set) => Some(set),
            None => Some(self.components.borrow_mut::<T>()?),
        }
    }

    pub fn create<C>(&mut self, components: C) -> Entity
    where
        C: ComponentSet,
    {
        let entity = self.entities.create();
        self.insert(entity, components);
        entity
    }

    pub fn destroy(&mut self, entity: Entity) -> bool {
        if self.entities.destroy(entity) {
            self.components.remove(entity);
            self.grouped_components.remove(entity);
            true
        } else {
            false
        }
    }

    pub fn insert<C>(&mut self, entity: Entity, components: C) -> bool
    where
        C: ComponentSet,
    {
        if !self.entities.contains(entity) {
            return false;
        }

        unsafe {
            C::insert_raw(self, entity, components);
        }

        let group_indexes = unsafe { C::components() }
            .as_ref()
            .iter()
            .flat_map(|&c| self.grouped_components.get_group_index(c))
            .collect::<HashSet<_>>();

        for &i in group_indexes.iter() {
            let (mut sparse_sets, mut subgroups) = unsafe {
                self.grouped_components
                    .get_component_group_split_view_mut_unchecked(i)
            };

            let mut previous_arity = 0_usize;

            for (arity, len) in unsafe { subgroups.iter_split_subgroups_mut(..) } {
                let status = group::get_group_status(
                    sparse_sets.iter_abstract_sparse_set_views(previous_arity..arity),
                    *len,
                    entity,
                );

                match status {
                    GroupStatus::Grouped => (),
                    GroupStatus::Ungrouped => unsafe {
                        group::group_components(
                            sparse_sets.iter_abstract_sparse_set_views_mut(..arity),
                            len,
                            entity,
                        );
                    },
                    GroupStatus::MissingComponents => break,
                }

                previous_arity = arity;
            }
        }

        true
    }

    pub fn remove<C>(&mut self, entity: Entity) -> Option<C>
    where
        C: ComponentSet,
    {
        if !self.entities.contains(entity) {
            return None;
        }

        let group_indexes = unsafe { C::components() }
            .as_ref()
            .iter()
            .flat_map(|&c| self.grouped_components.get_group_index(c))
            .collect::<HashSet<_>>();

        for &i in group_indexes.iter() {
            let (mut sparse_sets, mut subgroups) = unsafe {
                self.grouped_components
                    .get_component_group_split_view_mut_unchecked(i)
            };

            let mut previous_arity = 0_usize;
            let mut ungroup_start = Option::<usize>::None;
            let mut ungroup_len = 0;

            for (i, (arity, len)) in unsafe { subgroups.iter_split_subgroups_mut(..).enumerate() } {
                let status = group::get_group_status(
                    sparse_sets.iter_abstract_sparse_set_views(previous_arity..arity),
                    *len,
                    entity,
                );

                match status {
                    GroupStatus::Grouped => {
                        if ungroup_start.is_none() {
                            ungroup_start = Some(i);
                        }
                        ungroup_len += 1;
                    }
                    GroupStatus::Ungrouped => break,
                    GroupStatus::MissingComponents => break,
                }

                previous_arity = arity;
            }

            if let Some(ungroup_start) = ungroup_start {
                let ungroup_range = ungroup_start..(ungroup_start + ungroup_len);

                unsafe {
                    for (arity, len) in subgroups.iter_split_subgroups_mut(ungroup_range).rev() {
                        group::ungroup_components(
                            sparse_sets.iter_abstract_sparse_set_views_mut(..arity),
                            len,
                            entity,
                        );
                    }
                }
            }
        }

        unsafe { C::remove_raw(self, entity) }
    }
}
