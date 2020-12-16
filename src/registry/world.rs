use crate::{
    entity::Entity,
    group::WorldLayoutDescriptor,
    registry::*,
    storage::{AbstractStorage, EntityStorage, SparseSet},
};
use atomic_refcell::{AtomicRef, AtomicRefMut};
use std::{
    collections::HashSet,
    hint::unreachable_unchecked,
    ops::{Deref, DerefMut},
};

pub struct World {
    entities: EntityStorage,
    storages: Storages,
    pub groups: Groups,
}

impl World {
    pub fn new<L>() -> Self
    where
        L: WorldLayoutDescriptor,
    {
        Self {
            entities: Default::default(),
            storages: Default::default(),
            groups: Groups::new(L::world_layout()),
        }
    }

    pub fn register<T>(&mut self)
    where
        T: Component,
    {
        self.storages.register::<T>()
    }

    pub fn borrow<T>(&self) -> Option<AtomicRef<SparseSet<T>>>
    where
        T: Component,
    {
        self.storages.borrow::<T>()
    }

    pub fn borrow_mut<T>(&self) -> Option<AtomicRefMut<SparseSet<T>>>
    where
        T: Component,
    {
        self.storages.borrow_mut::<T>()
    }

    pub(crate) fn borrow_raw_mut<T>(&self) -> Option<AtomicRefMut<SparseSet<T>>>
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

        let mut storages = Vec::new();

        for group_data in group_indexes
            .iter()
            .map(|&i| unsafe { self.groups.get_mut_unchecked(i) })
        {
            storages.extend(
                group_data
                    .components()
                    .iter()
                    .map(|&c| self.storages.borrow_raw_mut(c).unwrap()),
            );

            let (_, subgroup_ends, subgroup_lengths) = group_data.split();
            let mut previous_end = 0_usize;

            for (&group_end, group_len) in subgroup_ends.iter().zip(subgroup_lengths.iter_mut()) {
                let status =
                    insert_group_status(&storages[previous_end..group_end], *group_len, entity);

                match status {
                    InsertGroupStatus::Grouped => {}
                    InsertGroupStatus::NeedsGrouping => unsafe {
                        group_components(&mut storages[..group_end], group_len, entity);
                    },
                    InsertGroupStatus::MissingComponents => break,
                }

                previous_end = group_end;
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

        for group_data in group_indexes
            .iter()
            .map(|&i| unsafe { self.groups.get_mut_unchecked(i) })
        {
            let mut storages = group_data
                .components()
                .iter()
                .map(|&c| self.storages.borrow_raw_mut(c).unwrap())
                .collect::<Vec<_>>();

            let (_, subgroup_ends, subgroup_lengths) = group_data.split();
            let mut previous_end = 0_usize;

            let mut ungroup_start = Option::<usize>::None;
            let mut ungroup_length = 0;

            for (i, (&group_end, group_len)) in subgroup_ends
                .iter()
                .zip(subgroup_lengths.iter_mut())
                .enumerate()
            {
                let status =
                    remove_group_status(&storages[previous_end..group_end], *group_len, entity);

                match status {
                    RemoveGroupStatus::Ungrouped => break,
                    RemoveGroupStatus::NeedsUngrouping => {
                        if ungroup_start.is_none() {
                            ungroup_start = Some(i);
                        }

                        ungroup_length += 1;
                    }
                    RemoveGroupStatus::MissingComponents => break,
                }

                previous_end = group_end;
            }

            if let Some(ungroup_start) = ungroup_start {
                let subgroup_ends = &subgroup_ends[ungroup_start..(ungroup_start + ungroup_length)];
                let subgroup_lengths =
                    &mut subgroup_lengths[ungroup_start..(ungroup_start + ungroup_length)];

                for (&group_end, group_len) in
                    subgroup_ends.iter().zip(subgroup_lengths.iter_mut()).rev()
                {
                    unsafe {
                        ungroup_components(&mut storages[..group_end], group_len, entity);
                    }
                }
            }
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

fn insert_group_status<S>(storages: &[S], group_len: usize, entity: Entity) -> InsertGroupStatus
where
    S: Deref<Target = dyn AbstractStorage>,
{
    let mut status = InsertGroupStatus::Grouped;

    for storage in storages.iter().map(|s| s.deref()) {
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

fn remove_group_status<S>(storages: &[S], group_len: usize, entity: Entity) -> RemoveGroupStatus
where
    S: Deref<Target = dyn AbstractStorage>,
{
    let mut status = RemoveGroupStatus::Ungrouped;

    for storage in storages.iter().map(|s| s.deref()) {
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

unsafe fn group_components<S>(storages: &mut [S], group_len: &mut usize, entity: Entity)
where
    S: DerefMut<Target = dyn AbstractStorage>,
{
    for storage in storages.iter_mut().map(|s| s.deref_mut()) {
        let index = match storage.get_index_entity(entity) {
            Some(index_entity) => index_entity.index(),
            None => unreachable_unchecked(),
        };

        storage.swap(index, *group_len);
    }

    *group_len += 1;
}

unsafe fn ungroup_components<S>(storages: &mut [S], group_len: &mut usize, entity: Entity)
where
    S: DerefMut<Target = dyn AbstractStorage>,
{
    if *group_len > 0 {
        let last_index = *group_len - 1;

        for storage in storages.iter_mut().map(|s| s.deref_mut()) {
            let index = match storage.get_index_entity(entity) {
                Some(index_entity) => index_entity.index(),
                None => unreachable_unchecked(),
            };

            storage.swap(index, last_index);
        }

        *group_len -= 1;
    }
}
