use crate::entity::Entity;
use crate::group::WorldLayoutDescriptor;
use crate::registry::{
    BorrowFromWorld, Comp, CompMut, Component, ComponentSource, Groups, Storages,
};
use crate::storage::{AbstractStorageViewMut, EntityStorage, SparseSet};
use atomic_refcell::AtomicRefMut;
use std::any::TypeId;
use std::collections::HashSet;
use std::hint::unreachable_unchecked;

pub struct World {
    entities: EntityStorage,
    storages: Storages,
    groups: Groups,
}

impl World {
    pub fn new<L>() -> Self
    where
        L: WorldLayoutDescriptor,
    {
        let mut world = Self {
            entities: Default::default(),
            storages: Default::default(),
            groups: Groups::new(L::world_layout()),
        };

        L::register_components(&mut world);
        world
    }

    pub fn register<T>(&mut self)
    where
        T: Component,
    {
        self.storages.register::<T>()
    }

    pub(crate) fn borrow_comp<T>(&self) -> Option<Comp<T>>
    where
        T: Component,
    {
        unsafe {
            Some(Comp::new(
                self.storages.borrow::<T>()?,
                self.groups.get_parent_group(TypeId::of::<T>()),
            ))
        }
    }

    pub(crate) fn borrow_comp_mut<T>(&self) -> Option<CompMut<T>>
    where
        T: Component,
    {
        unsafe {
            Some(CompMut::new(
                self.storages.borrow_mut::<T>()?,
                self.groups.get_parent_group(TypeId::of::<T>()),
            ))
        }
    }

    pub(crate) unsafe fn borrow_sparse_set_mut<T>(&self) -> Option<AtomicRefMut<SparseSet<T>>>
    where
        T: Component,
    {
        self.storages.borrow_mut::<T>()
    }

    pub(crate) fn entities(&self) -> &EntityStorage {
        &self.entities
    }

    pub fn create<'a, C>(&'a mut self, components: C) -> Entity
    where
        C: ComponentSource<'a>,
    {
        let entity = self.entities.create();
        self.insert(entity, components);
        entity
    }

    pub fn insert<'a, C>(&'a mut self, entity: Entity, components: C)
    where
        C: ComponentSource<'a>,
    {
        {
            let mut target = <C::Target as BorrowFromWorld>::borrow(self);
            C::insert(&mut target, entity, components);
        }

        let group_indexes = C::components()
            .as_ref()
            .iter()
            .flat_map(|&c| self.groups.get_subgroup_index(c))
            .map(|c| c.group_index())
            .collect::<HashSet<_>>();

        let mut storages = Vec::<AbstractStorageViewMut>::new();

        for group in group_indexes
            .iter()
            .map(|&i| unsafe { self.groups.get_mut_unchecked(i) })
        {
            storages.extend(group.components().iter().map(|&c| unsafe {
                self.storages
                    .get_abstract_mut_unchecked(c)
                    .unwrap()
                    .as_storage_view_mut()
            }));

            let mut previous_arity = 0_usize;

            for (arity, len) in group.iter_subgroups_mut(..) {
                let status = group_insert_status(&storages[previous_arity..arity], *len, entity);

                match status {
                    InsertGroupStatus::NeedsGrouping => unsafe {
                        group_components(&mut storages[..arity], len, entity);
                    },
                    InsertGroupStatus::MissingComponents => break,
                    InsertGroupStatus::Grouped => (),
                }

                previous_arity = arity;
            }

            storages.clear()
        }
    }

    pub fn remove<'a, C>(&'a mut self, entity: Entity) -> Option<C>
    where
        C: ComponentSource<'a>,
    {
        let group_indexes = C::components()
            .as_ref()
            .iter()
            .flat_map(|&c| self.groups.get_subgroup_index(c))
            .map(|c| c.group_index())
            .collect::<HashSet<_>>();

        let mut storages = Vec::<AbstractStorageViewMut>::new();

        for group in group_indexes
            .iter()
            .map(|&i| unsafe { self.groups.get_mut_unchecked(i) })
        {
            storages.extend(group.components().iter().map(|&c| unsafe {
                self.storages
                    .get_abstract_mut_unchecked(c)
                    .unwrap()
                    .as_storage_view_mut()
            }));

            let mut previous_arity = 0_usize;
            let mut ungroup_start = Option::<usize>::None;
            let mut ungroup_len = 0;

            for (i, (arity, len)) in group.iter_subgroups_mut(..).enumerate() {
                let status = group_remove_status(&storages[previous_arity..arity], *len, entity);

                match status {
                    RemoveGroupStatus::NeedsUngrouping => {
                        if ungroup_start.is_none() {
                            ungroup_start = Some(i);
                        }

                        ungroup_len += 1;
                    }
                    RemoveGroupStatus::Ungrouped => break,
                    RemoveGroupStatus::MissingComponents => break,
                }

                previous_arity = arity;
            }

            if let Some(ungroup_start) = ungroup_start {
                let ungroup_range = ungroup_start..(ungroup_start + ungroup_len);

                for (arity, len) in group.iter_subgroups_mut(ungroup_range).rev() {
                    unsafe {
                        ungroup_components(&mut storages[..arity], len, entity);
                    }
                }
            }

            storages.clear();
        }

        let mut target = <C::Target as BorrowFromWorld>::borrow(self);
        C::remove(&mut target, entity)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum InsertGroupStatus {
    Grouped,
    NeedsGrouping,
    MissingComponents,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum RemoveGroupStatus {
    Ungrouped,
    NeedsUngrouping,
    MissingComponents,
}

fn group_insert_status(
    storages: &[AbstractStorageViewMut],
    group_len: usize,
    entity: Entity,
) -> InsertGroupStatus {
    let mut status = InsertGroupStatus::Grouped;

    for storage in storages.iter() {
        match storage.get_index_entity(entity) {
            Some(index_entity) => {
                if index_entity.index() >= group_len {
                    status = InsertGroupStatus::NeedsGrouping;
                }
            }
            None => return InsertGroupStatus::MissingComponents,
        }
    }

    status
}

fn group_remove_status(
    storages: &[AbstractStorageViewMut],
    group_len: usize,
    entity: Entity,
) -> RemoveGroupStatus {
    let mut status = RemoveGroupStatus::Ungrouped;

    for storage in storages.iter() {
        match storage.get_index_entity(entity) {
            Some(index_entity) => {
                if index_entity.index() < group_len {
                    status = RemoveGroupStatus::NeedsUngrouping;
                }
            }
            None => return RemoveGroupStatus::MissingComponents,
        }
    }

    status
}

unsafe fn group_components(
    storages: &mut [AbstractStorageViewMut],
    group_len: &mut usize,
    entity: Entity,
) {
    for storage in storages.iter_mut() {
        let index = match storage.get_index_entity(entity) {
            Some(index_entity) => index_entity.index(),
            None => unreachable_unchecked(),
        };

        storage.swap(index, *group_len);
    }

    *group_len += 1;
}

unsafe fn ungroup_components(
    storages: &mut [AbstractStorageViewMut],
    group_len: &mut usize,
    entity: Entity,
) {
    if *group_len > 0 {
        let last_index = *group_len - 1;

        for storage in storages.iter_mut() {
            let index = match storage.get_index_entity(entity) {
                Some(index_entity) => index_entity.index(),
                None => unreachable_unchecked(),
            };

            storage.swap(index, last_index);
        }

        *group_len -= 1;
    }
}
