use crate::storage::{ComponentStorage, Entity, SparseArrayView};
use crate::utils::ChangeTicks;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::{mem, ptr, slice};

/// Wrapper around a `ComponentStorage` which strongly types it.
pub struct TypedComponentStorage<T, S> {
    storage: S,
    component: PhantomData<*const T>,
}

impl<T, S> TypedComponentStorage<T, S>
where
    S: Deref<Target = ComponentStorage>,
{
    /// Creates a new `TypedComponentStorage` from the given storage.
    /// The storage must have been created for storing components of type `T`.
    pub unsafe fn new(storage: S) -> Self {
        Self {
            storage,
            component: PhantomData,
        }
    }

    /// Returns the component of `entity`, if it was found in the storage.
    #[inline]
    pub fn get(&self, entity: Entity) -> Option<&T> {
        unsafe { self.storage.get(entity).map(|v| &*v.as_ptr().cast::<T>()) }
    }

    /// Returns the `ChangeTicks` of the component of `entity`, if it was found
    /// in the storage.
    #[inline]
    pub fn get_ticks(&self, entity: Entity) -> Option<&ChangeTicks> {
        self.storage.get_ticks(entity)
    }

    /// Returns the component and `ChangeTicks` of `entity`, if it was found in
    /// the storage.
    #[inline]
    pub fn get_with_ticks(&self, entity: Entity) -> Option<(&T, &ChangeTicks)> {
        self.storage
            .get_with_ticks(entity)
            .map(|(value, ticks)| unsafe { (&*value.as_ptr().cast::<T>(), ticks) })
    }

    /// Returns `true` if `entity` was found in the storage.
    #[inline]
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
        unsafe { slice::from_raw_parts(self.storage.components().cast::<T>(), self.storage.len()) }
    }

    /// Returns all `ChangeTicks` in the storage as a slice.
    pub fn ticks(&self) -> &[ChangeTicks] {
        self.storage.ticks()
    }

    /// Splits the storage for iteration.
    pub fn split_for_iteration(
        &self,
    ) -> (SparseArrayView, &[Entity], *const T, *const ChangeTicks) {
        let (sparse, entities, components, ticks) = self.storage.split_for_iteration();
        (sparse, entities, components.cast::<T>(), ticks)
    }
}

impl<T, S> TypedComponentStorage<T, S>
where
    S: Deref<Target = ComponentStorage> + DerefMut,
{
    /// Inserts a value into the storage and returns the previous one, if any.
    pub fn insert(&mut self, entity: Entity, value: T, ticks: ChangeTicks) -> Option<T> {
        unsafe {
            let raw_value = &value as *const _ as *const _;
            let prev = self
                .storage
                .insert_and_forget_prev(entity, raw_value, ticks);
            mem::forget(value);

            prev.map(|v| ptr::read(v.as_ptr().cast::<T>()))
        }
    }

    /// Removes `entity` from the storage and returns its component, if it
    /// exists.
    #[must_use = "use `delete` to discard the component"]
    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        unsafe {
            self.storage
                .remove_and_forget(entity)
                .map(|v| ptr::read(v.as_ptr().cast::<T>()))
        }
    }

    // Removes `entity` from the storage if it exists. This is faster than removing
    // it.
    #[inline]
    pub fn delete(&mut self, entity: Entity) {
        self.storage.remove_and_drop(entity)
    }

    /// Removes all entities, components and `ChangeTicks` from the storage.
    pub fn clear(&mut self) {
        self.storage.clear();
    }

    /// Returns the component and `ChangeTicks` of `entity`, if it was found in
    /// the storage.
    #[inline]
    pub fn get_with_ticks_mut(&mut self, entity: Entity) -> Option<(&mut T, &mut ChangeTicks)> {
        self.storage
            .get_with_ticks_mut(entity)
            .map(|(value, ticks)| unsafe { (&mut *value.as_ptr().cast::<T>(), ticks) })
    }

    /// Splits the storage for iteration.
    pub fn split_for_iteration_mut(
        &mut self,
    ) -> (SparseArrayView, &[Entity], *mut T, *mut ChangeTicks) {
        let (sparse, entities, components, ticks) = self.storage.split_for_iteration_mut();
        (sparse, entities, components.cast::<T>(), ticks)
    }
}
