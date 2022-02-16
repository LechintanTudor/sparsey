use crate::components::{ComponentSet, ComponentStorages};
use crate::layout::Layout;
use crate::storage::{Component, ComponentStorage, Entity, EntityStorage};
use crate::utils::panic_missing_comp;
use crate::world::{Comp, CompMut, Entities};
use std::any::TypeId;
use std::{iter, mem};

/// Container for entities and their associated components.
#[derive(Default)]
pub struct World {
    entities: EntityStorage,
    components: ComponentStorages,
}

impl World {
    /// Creates an empty world with the storages arranged as described by `layout`.
    pub fn with_layout(layout: &Layout) -> Self {
        let mut world = Self::default();
        world.set_layout(layout);
        world
    }

    /// Arranges the storages as described by `layout`. This function iterates through all the
    /// entities to ararange their components, so it is best called right after creating the
    /// `World`.
    pub fn set_layout(&mut self, layout: &Layout) {
        let mut storages = mem::take(&mut self.components).into_storages();

        unsafe {
            self.components = ComponentStorages::new(layout, &mut storages);
            self.components.group_all_families(self.entities.iter().copied());
        }
    }

    /// Creates a storage for components of type `T` if one doesn't already exist.
    pub fn register<T>(&mut self)
    where
        T: Component,
    {
        self.components.register::<T>()
    }

    pub(crate) unsafe fn register_with(
        &mut self,
        component_type_id: TypeId,
        storage_builder: impl FnOnce() -> ComponentStorage,
    ) {
        self.components.register_with(component_type_id, storage_builder)
    }

    /// Check if a component type is registered.
    #[must_use]
    pub fn is_registered(&self, component_type_id: &TypeId) -> bool {
        self.components.is_registered(component_type_id)
    }

    /// Creates an entity with the given `components` and returns it.
    pub fn create<C>(&mut self, components: C) -> Entity
    where
        C: ComponentSet,
    {
        let entity = self.entities.create();
        let _ = self.insert(entity, components);
        entity
    }

    /// Creates new entities with the components produced by `components_iter`. Returns the newly
    /// created entities as a slice.
    pub fn bulk_create<C, I>(&mut self, components_iter: I) -> &[Entity]
    where
        C: ComponentSet,
        I: IntoIterator<Item = C>,
    {
        C::extend(&mut self.entities, &mut self.components, components_iter)
    }

    /// Removes `entity` and all of its components from the world. Returns `true` if the entity was
    /// successfully removed.
    pub fn destroy(&mut self, entity: Entity) -> bool {
        if !self.entities.destroy(entity) {
            return false;
        }

        self.components.ungroup_all_families(iter::once(entity));

        for storage in self.components.iter_mut() {
            storage.delete(entity);
        }

        true
    }

    /// Removes all entities (and their components) yielded by the `entities` iterator. Returns the
    /// number of entities successfully removed.
    pub fn bulk_destroy<'a, E>(&mut self, entities: E) -> usize
    where
        E: IntoIterator<Item = &'a Entity>,
        E::IntoIter: Clone,
    {
        let entities = entities.into_iter().copied();

        self.components.ungroup_all_families(entities.clone());

        for storage in self.components.iter_mut() {
            entities.clone().for_each(|entity| {
                storage.delete(entity);
            });
        }

        entities.into_iter().map(|entity| self.entities.destroy(entity) as usize).sum()
    }

    /// Associates the given `components` to `entity` if `entity` exists in the world. Returns
    /// `true` if the operation was successful.
    pub fn insert<C>(&mut self, entity: Entity, components: C) -> bool
    where
        C: ComponentSet,
    {
        if !self.entities.contains(entity) {
            return false;
        }

        C::insert(&mut self.components, entity, components);
        true
    }

    /// Removes a component set from `entity` and returns them.
    #[must_use = "use `delete` to discard the components"]
    pub fn remove<C>(&mut self, entity: Entity) -> C::RemoveResult
    where
        C: ComponentSet,
    {
        C::remove(&mut self.components, entity)
    }

    /// Deletes a component set from `entity`. This is faster than removing them.
    pub fn delete<C>(&mut self, entity: Entity)
    where
        C: ComponentSet,
    {
        C::delete(&mut self.components, entity);
    }

    /// Returns `true` if `entity` exists in the world.
    #[must_use]
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains(entity)
    }

    /// Returns all the entities in the world as a slice.
    pub fn entities(&self) -> &[Entity] {
        self.entities.as_ref()
    }

    /// Removes all the entities and components from the world.
    pub fn clear(&mut self) {
        self.entities.clear();
        self.components.clear();
    }

    /// Borrows a view over all entities in the world. The view can be used in systems running in
    /// parallel to create new entities atomically.
    pub fn borrow_entities(&self) -> Entities {
        Entities::new(&self.entities)
    }

    /// Borrows an immutable view over all components of type `T` in the world. Panics if the
    /// component wasn't registered.
    pub fn borrow<T>(&self) -> Comp<T>
    where
        T: Component,
    {
        self.components
            .borrow_with_info(&TypeId::of::<T>())
            .map(|(storage, info)| unsafe { Comp::<T>::new(storage, info) })
            .unwrap_or_else(|| panic_missing_comp::<T>())
    }

    /// Borrows a mutable view over all components of type `T` in in the world. Panics if the
    /// component wasn't registered.
    pub fn borrow_mut<T>(&self) -> CompMut<T>
    where
        T: Component,
    {
        self.components
            .borrow_with_info_mut(&TypeId::of::<T>())
            .map(|(storage, info)| unsafe { CompMut::<T>::new(storage, info) })
            .unwrap_or_else(|| panic_missing_comp::<T>())
    }

    /// Adds the atomically created entities to the main entity storage. Called automatically by
    /// `Schedule`.
    pub fn maintain(&mut self) {
        self.entities.maintain();
    }
}
