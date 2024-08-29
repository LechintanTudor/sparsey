mod builder;

pub use self::builder::*;

use crate::component::{
    Component, ComponentData, ComponentSet, ComponentStorage, GroupInfo, GroupLayout, View, ViewMut,
};
use crate::entity::{Entity, EntityStorage};
use crate::query::{Query, QueryAll, QueryOne};

/// Storage for entities and components.
#[derive(Default, Debug)]
pub struct World {
    pub(crate) entities: EntityStorage,
    pub(crate) components: ComponentStorage,
}

impl World {
    /// Creates a new `EntityStorage` with the given `GroupLayout`.
    #[inline]
    #[must_use]
    pub fn new(layout: &GroupLayout) -> Self {
        Self {
            entities: EntityStorage::default(),
            components: ComponentStorage::new(layout),
        }
    }

    #[inline]
    pub fn builder() -> WorldBuilder {
        WorldBuilder::default()
    }

    pub fn query_one<G>(&self) -> QueryOne<G, (), ()>
    where
        G: Query,
    {
        QueryOne::new(self)
    }

    pub fn query_all<G>(&self) -> QueryAll<G, (), ()>
    where
        G: Query,
    {
        QueryAll::new(self)
    }

    #[must_use]
    pub fn contains<G>(&self, entity: Entity) -> bool
    where
        G: Query,
    {
        QueryOne::<G, (), ()>::new(self).contains(entity)
    }

    pub fn for_each<G>(&self, f: impl FnMut(G::Item<'_>))
    where
        G: Query,
    {
        self.query_all().for_each(f);
    }

    #[cfg(feature = "parallel")]
    pub fn par_for_each<G>(&self, f: impl Fn(G::Item<'_>) + Send + Sync)
    where
        G: Query,
        for<'a> G::Item<'a>: Send,
    {
        self.query_all().par_for_each(f);
    }

    /// Sets a new `GroupLayout`.
    ///
    /// This function iterates over all entities in the storage, so it is best called when the
    /// storage is empty.
    #[inline]
    pub fn set_layout(&mut self, layout: &GroupLayout) {
        unsafe {
            self.components.set_layout(layout, self.entities.as_slice());
        }
    }

    /// Registers a new component type.
    ///
    /// Returns whether the component was newly registered.
    pub fn register<T>(&mut self) -> bool
    where
        T: Component,
    {
        self.components.register::<T>()
    }

    #[inline]
    pub fn register_dyn(&mut self, component: ComponentData) -> bool {
        self.components.register_dyn(component)
    }

    /// Returns whether component type `T` is registered.
    #[must_use]
    pub fn is_registered<T>(&self) -> bool
    where
        T: Component,
    {
        self.components.is_registered::<T>()
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

    /// Creates new entities with the components produced by the iterator.
    ///
    /// Returns the newly created entities as a slice.
    pub fn extend<C, I>(&mut self, components: I) -> &[Entity]
    where
        C: ComponentSet,
        I: IntoIterator<Item = C>,
    {
        C::extend(self, components)
    }

    /// Creates a new entity without requiring exclusive access to the storage. The entity is not
    /// added to the storage until [`maintain`](Self::maintain) is called.
    ///
    /// Returns the newly created entity.
    #[inline]
    pub fn create_atomic(&self) -> Entity {
        self.entities.create_atomic()
    }

    /// Adds the given `components` to `entity` if `entity` is present in the storage.
    ///
    /// Returns whether the components were successfully added.
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

    /// Removes components from the given `entity`.
    ///
    /// Returns the components that were successfully removed.
    #[must_use = "Use `delete` to discard the components."]
    pub fn remove<C>(&mut self, entity: Entity) -> C::Remove
    where
        C: ComponentSet,
    {
        C::remove(self, entity)
    }

    /// Removes components from the given `entity`.
    pub fn delete<C>(&mut self, entity: Entity)
    where
        C: ComponentSet,
    {
        C::delete(self, entity);
    }

    /// Removes the given `entity` and its components from the storage.
    ///
    /// Returns whether the `entity` was present in the storage.
    #[inline]
    pub fn destroy(&mut self, entity: Entity) -> bool {
        if !self.entities.remove(entity) {
            return false;
        }

        self.components.strip(entity);
        true
    }

    /// Returns whether the storage contains no entities.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// Removes all entities and components from the storage.
    #[inline]
    pub fn clear(&mut self) {
        self.maintain();
        self.entities.clear();
        self.components.clear();
    }

    /// Removes all entities and components from the storage and resets the entity allocator.
    ///
    /// After this call, the storage is allowed to return previously allocated entities.
    #[inline]
    pub fn reset(&mut self) {
        self.entities.reset();
        self.components.clear();
    }

    /// Adds the entities allocated with [`create_atomic`](Self::create_atomic) to the storage.
    #[inline]
    pub fn maintain(&mut self) {
        self.entities.maintain();
    }

    /// Returns wether `entity` is present in the storage.
    #[inline]
    #[must_use]
    pub fn contains_entity(&self, entity: Entity) -> bool {
        self.entities.contains(entity)
    }

    /// Returns all entities in the storage as a slice.
    #[inline]
    #[must_use]
    pub fn entities(&self) -> &[Entity] {
        self.entities.as_slice()
    }

    /// Borrows a shared view over all components of type `T` in the storage.
    #[must_use]
    pub fn borrow<T>(&self) -> View<T>
    where
        T: Component,
    {
        self.components.borrow::<T>()
    }

    /// Borrows an exclusive view over all components of type `T` in the storage.
    #[must_use]
    pub fn borrow_mut<T>(&self) -> ViewMut<T>
    where
        T: Component,
    {
        self.components.borrow_mut::<T>()
    }

    #[must_use]
    pub fn borrow_with_group_info<T>(&self) -> (View<T>, Option<GroupInfo>)
    where
        T: Component,
    {
        self.components.borrow_with_group_info::<T>()
    }

    #[must_use]
    pub fn borrow_with_group_info_mut<T>(&self) -> (ViewMut<T>, Option<GroupInfo>)
    where
        T: Component,
    {
        self.components.borrow_with_group_info_mut::<T>()
    }
}
