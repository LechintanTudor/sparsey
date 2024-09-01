//! Manage and query entities and their associated components.

mod builder;

pub use self::builder::*;

use crate::component::{
    Component, ComponentData, ComponentSet, ComponentStorage, GroupInfo, GroupLayout, View, ViewMut,
};
use crate::entity::{Entity, EntityStorage};
use crate::query::{Query, QueryAll, QueryOne};
use core::any::TypeId;

/// Collection for entities and their associated components.
#[derive(Default, Debug)]
pub struct World {
    pub(crate) entities: EntityStorage,
    pub(crate) components: ComponentStorage,
}

impl World {
    /// Create and configure a new world using the builder pattern.
    #[inline]
    pub fn builder() -> WorldBuilder {
        WorldBuilder::default()
    }

    /// Creates a new world with the given group `layout`.
    #[inline]
    #[must_use]
    pub fn new(layout: &GroupLayout) -> Self {
        Self {
            entities: EntityStorage::default(),
            components: ComponentStorage::new(layout),
        }
    }

    /// Sets a new group `layout` on this world.
    ///
    /// This operation requires iteration over all entities in the world, so it
    /// is best called when the world is empty.
    #[inline]
    pub fn set_layout(&mut self, layout: &GroupLayout) {
        unsafe {
            self.components.set_layout(layout, self.entities.as_slice());
        }
    }

    /// Registers a new component type on this world.
    ///
    /// Returns whether the component was newly registered.
    pub fn register<T>(&mut self) -> bool
    where
        T: Component,
    {
        self.register_dyn(ComponentData::new::<T>())
    }

    /// Registers a new component type on this world.
    ///
    /// Returns whether the component was newly registered.
    #[inline]
    pub fn register_dyn(&mut self, component: ComponentData) -> bool {
        self.components.register_dyn(component)
    }

    /// Returns whether the component type is registered.
    #[must_use]
    pub fn is_registered<T>(&self) -> bool
    where
        T: Component,
    {
        self.is_registered_dyn(TypeId::of::<T>())
    }

    /// Returns whether the component type is registered.
    #[inline]
    #[must_use]
    pub fn is_registered_dyn(&self, component: TypeId) -> bool {
        self.components.is_registered_dyn(component)
    }

    /// Creates a new entity with the given `components`.
    ///
    /// Returns the newly created entity.
    pub fn create<C>(&mut self, components: C) -> Entity
    where
        C: ComponentSet,
    {
        let entity = self.entities.create();
        C::insert(self, entity, components);
        entity
    }

    /// Creates new entities with the `components` produced by the iterator.
    ///
    /// Returns the newly created entities as a slice.
    pub fn extend<C, I>(&mut self, components: I) -> &[Entity]
    where
        C: ComponentSet,
        I: IntoIterator<Item = C>,
    {
        C::extend(self, components)
    }

    /// Removes the `entity` and its associated components from the world.
    ///
    /// Returns whether the operation was successfull, i.e. whether the entity
    /// existed in the world before this call.
    #[inline]
    pub fn destroy(&mut self, entity: Entity) -> bool {
        if !self.entities.remove(entity) {
            return false;
        }

        self.components.strip(entity);
        true
    }

    /// Queues the creation of an entity without requiring exclusive access to
    /// the world. Entities created with this method can be added to the world
    /// by calling [`maintain`](Self::maintain).
    ///
    /// Returns the entity to be created.
    #[inline]
    pub fn create_atomic(&self) -> Entity {
        self.entities.create_atomic()
    }

    /// Adds the entities created with [`create_atomic`](Self::create_atomic)
    /// to the world.
    #[inline]
    pub fn maintain(&mut self) {
        self.entities.maintain();
    }

    /// Inserts `components` to an existing `entity`, overwriting previous data
    /// if necessary.
    ///
    /// Returns whether the `components` were added successfully, i.e. whether
    /// the `entity` existed in the world before this call.
    pub fn insert<C>(&mut self, entity: Entity, components: C) -> bool
    where
        C: ComponentSet,
    {
        if !self.entities.contains(entity) {
            return false;
        }

        C::insert(self, entity, components);
        true
    }

    /// Removes components from the `entity`, returning the removed components
    /// as options.
    #[must_use = "Use `delete` to discard the components."]
    pub fn remove<C>(&mut self, entity: Entity) -> C::Remove
    where
        C: ComponentSet,
    {
        C::remove(self, entity)
    }

    /// Removes components from the `entity`, without returning them.
    ///
    /// This is faster than calling [`remove`](Self::remove).
    pub fn delete<C>(&mut self, entity: Entity)
    where
        C: ComponentSet,
    {
        C::delete(self, entity);
    }

    /// Queries an entity with the given components.
    pub fn query_one<G>(&self) -> QueryOne<G, (), ()>
    where
        G: Query,
    {
        QueryOne::new(self)
    }

    /// Queries all entities with the given components.
    pub fn query_all<G>(&self) -> QueryAll<G, (), ()>
    where
        G: Query,
    {
        QueryAll::new(self)
    }

    /// Returns whether the `entity` contains the given components.
    #[must_use]
    pub fn contains<G>(&self, entity: Entity) -> bool
    where
        G: Query,
    {
        QueryOne::<G, (), ()>::new(self).contains(entity)
    }

    /// Iterates over all entities with the given components.
    pub fn for_each<G>(&self, f: impl FnMut(G::Item<'_>))
    where
        G: Query,
    {
        self.query_all().for_each(f);
    }

    /// Iterates in parallel over all entities with the given components.
    #[cfg(feature = "parallel")]
    pub fn par_for_each<G>(&self, f: impl Fn(G::Item<'_>) + Send + Sync)
    where
        G: Query,
    {
        self.query_all().par_for_each(f);
    }

    /// Returns whether the world contains the given `entity`.
    #[inline]
    #[must_use]
    pub fn contains_entity(&self, entity: Entity) -> bool {
        self.entities.contains(entity)
    }

    /// Returns all entities in the world as a slice.
    #[inline]
    #[must_use]
    pub fn entities(&self) -> &[Entity] {
        self.entities.as_slice()
    }

    /// Returns whether the world contains no entities.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// Removes all entities and components from the world.
    #[inline]
    pub fn clear(&mut self) {
        self.entities.clear();
        self.components.clear();
    }

    /// Removes all entities and components from the world and resets the entity
    /// allocator, allowing the world to reuse previously allocated entities.
    #[inline]
    pub fn reset(&mut self) {
        self.entities.reset();
        self.components.clear();
    }

    /// Returns a shared view over all components of type `T`.
    #[must_use]
    pub fn borrow<T>(&self) -> View<T>
    where
        T: Component,
    {
        self.components.borrow::<T>()
    }

    /// Returns an exclusive view over all components of type `T`.
    #[must_use]
    pub fn borrow_mut<T>(&self) -> ViewMut<T>
    where
        T: Component,
    {
        self.components.borrow_mut::<T>()
    }

    /// Returns a shared view over all components of type `T`, along with
    /// grouping information.
    #[must_use]
    pub fn borrow_with_group_info<T>(&self) -> (View<T>, Option<GroupInfo>)
    where
        T: Component,
    {
        self.components.borrow_with_group_info::<T>()
    }

    /// Returns an exclusive view over all components of type `T`, along with
    /// grouping information.
    #[must_use]
    pub fn borrow_with_group_info_mut<T>(&self) -> (ViewMut<T>, Option<GroupInfo>)
    where
        T: Component,
    {
        self.components.borrow_with_group_info_mut::<T>()
    }
}
