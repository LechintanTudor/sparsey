//! Manages entities and their associated components.

mod borrow;
mod component;
mod component_set;
mod component_sparse_set;
mod component_storage;
mod entity;
mod entity_allocator;
mod entity_sparse_set;
mod group;
mod group_info;
mod group_layout;
mod group_mask;
mod sparse_vec;

pub use self::borrow::*;
pub use self::component::*;
pub use self::component_set::*;
pub use self::entity::*;
pub use self::group_info::*;
pub use self::group_layout::*;
pub use self::sparse_vec::*;

pub(crate) use self::component_sparse_set::*;
pub(crate) use self::component_storage::*;
pub(crate) use self::entity_allocator::*;
pub(crate) use self::entity_sparse_set::*;
pub(crate) use self::group::*;
pub(crate) use self::group_mask::*;

use rustc_hash::FxHashMap;
use std::mem;

/// Storage for entities and components.
#[derive(Default, Debug)]
pub struct EntityStorage {
    allocator: EntityAllocator,
    entities: EntitySparseSet,
    components: ComponentStorage,
}

impl EntityStorage {
    /// Creates a new `EntityStorage` with the given `GroupLayout`.
    #[inline]
    #[must_use]
    pub fn new(layout: &GroupLayout) -> Self {
        let components = unsafe { ComponentStorage::new(&[], layout, FxHashMap::default()) };

        Self {
            allocator: EntityAllocator::new(),
            entities: EntitySparseSet::new(),
            components,
        }
    }

    /// Sets a new `GroupLayout`.
    ///
    /// This function iterates over all entities in the storage, so it is best called when the
    /// storage is empty.
    #[inline]
    pub fn set_layout(&mut self, layout: &GroupLayout) {
        let sparse_sets = mem::take(&mut self.components).into_sparse_sets();

        self.components =
            unsafe { ComponentStorage::new(self.entities.as_slice(), layout, sparse_sets) };
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
        let entity = self.create_empty_entity();
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
        self.allocator
            .allocate_atomic()
            .expect("Failed to create a new Entity")
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

        self.allocator.recycle(entity);
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
        self.allocator.reset();
        self.entities.clear();
        self.components.clear();
    }

    /// Adds the entities allocated with [`create_atomic`](Self::create_atomic) to the storage.
    #[inline]
    pub fn maintain(&mut self) {
        self.allocator.maintain().for_each(|entity| {
            self.entities.insert(entity);
        });
    }

    /// Returns wether `entity` is present in the storage.
    #[inline]
    #[must_use]
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains(entity)
    }

    /// Returns all entities in the storage as a slice.
    #[inline]
    #[must_use]
    pub fn entities(&self) -> &[Entity] {
        self.entities.as_slice()
    }

    /// Borrows a view over all entities in the storage.
    ///
    /// This view supports the creation of new entities without requiring exclusive access to the
    /// storage.
    #[inline]
    #[must_use]
    pub fn borrow_entities(&self) -> Entities {
        Entities::new(self)
    }

    /// Borrows a shared view over all components of type `T` in the storage.
    #[must_use]
    pub fn borrow<T>(&self) -> Comp<T>
    where
        T: Component,
    {
        self.components.borrow::<T>()
    }

    /// Borrows an exclusive view over all components of type `T` in the storage.
    #[must_use]
    pub fn borrow_mut<T>(&self) -> CompMut<T>
    where
        T: Component,
    {
        self.components.borrow_mut::<T>()
    }

    #[inline]
    #[must_use]
    fn create_empty_entity(&mut self) -> Entity {
        let entity = self
            .allocator
            .allocate()
            .expect("Failed to create a new Entity");

        self.entities.insert(entity);
        entity
    }
}
