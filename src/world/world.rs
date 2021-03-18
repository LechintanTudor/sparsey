use crate::data::{Component, Entity, TypeErasedSparseSet};
use crate::query::{Comp, CompMut};
use crate::world::{ComponentSet, Components, Entities, Layout};
use std::any::TypeId;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;

/// Container for component storages and entities.
#[derive(Default)]
pub struct World {
    entities: Entities,
    components: Components,
    group_indexes: HashSet<usize>,
}

impl World {
    /// Create a `World` with the given `Layout`.
    pub fn with_layout(layout: &Layout) -> Self {
        let mut world = Self::default();
        world.set_layout(layout);
        world
    }

    /// Set the `Layout` of the `World`. Use this before adding
    /// any entities to the `World` as this function has to iterate
    /// all the entities in the `World` in order to group them.
    pub fn set_layout(&mut self, layout: &Layout) {
        self.entities.maintain();
        self.components.set_layout(&layout, self.entities.as_ref());
    }

    /// Add a component storage for the given type if one does not exist already.
    pub fn register<T>(&mut self)
    where
        T: Component,
    {
        self.components.register::<T>()
    }

    /// Add a component storage if one does not exist already for this type.
    pub fn register_storage(&mut self, sparse_set: TypeErasedSparseSet) {
        self.components.register_storage(sparse_set);
    }

    /// Remove all entities and components.
    pub fn clear(&mut self) {
        self.entities.clear();
        self.components.clear();
    }

    /// Clear all component flags.
    pub fn clear_flags(&mut self) {
        self.components.clear_flags();
    }

    /// Create an `Entity` with the given components and return it.
    pub fn create<C>(&mut self, components: C) -> Entity
    where
        C: ComponentSet,
    {
        let entity = self.entities.create();
        let _ = self.append(entity, components);
        entity
    }

    /// Extend the `World` with a component iterator.
    /// Return the newly created entities as a slice.
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

    /// Destroy an `Entity` and all of its components.
    /// Return whether or not there was an `Entity` to destroy.
    pub fn destroy(&mut self, entity: Entity) -> bool {
        if !self.entities.destroy(entity) {
            return false;
        }

        for i in 0..self.components.grouped.group_set_count() {
            self.components.grouped.ungroup_components(i, entity);
        }

        for sparse_set in self.components.iter_sparse_sets_mut() {
            sparse_set.delete(entity);
        }

        true
    }

    /// Append a set of components to the given `Entity`, if it exists.
    pub fn append<C>(&mut self, entity: Entity, components: C) -> Result<(), NoSuchEntity>
    where
        C: ComponentSet,
    {
        if !self.contains(entity) {
            return Err(NoSuchEntity);
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

    /// Remove a set of components from an `Entity` and return them if they
    /// were all present before calling this function.
    pub fn remove<C>(&mut self, entity: Entity) -> Option<C>
    where
        C: ComponentSet,
    {
        if !self.contains(entity) {
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

    /// Delete a set of components from an `Entity`.
    pub fn delete<C>(&mut self, entity: Entity)
    where
        C: ComponentSet,
    {
        if !self.contains(entity) {
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

    // Check if the `World` contains the given `Entity`.
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains(entity)
    }

    // Get the number of entities in the `World`.
    pub fn len(&self) -> usize {
        self.entities.as_ref().len()
    }

    /// Get a slice containing all entities in the `World`.
    pub fn entities(&self) -> &[Entity] {
        self.entities.as_ref()
    }

    /// Get a shared view over a component storage.
    pub fn borrow_comp<T>(&self) -> Option<Comp<T>>
    where
        T: Component,
    {
        self.components.borrow_comp::<T>()
    }

    /// Get an exclusive view over a component storage.
    pub fn borrow_comp_mut<T>(&self) -> Option<CompMut<T>>
    where
        T: Component,
    {
        self.components.borrow_comp_mut::<T>()
    }

    pub(crate) fn maintain(&mut self) {
        self.entities.maintain();
    }

    pub(crate) fn entity_storage(&self) -> &Entities {
        &self.entities
    }

    fn update_group_indexes(&mut self, type_ids: &[TypeId]) {
        let grouped_components = &self.components.grouped;

        self.group_indexes.clear();
        self.group_indexes.extend(
            type_ids
                .iter()
                .flat_map(|type_id| grouped_components.get_group_set_index(type_id)),
        );
    }
}

/// Error returned when trying to access entities
/// which are not contained in the `World`.
#[derive(Debug)]
pub struct NoSuchEntity;

impl Error for NoSuchEntity {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for NoSuchEntity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No such entity was found in the World")
    }
}
