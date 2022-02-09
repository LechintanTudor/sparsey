use crate::components::{ComponentSet, ComponentStorages};
use crate::layout::Layout;
use crate::storage::{Component, ComponentStorage, Entity, EntityStorage};
use crate::utils::panic_missing_comp;
use crate::world::{Comp, CompMut, Entities, NoSuchEntity};
use std::any::TypeId;
use std::mem;

/// Container for entities, components and resources.
#[derive(Default)]
pub struct World {
    entities: EntityStorage,
    components: ComponentStorages,
}

impl World {
    /// Creates an empty world with the storages arranged as described by
    /// `layout`.
    pub fn with_layout(layout: &Layout) -> Self {
        let mut world = Self::default();
        world.set_layout(layout);
        world
    }

    /// Arranges the storages as described by `layout`. This function iterates
    /// through all the entities to ararange their components, so it is best
    /// called right after creating the `World`.
    pub fn set_layout(&mut self, layout: &Layout) {
        let mut storages = mem::take(&mut self.components).into_storages();

        unsafe {
            self.components = ComponentStorages::new(layout, &mut storages);
            self.components.group_all_components(self.entities.as_ref());
        }
    }

    /// Creates a component storage for `T` if one doesn't already exist.
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

    /// Creates an `Entity` with the given `components` and returns it.
    pub fn create<C>(&mut self, components: C) -> Entity
    where
        C: ComponentSet,
    {
        let entity = self.entities.create();
        let _ = self.insert(entity, components);
        entity
    }

    /// Creates new entities with the components produced by
    /// `components_iter`. Returns the newly created entities as a slice.
    pub fn bulk_create<C, I>(&mut self, components_iter: I) -> &[Entity]
    where
        C: ComponentSet,
        I: IntoIterator<Item = C>,
    {
        C::extend(&mut self.entities, &mut self.components, components_iter)
    }

    /// Removes `entity` and all of its components from the `World`.
    /// Returns `true` if the `Entity` was successfully removed.
    pub fn destroy(&mut self, entity: Entity) -> bool {
        if !self.entities.destroy(entity) {
            return false;
        }

        self.components.ungroup_all_components(Some(&entity));

        for storage in self.components.iter_mut() {
            storage.remove_and_drop(entity);
        }

        true
    }

    /// Removes all entities (and their components) produced by the iterator.
    /// Returns the number of entities successfully removed.
    pub fn bulk_destroy<'a, E>(&mut self, entities: E) -> usize
    where
        E: IntoIterator<Item = &'a Entity>,
        E::IntoIter: Clone,
    {
        let entities = entities.into_iter();

        self.components.ungroup_all_components(entities.clone());

        for storage in self.components.iter_mut() {
            entities.clone().for_each(|&entity| {
                storage.remove_and_drop(entity);
            });
        }

        entities.into_iter().map(|&entity| self.entities.destroy(entity) as usize).sum()
    }

    /// Appends the given `components` to `entity` if `entity` exists in the
    /// `World`.
    pub fn insert<C>(&mut self, entity: Entity, components: C) -> Result<(), NoSuchEntity>
    where
        C: ComponentSet,
    {
        if !self.contains(entity) {
            return Err(NoSuchEntity);
        }

        C::insert(&mut self.components, entity, components);
        Ok(())
    }

    /// Removes a component set from `entity` and returns them if they all
    /// exist in the `World` before the call.
    #[must_use = "use `delete_components` to discard the components"]
    pub fn remove<C>(&mut self, entity: Entity) -> C::RemoveResult
    where
        C: ComponentSet,
    {
        C::remove(&mut self.components, entity)
    }

    /// Deletes a component set from `entity`. This is faster than removing
    /// the components.
    pub fn delete<C>(&mut self, entity: Entity)
    where
        C: ComponentSet,
    {
        C::delete(&mut self.components, entity);
    }

    /// Returns `true` if `entity` exists in the `World`.
    #[must_use]
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains(entity)
    }

    /// Returns all the `entities` in the world as a slice.
    pub fn entities(&self) -> &[Entity] {
        self.entities.as_ref()
    }

    /// Removes all entities and components in the world.
    pub fn clear(&mut self) {
        self.entities.clear();
        self.components.clear();
    }

    pub fn borrow_entities(&self) -> Entities {
        Entities::new(&self.entities)
    }

    pub fn borrow<T>(&self) -> Comp<T>
    where
        T: Component,
    {
        self.components
            .borrow_with_info(&TypeId::of::<T>())
            .map(|(storage, info)| unsafe { Comp::<T>::new(storage, info) })
            .unwrap_or_else(|| panic_missing_comp::<T>())
    }

    pub fn borrow_mut<T>(&self) -> CompMut<T>
    where
        T: Component,
    {
        self.components
            .borrow_with_info_mut(&TypeId::of::<T>())
            .map(|(storage, info)| unsafe { CompMut::<T>::new(storage, info) })
            .unwrap_or_else(|| panic_missing_comp::<T>())
    }

    pub fn maintain(&mut self) {
        self.entities.maintain();
    }
}
