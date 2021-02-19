use crate::data::{AtomicRef, AtomicRefMut, Component, Entity, SparseSetMutPtr};
use crate::query::{Comp, CompMut};
use crate::world::{
    ComponentSet, Entities, GroupedComponents, UngroupedComponents, WorldLayoutDescriptor,
};
use std::any::TypeId;
use std::collections::HashSet;

pub struct World {
    entities: Entities,
    grouped_components: GroupedComponents,
    ungrouped_components: UngroupedComponents,
    group_indexes: HashSet<usize>,
}

impl World {
    pub fn new<L>() -> Self
    where
        L: WorldLayoutDescriptor,
    {
        Self {
            entities: Entities::default(),
            grouped_components: GroupedComponents::new(&L::world_layout()),
            ungrouped_components: UngroupedComponents::default(),
            group_indexes: HashSet::default(),
        }
    }

    pub fn register<T>(&mut self)
    where
        T: Component,
    {
        if !self.grouped_components.contains(&TypeId::of::<T>()) {
            self.ungrouped_components.register::<T>();
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
        let _ = self.insert(entity, components);
        entity
    }

    pub fn destroy(&mut self, entity: Entity) -> bool {
        if !self.entities.contains(entity) {
            return false;
        }

        for i in 0..self.grouped_components.group_count() {
            unsafe {
                self.grouped_components.ungroup_components(i, entity);
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

        true
    }

    pub fn insert<C>(&mut self, entity: Entity, components: C) -> Result<(), ()>
    where
        C: ComponentSet,
    {
        if !self.entities.contains(entity) {
            return Err(());
        }

        unsafe {
            let mut storages = C::get_storages(self);
            C::insert(&mut storages, entity, components);
        }

        self.update_group_indexes(C::components().as_ref());

        for &i in self.group_indexes.iter() {
            unsafe {
                self.grouped_components.group_components(i, entity);
            }
        }

        Ok(())
    }

    pub fn remove<C>(&mut self, entity: Entity) -> Option<C>
    where
        C: ComponentSet,
    {
        if !self.entities.contains(entity) {
            return None;
        }

        self.update_group_indexes(C::components().as_ref());

        for &i in self.group_indexes.iter() {
            unsafe {
                self.grouped_components.ungroup_components(i, entity);
            }
        }

        unsafe {
            let mut storages = C::get_storages(self);
            C::remove(&mut storages, entity)
        }
    }

    pub fn delete<C>(&mut self, entity: Entity)
    where
        C: ComponentSet,
    {
        if !self.entities.contains(entity) {
            return;
        }

        self.update_group_indexes(C::components().as_ref());

        for &i in self.group_indexes.iter() {
            unsafe {
                self.grouped_components.ungroup_components(i, entity);
            }
        }

        unsafe {
            let mut storages = C::get_storages(self);
            C::delete(&mut storages, entity);
        }
    }

    pub fn clear(&mut self) {
        self.entities.clear();
        self.grouped_components.clear();
        self.ungrouped_components.clear();
    }

    pub fn entities(&self) -> &Entities {
        &self.entities
    }

    pub fn borrow_comp<T>(&self) -> Option<Comp<T>>
    where
        T: Component,
    {
        match self.grouped_components.borrow(&TypeId::of::<T>()) {
            Some(sparse_set) => unsafe {
                Some(Comp::new(
                    AtomicRef::map_into(sparse_set, |sparse_set| sparse_set.to_ref()),
                    self.grouped_components
                        .get_group_len_ref(&TypeId::of::<T>()),
                ))
            },
            None => match self.ungrouped_components.borrow(&TypeId::of::<T>()) {
                Some(sparse_set) => unsafe {
                    Some(Comp::new(
                        AtomicRef::map_into(sparse_set, |sparse_set| sparse_set.to_ref()),
                        None,
                    ))
                },
                None => None,
            },
        }
    }

    pub fn borrow_comp_mut<T>(&self) -> Option<CompMut<T>>
    where
        T: Component,
    {
        match unsafe { self.grouped_components.borrow_mut(&TypeId::of::<T>()) } {
            Some(sparse_set) => unsafe {
                Some(CompMut::new(
                    AtomicRefMut::map_into(sparse_set, |sparse_set| sparse_set.to_ref_mut()),
                    self.grouped_components
                        .get_group_len_ref(&TypeId::of::<T>()),
                ))
            },
            None => match self.ungrouped_components.borrow_mut(&TypeId::of::<T>()) {
                Some(sparse_set) => unsafe {
                    Some(CompMut::new(
                        AtomicRefMut::map_into(sparse_set, |sparse_set| sparse_set.to_ref_mut()),
                        None,
                    ))
                },
                None => None,
            },
        }
    }

    pub(crate) unsafe fn get_sparse_set_mut_ptr<T>(&self) -> Option<SparseSetMutPtr<T>>
    where
        T: Component,
    {
        match self.ungrouped_components.borrow_mut(&TypeId::of::<T>()) {
            Some(mut sparse_set) => Some(sparse_set.to_mut_ptr::<T>()),
            None => Some(
                self.grouped_components
                    .borrow_mut(&TypeId::of::<T>())?
                    .to_mut_ptr::<T>(),
            ),
        }
    }

    fn update_group_indexes(&mut self, type_ids: &[TypeId]) {
        let grouped_components = &self.grouped_components;

        self.group_indexes.clear();
        self.group_indexes.extend(
            type_ids
                .iter()
                .flat_map(|type_id| grouped_components.get_group_index(type_id)),
        );
    }
}
