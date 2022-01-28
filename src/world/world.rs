use crate::components::{ComponentSet, ComponentStorages};
use crate::layout::Layout;
use crate::resources::{Resource, Resources};
use crate::storage::{Component, ComponentStorage, Entity, EntityStorage};
use crate::world::{BorrowWorld, Comp, CompMut, Entities, NoSuchEntity, Res, ResMut, SyncWorld};
use std::any::TypeId;
use std::mem;

// TODO: Remove Resources from World and rename create_entity to create

/// Container for entities, components and resources.
#[derive(Default)]
pub struct World {
    entities: EntityStorage,
    components: ComponentStorages,
    resources: Resources,
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

    pub(crate) unsafe fn register_with<F>(&mut self, type_id: TypeId, storage_builder: F)
    where
        F: FnOnce() -> ComponentStorage,
    {
        self.components.register_with(type_id, storage_builder);
    }

    /// Check if a component type is registered.
    #[must_use]
    pub fn is_registered(&self, component_type_id: &TypeId) -> bool {
        self.components.is_registered(component_type_id)
    }

    /// Creates an `Entity` with the given `components` and returns it.
    pub fn create_entity<C>(&mut self, components: C) -> Entity
    where
        C: ComponentSet,
    {
        let entity = self.entities.create();
        let _ = self.insert_components(entity, components);
        entity
    }

    /// Creates new entities with the components produced by
    /// `components_iter`. Returns the newly created entities as a slice.
    pub fn create_entities<C, I>(&mut self, components_iter: I) -> &[Entity]
    where
        C: ComponentSet,
        I: IntoIterator<Item = C>,
    {
        C::extend(&mut self.entities, &mut self.components, components_iter)
    }

    /// Removes `entity` and all of its components from the `World`.
    /// Returns `true` if the `Entity` was successfully removed.
    pub fn destroy_entity(&mut self, entity: Entity) -> bool {
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
    pub fn destroy_entities<'a, E>(&mut self, entities: E) -> usize
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
    pub fn insert_components<C>(
        &mut self,
        entity: Entity,
        components: C,
    ) -> Result<(), NoSuchEntity>
    where
        C: ComponentSet,
    {
        if !self.contains_entity(entity) {
            return Err(NoSuchEntity);
        }

        C::insert(&mut self.components, entity, components);
        Ok(())
    }

    /// Removes a component set from `entity` and returns them if they all
    /// exist in the `World` before the call.
    #[must_use = "use `delete_components` to discard the components"]
    pub fn remove_components<C>(&mut self, entity: Entity) -> Option<C>
    where
        C: ComponentSet,
    {
        C::remove(&mut self.components, entity)
    }

    /// Deletes a component set from `entity`. This is faster than removing
    /// the components.
    pub fn delete_components<C>(&mut self, entity: Entity)
    where
        C: ComponentSet,
    {
        C::delete(&mut self.components, entity);
    }

    /// Returns `true` if `entity` exists in the `World`.
    #[must_use]
    pub fn contains_entity(&self, entity: Entity) -> bool {
        self.entities.contains(entity)
    }

    /// Returns all the `entities` in the world as a slice.
    pub fn entities(&self) -> &[Entity] {
        self.entities.as_ref()
    }

    /// Removes all entities and components in the world.
    pub fn clear_entities(&mut self) {
        self.entities.clear();
        self.components.clear();
    }

    /// Inserts a resource of type `T` into the `World` and returns the previous
    /// one, if any.
    pub fn insert_resource<T>(&mut self, resource: T) -> Option<T>
    where
        T: Resource,
    {
        self.resources.insert(resource)
    }

    /// Removes a resource of type `T` from the `World` and returns it if it was
    /// successfully removed.
    pub fn remove_resource<T>(&mut self) -> Option<T>
    where
        T: Resource,
    {
        self.resources.remove::<T>()
    }

    /// Removes the resource with the given `TypeId` from the `World`. Returns
    /// `true` if the resource was successfully removed.
    pub fn delete_resource(&mut self, resource_type_id: &TypeId) -> bool {
        self.resources.delete(resource_type_id)
    }

    /// Returns `true` if the `World` contains a resource with the given
    /// `TypeId`.
    #[must_use]
    pub fn contains_resource(&self, resource_type_id: &TypeId) -> bool {
        self.resources.contains(resource_type_id)
    }

    /// Removes all resources from the `World`.
    pub fn clear_resources(&mut self) {
        self.resources.clear();
    }

    /// Removes all entities, components and resources from the `World`.
    pub fn clear(&mut self) {
        self.entities.clear();
        self.components.clear();
        self.resources.clear();
    }

    pub fn sync(&self) -> SyncWorld {
        SyncWorld::new(&self.entities, &self.components, self.resources.sync())
    }

    /// Borrows a component view or resource view from the `World`.
    pub fn borrow<'a, T>(&'a self) -> T::Item
    where
        T: BorrowWorld<'a>,
    {
        T::borrow(self)
    }

    pub(crate) fn borrow_entities(&self) -> Entities {
        Entities::new(&self.entities)
    }

    pub(crate) fn borrow_comp<T>(&self) -> Option<Comp<T>>
    where
        T: Component,
    {
        self.components
            .borrow_with_info(&TypeId::of::<T>())
            .map(|(storage, info)| unsafe { Comp::new(storage, info) })
    }

    pub(crate) fn borrow_comp_mut<T>(&self) -> Option<CompMut<T>>
    where
        T: Component,
    {
        self.components
            .borrow_with_info_mut(&TypeId::of::<T>())
            .map(|(storage, info)| unsafe { CompMut::new(storage, info) })
    }

    pub(crate) fn borrow_res<T>(&self) -> Option<Res<T>>
    where
        T: Resource,
    {
        self.resources.borrow::<T>().map(Res::new)
    }

    pub(crate) fn borrow_res_mut<T>(&self) -> Option<ResMut<T>>
    where
        T: Resource,
    {
        self.resources.borrow_mut::<T>().map(ResMut::new)
    }

    pub(crate) fn maintain(&mut self) {
        self.entities.maintain();
    }
}
