use crate::components::{Component, ComponentSet, ComponentStorages, ComponentStoragesIter};
use crate::layout::Layout;
use crate::resources::{Resource, ResourceStorage, ResourceStorageIter};
use crate::storage::{ComponentStorage, Entity, EntityStorage};
use crate::utils::{ChangeTicks, NonZeroTicks, Ticks};
use crate::world::{BorrowWorld, NoSuchEntity, TickOverflow};
use std::any::TypeId;
use std::mem;
use std::num::NonZeroU64;
use std::sync::atomic::{AtomicU64, Ordering};

/// Uniquely identifies a `World` during the execution of the program.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct WorldId(NonZeroU64);

impl WorldId {
    fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);

        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        NonZeroU64::new(id).map(Self).expect("Ran out of WorldIds")
    }
}

/// Container for component storages and entities.
pub struct World {
    pub(crate) id: WorldId,
    pub(crate) tick: NonZeroTicks,
    pub(crate) entities: EntityStorage,
    pub(crate) storages: ComponentStorages,
    pub(crate) resources: ResourceStorage,
}

impl Default for World {
    fn default() -> Self {
        Self {
            id: WorldId::new(),
            tick: NonZeroTicks::new(1).unwrap(),
            entities: Default::default(),
            storages: Default::default(),
            resources: Default::default(),
        }
    }
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
        let mut storages = mem::take(&mut self.storages).into_storages();

        unsafe {
            self.storages = ComponentStorages::new(layout, &mut storages);
            self.storages.group_all_components(self.entities.as_ref());
        }
    }

    /// Creates a component storage for `T` if one doesn't already exist.
    pub fn register<T>(&mut self)
    where
        T: Component,
    {
        self.storages.register::<T>()
    }

    pub(crate) unsafe fn register_with<F>(&mut self, type_id: TypeId, storage_builder: F)
    where
        F: FnOnce() -> ComponentStorage,
    {
        self.storages.register_with(type_id, storage_builder);
    }

    /// Check if a component type is registered.
    pub fn is_registered(&self, component_type_id: &TypeId) -> bool {
        self.storages.is_registered(component_type_id)
    }

    /// Returns an iterator over all registered storages and the`TypeId`s of the
    /// components they hold.
    pub fn storages(&mut self) -> ComponentStoragesIter {
        self.storages.iter()
    }

    /// Creates an `Entity` with the given `components` and returns it.
    pub fn create_entity<C>(&mut self, components: C) -> Entity
    where
        C: ComponentSet,
    {
        let ticks = ChangeTicks::just_added(self.tick.get());
        self.create_entity_with_ticks(components, ticks)
    }

    /// Same as `create`, but the `ChangeTicks` are provided by the caller.
    pub fn create_entity_with_ticks<C>(&mut self, components: C, ticks: ChangeTicks) -> Entity
    where
        C: ComponentSet,
    {
        let entity = self.entities.create();
        let _ = self.append_components_with_ticks(entity, components, ticks);
        entity
    }

    /// Creates new entities with the components produced by
    /// `components_iter`. Returns the newly created entities as a slice.
    pub fn create_entities<C, I>(&mut self, components_iter: I) -> &[Entity]
    where
        C: ComponentSet,
        I: IntoIterator<Item = C>,
    {
        let ticks = ChangeTicks::just_added(self.tick.get());
        self.create_entities_with_ticks(components_iter, ticks)
    }

    /// Same as `extend`, but the `ChangeTicks` are provided by the caller.
    pub fn create_entities_with_ticks<C, I>(
        &mut self,
        components_iter: I,
        ticks: ChangeTicks,
    ) -> &[Entity]
    where
        C: ComponentSet,
        I: IntoIterator<Item = C>,
    {
        unsafe {
            C::extend(
                &mut self.storages,
                &mut self.entities,
                components_iter,
                ticks,
            )
        }
    }

    /// Removes `entity` and all of its components from the `World`.
    /// Returns `true` if the `Entity` was successfully removed.
    pub fn destroy_entity(&mut self, entity: Entity) -> bool {
        if !self.entities.destroy(entity) {
            return false;
        }

        self.storages.ungroup_all_components(Some(&entity));

        for storage in self.storages.iter_mut() {
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

        self.storages.ungroup_all_components(entities.clone());

        for storage in self.storages.iter_mut() {
            entities.clone().for_each(|&entity| {
                storage.remove_and_drop(entity);
            });
        }

        entities
            .into_iter()
            .map(|&entity| self.entities.destroy(entity) as usize)
            .sum()
    }

    /// Appends the given `components` to `entity` if `entity` exists in the
    /// `World`.
    pub fn append_components<C>(
        &mut self,
        entity: Entity,
        components: C,
    ) -> Result<(), NoSuchEntity>
    where
        C: ComponentSet,
    {
        let ticks = ChangeTicks::just_added(self.tick.get());
        self.append_components_with_ticks(entity, components, ticks)
    }

    /// Same as `append`, but the `ChangeTicks` are provided by the caller.
    pub fn append_components_with_ticks<C>(
        &mut self,
        entity: Entity,
        components: C,
        ticks: ChangeTicks,
    ) -> Result<(), NoSuchEntity>
    where
        C: ComponentSet,
    {
        if !self.contains_entity(entity) {
            return Err(NoSuchEntity);
        }

        unsafe {
            C::insert(&mut self.storages, entity, components, ticks);
        }

        Ok(())
    }

    /// Removes a component set from `entity` and returns them if they all
    /// existed in the `World` before the call.
    pub fn remove_components<C>(&mut self, entity: Entity) -> Option<C>
    where
        C: ComponentSet,
    {
        unsafe { C::remove(&mut self.storages, entity) }
    }

    /// Deletes a component set from `entity`. This is faster than removing
    /// the components.
    pub fn delete_components<C>(&mut self, entity: Entity)
    where
        C: ComponentSet,
    {
        unsafe {
            C::delete(&mut self.storages, entity);
        }
    }

    /// Returns `true` if `entity` exists in the `World`.
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
        self.storages.clear();
    }

    /// Inserts a resource of type `T` into the `World` and returns the previous
    /// one, if any.
    pub fn insert_resource<T>(&mut self, resource: T) -> Option<T>
    where
        T: Resource,
    {
        let ticks = ChangeTicks::just_added(self.tick.get());
        self.insert_resource_with_ticks(resource, ticks)
    }

    /// Same as `insert_resource`, but the `ChangeTicks` are provided by the
    /// caller.
    pub fn insert_resource_with_ticks<T>(&mut self, resource: T, ticks: ChangeTicks) -> Option<T>
    where
        T: Resource,
    {
        self.resources.insert(resource, ticks)
    }

    /// Removes a resource of type `T` from the `World` and returns it if it was
    /// successfully removed.
    pub fn remove_resource<T>(&mut self) -> Option<T>
    where
        T: Resource,
    {
        self.resources.remove::<T>()
    }

    /// Returns `true` if the `World` contains a resource with the given
    /// `TypeId`.
    pub fn contains_resource(&self, resource_type_id: &TypeId) -> bool {
        self.resources.contains(resource_type_id)
    }

    /// Removes all resources from the `World`.
    pub fn clear_resources(&mut self) {
        self.resources.clear();
    }

    /// Returns an iterator over all resources stored in the `World` and their
    /// `TypeId`s.
    pub fn resources(&mut self) -> ResourceStorageIter {
        self.resources.iter()
    }

    /// Removes all entities, components and resources from the `World`.
    pub fn clear(&mut self) {
        self.entities.clear();
        self.storages.clear();
        self.resources.clear();
    }

    /// Borrows a component view or resource view from the `World`.
    pub fn borrow<'a, T>(&'a self) -> T::Item
    where
        T: BorrowWorld<'a>,
    {
        T::borrow(self, self.tick.get() - 1)
    }

    /// Sets the world tick used for change detection.
    pub fn set_tick(&mut self, tick: NonZeroTicks) {
        self.tick = tick;
    }

    /// Advances the world tick. Should be called after each game
    /// update for proper change detection.
    pub fn increment_tick(&mut self) -> Result<(), TickOverflow> {
        if self.tick.get() != Ticks::MAX {
            self.tick = NonZeroTicks::new(self.tick.get() + 1).unwrap();
            Ok(())
        } else {
            self.tick = NonZeroTicks::new(1).unwrap();
            Err(TickOverflow)
        }
    }

    /// Returns the world tick used for change detection.
    pub fn tick(&self) -> Ticks {
        self.tick.get()
    }

    /// Returns the `WorldId` which uniquely identifies this `World`.
    pub fn id(&self) -> WorldId {
        self.id
    }
}
