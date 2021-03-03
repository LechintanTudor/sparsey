use crate::data::{Component, Entity};
use crate::query::{Comp, CompMut};
use crate::world::{ComponentSet, Components, Entities, Layout};
use std::any::TypeId;
use std::collections::HashSet;

#[derive(Default)]
pub struct World {
    entities: Entities,
    components: Components,
    group_indexes: HashSet<usize>,
}

impl World {
    pub fn with_layout(layout: &Layout) -> Self {
        let mut world = Self::default();
        world.set_layout(layout);
        world
    }

    pub fn set_layout(&mut self, layout: &Layout) {
        self.entities.maintain();
        self.components.set_layout(&layout, self.entities.as_ref());
    }

    pub fn register<T>(&mut self)
    where
        T: Component,
    {
        self.components.register::<T>()
    }

    pub fn maintain(&mut self) {
        self.entities.maintain();
    }

    pub fn finish_frame(&mut self) {
        self.entities.maintain();
        self.components.clear_flags();
    }

    pub fn clear(&mut self) {
        self.entities.clear();
        self.components.clear();
    }

    pub fn create<C>(&mut self, components: C) -> Entity
    where
        C: ComponentSet,
    {
        let entity = self.entities.create();
        let _ = self.append(entity, components);
        entity
    }

    pub fn extend<C, I>(&mut self, components_iter: I) -> &[Entity]
    where
        C: ComponentSet,
        I: IntoIterator<Item = C>,
    {
        let initial_entity_count = self.entities.as_ref().len();

        {
            let mut storages = unsafe { C::borrow_storages(&self.components) };
            let entities = &mut self.entities;

            components_iter.into_iter().for_each(|components| {
                let entity = entities.create();

                unsafe {
                    C::insert(&mut storages, entity, components);
                }
            });
        }

        self.update_group_indexes(C::components().as_ref());
        let new_entities = &self.entities.as_ref()[initial_entity_count..];

        for &entity in new_entities {
            for &i in self.group_indexes.iter() {
                self.components.grouped.group_components(i, entity);
            }
        }

        new_entities
    }

    pub fn destroy(&mut self, entity: Entity) -> bool {
        if !self.entities.contains(entity) {
            return false;
        }

        for i in 0..self.components.grouped.group_count() {
            self.components.grouped.ungroup_components(i, entity);
        }

        for sparse_set in self.components.iter_sparse_sets_mut() {
            sparse_set.delete(entity);
        }

        true
    }

    pub fn append<C>(&mut self, entity: Entity, components: C) -> Result<(), ()>
    where
        C: ComponentSet,
    {
        if !self.entities.contains(entity) {
            return Err(());
        }

        unsafe {
            let mut storages = C::borrow_storages(&self.components);
            C::insert(&mut storages, entity, components);
        }

        self.update_group_indexes(C::components().as_ref());

        for &i in self.group_indexes.iter() {
            self.components.grouped.group_components(i, entity);
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
            self.components.grouped.ungroup_components(i, entity);
        }

        unsafe {
            let mut storages = C::borrow_storages(&self.components);
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
            self.components.grouped.ungroup_components(i, entity);
        }

        unsafe {
            let mut storages = C::borrow_storages(&self.components);
            C::delete(&mut storages, entity);
        }
    }

    pub fn entities(&self) -> &Entities {
        &self.entities
    }

    pub fn borrow_comp<T>(&self) -> Option<Comp<T>>
    where
        T: Component,
    {
        self.components.borrow_comp::<T>()
    }

    pub fn borrow_comp_mut<T>(&self) -> Option<CompMut<T>>
    where
        T: Component,
    {
        self.components.borrow_comp_mut::<T>()
    }

    fn update_group_indexes(&mut self, type_ids: &[TypeId]) {
        let grouped_components = &self.components.grouped;

        self.group_indexes.clear();
        self.group_indexes.extend(
            type_ids
                .iter()
                .flat_map(|type_id| grouped_components.get_group_index(type_id)),
        );
    }
}
