use crate::group::WorldLayoutDescriptor;
use crate::registry::{
    Comp, CompMut, Component, ComponentSet, ComponentTypeId, GroupedComponents, UngroupedComponents,
};
use crate::storage::{AbstractSparseSet, Entity, EntityStorage, SparseSet};
use atomic_refcell::{AtomicRef, AtomicRefMut};
use std::collections::HashSet;
use std::hint::unreachable_unchecked;

pub struct World {
    entities: EntityStorage,
    grouped_components: GroupedComponents,
    ungrouped_components: UngroupedComponents,
}

impl World {
    pub fn new<L>() -> Self
    where
        L: WorldLayoutDescriptor,
    {
        Self {
            entities: Default::default(),
            grouped_components: GroupedComponents::new(&L::world_layout()),
            ungrouped_components: Default::default(),
        }
    }

    pub fn register<C>(&mut self)
    where
        C: Component,
    {
        if !self
            .grouped_components
            .contains(&ComponentTypeId::of::<C>())
        {
            self.ungrouped_components.register::<C>();
        }
    }

    pub fn maintain(&mut self) {
        self.entities.maintain();

        unsafe {
            for sparse_set in self.grouped_components.iter_sparse_sets_mut() {
                sparse_set.maintain();
            }

            for sparse_set in self.ungrouped_components.iter_sparse_sets_mut() {
                sparse_set.maintain();
            }
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
        if !self.entities.contains(entity) {
            return false;
        }

        for group_index in 0..self.grouped_components.group_count() {
            unsafe {
                self.grouped_components
                    .ungroup_components(group_index, Some(entity));
            }
        }

        unsafe {
            for sparse_set in self.grouped_components.iter_sparse_sets_mut() {
                sparse_set.delete(entity);
            }

            for sparse_set in self.ungrouped_components.iter_sparse_sets_mut() {
                sparse_set.delete(entity);
            }
        }

        self.entities.destroy(entity);
        true
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

            let group_indexes = C::components()
                .as_ref()
                .iter()
                .flat_map(|type_id| self.grouped_components.get_group_index(type_id))
                .collect::<HashSet<_>>();

            for &group_index in group_indexes.iter() {
                self.grouped_components
                    .group_components(group_index, Some(entity));
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

        unsafe {
            let group_indexes = C::components()
                .as_ref()
                .iter()
                .flat_map(|type_id| self.grouped_components.get_group_index(type_id))
                .collect::<HashSet<_>>();

            for &group_index in group_indexes.iter() {
                self.grouped_components
                    .ungroup_components(group_index, Some(entity));
            }

            C::remove_raw(self, entity)
        }
    }

    pub(crate) fn borrow_comp<T>(&self) -> Option<Comp<T>>
    where
        T: Component,
    {
        let type_id = ComponentTypeId::of::<T>();

        match self.grouped_components.borrow_abstract(&type_id) {
            Some(sparse_set) => unsafe {
                Some(Comp::new(
                    downcast_sparse_set::<T>(sparse_set),
                    self.grouped_components.get_group(&type_id),
                ))
            },
            None => {
                let sparse_set = self.ungrouped_components.borrow_abstract(&type_id)?;

                unsafe { Some(Comp::new(downcast_sparse_set::<T>(sparse_set), None)) }
            }
        }
    }

    pub(crate) fn borrow_comp_mut<T>(&self) -> Option<CompMut<T>>
    where
        T: Component,
    {
        let type_id = ComponentTypeId::of::<T>();

        match unsafe { self.grouped_components.borrow_abstract_mut(&type_id) } {
            Some(sparse_set) => unsafe {
                Some(CompMut::new(
                    downcast_sparse_set_mut::<T>(sparse_set),
                    self.grouped_components.get_group(&type_id),
                ))
            },
            None => {
                let sparse_set = self.ungrouped_components.borrow_abstract_mut(&type_id)?;

                unsafe { Some(CompMut::new(downcast_sparse_set_mut::<T>(sparse_set), None)) }
            }
        }
    }

    pub(crate) unsafe fn borrow_sparse_set_mut<T>(&self) -> Option<AtomicRefMut<SparseSet<T>>>
    where
        T: Component,
    {
        let type_id = ComponentTypeId::of::<T>();
        let sparse_set = self
            .grouped_components
            .borrow_abstract_mut(&type_id)
            .or_else(|| self.ungrouped_components.borrow_abstract_mut(&type_id))?;

        Some(downcast_sparse_set_mut::<T>(sparse_set))
    }
}

unsafe fn downcast_sparse_set<T>(
    sparse_set: AtomicRef<dyn AbstractSparseSet>,
) -> AtomicRef<SparseSet<T>>
where
    T: Component,
{
    AtomicRef::map(sparse_set, |sparse_set| {
        match sparse_set.downcast_ref::<SparseSet<T>>() {
            Some(sparse_set) => sparse_set,
            None => unreachable_unchecked(),
        }
    })
}

unsafe fn downcast_sparse_set_mut<T>(
    sparse_set: AtomicRefMut<dyn AbstractSparseSet>,
) -> AtomicRefMut<SparseSet<T>>
where
    T: Component,
{
    AtomicRefMut::map(sparse_set, |sparse_set| {
        match sparse_set.downcast_mut::<SparseSet<T>>() {
            Some(sparse_set) => sparse_set,
            None => unreachable_unchecked(),
        }
    })
}
