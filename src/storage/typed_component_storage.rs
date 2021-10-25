use crate::storage::{ComponentStorage, Entity, EntitySparseArray};
use crate::utils::ChangeTicks;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/// Wrapper around a `ComponentStorage` which strongly types it.
pub struct TypedComponentStorage<T, S> {
    storage: S,
    _phantom: PhantomData<*const T>,
}

impl<T, S> TypedComponentStorage<T, S>
where
    T: 'static,
    S: Deref<Target = ComponentStorage>,
{
    /// Creates a new `TypedComponentStorage` from the given storage.
    /// The storage must have been created for storing components of type `T`.
    pub(crate) unsafe fn new(storage: S) -> Self {
        Self {
            storage,
            _phantom: PhantomData,
        }
    }

    /// Returns the component of `entity`, if it was found in the storage.
    pub fn get(&self, entity: Entity) -> Option<&T> {
        unsafe { self.storage.get(entity) }
    }

    /// Returns the `ChangeTicks` of the component of `entity`, if it was found
    /// in the storage.
    pub fn get_ticks(&self, entity: Entity) -> Option<&ChangeTicks> {
        self.storage.get_ticks(entity)
    }

    /// Returns the component and `ChangeTicks` of `entity`, if it was found in
    /// the storage.
    pub fn get_with_ticks(&self, entity: Entity) -> Option<(&T, &ChangeTicks)> {
        unsafe { self.storage.get_with_ticks(entity) }
    }

    /// Returns `true` if `entity` was found in the storage.
    pub fn contains(&self, entity: Entity) -> bool {
        self.storage.contains(entity)
    }

    /// Returns the number of entities in the storage.
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    /// Returns `true` if the storage is empty.
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    /// Returns the number of components that can be stored without
    /// reallocating.
    pub fn capacity(&self) -> usize {
        self.storage.capacity()
    }

    /// Returns all entities in the storage as a slice.
    pub fn entities(&self) -> &[Entity] {
        self.storage.entities()
    }

    /// Returns all components in the storage as a slice.
    pub fn components(&self) -> &[T] {
        unsafe { self.storage.components::<T>() }
    }

    /// Returns all `ChangeTicks` in the storage as a slice.
    pub fn ticks(&self) -> &[ChangeTicks] {
        self.storage.ticks()
    }

    /// Returns a reference to the inner `ComponentStorage`.
    pub(crate) fn storage(&self) -> &ComponentStorage {
        &self.storage
    }

    /// Splits the storage for iteration.
    pub(crate) fn split_for_iteration(
        &self,
    ) -> (&EntitySparseArray, &[Entity], *const T, *const ChangeTicks) {
        self.storage.split_for_iteration::<T>()
    }
}

impl<T, S> TypedComponentStorage<T, S>
where
    T: 'static,
    S: Deref<Target = ComponentStorage> + DerefMut,
{
    /// Inserts a component into the storage and returns the previous one, if
    /// any.
    pub(crate) fn insert(&mut self, entity: Entity, component: T, ticks: ChangeTicks) -> Option<T> {
        unsafe { self.storage.insert(entity, component, ticks) }
    }

    /// Removes `entity` from the storage and returns its component, if it
    /// exists.
    pub(crate) fn remove(&mut self, entity: Entity) -> Option<T> {
        unsafe { self.storage.remove::<T>(entity) }
    }

    /// Removes all entities, components and `ChangeTicks` from the storage.
    #[allow(dead_code)]
    pub(crate) fn clear(&mut self) {
        self.storage.clear();
    }

    /// Returns the component and `ChangeTicks` of `entity`, if it was found in
    /// the storage.
    pub(crate) fn get_with_ticks_mut(
        &mut self,
        entity: Entity,
    ) -> Option<(&mut T, &mut ChangeTicks)> {
        unsafe { self.storage.get_with_ticks_mut(entity) }
    }

    /// Splits the storage for iteration.
    pub(crate) fn split_for_iteration_mut(
        &mut self,
    ) -> (&EntitySparseArray, &[Entity], *mut T, *mut ChangeTicks) {
        self.storage.split_for_iteration_mut::<T>()
    }
}
