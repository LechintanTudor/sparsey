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
    pub fn get(&self, entity: Entity) -> Option<&T> {
        let value = self.storage.get(entity);

        if !value.is_null() {
            unsafe { Some(&*value.cast::<T>()) }
        } else {
            None
        }
    }

    /// Returns the `ChangeTicks` of the component of `entity`, if it was found
    /// in the storage.
    pub fn get_ticks(&self, entity: Entity) -> Option<&ChangeTicks> {
        self.storage.get_ticks(entity)
    }

    /// Returns the component and `ChangeTicks` of `entity`, if it was found in
    /// the storage,
    pub fn get_with_ticks(&self, entity: Entity) -> Option<(&T, &ChangeTicks)> {
        self.storage
            .get_with_ticks(entity)
            .map(|(value, ticks)| unsafe { (&*value.cast::<T>(), ticks) })
    }

    /// Returns `true` if `entity` was found in the storage.
    pub fn contains(&self, entity: Entity) -> bool {
        self.storage.contains(entity)
    }

    /// Returns all entities in the storage as a slice.
    pub fn entities(&self) -> &[Entity] {
        self.storage.entities()
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

    /// Returns all components in the storage as a slice.
    pub fn components(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.storage.components().cast::<T>(), self.storage.len()) }
    }

    /// Returns all `ChangeTicks` in the storage as a slice.
    pub fn ticks(&self) -> &[ChangeTicks] {
        self.storage.ticks()
    }

    /// Splits the storage into slices.
    pub fn split(&self) -> (SparseArrayView, &[Entity], &[T], &[ChangeTicks]) {
        let (sparse, entities, components, ticks) = self.storage.split();
        let components = unsafe { slice::from_raw_parts(components.cast::<T>(), entities.len()) };
        (sparse, entities, components, ticks)
    }
}

impl<T, S> Deref for TypedComponentStorage<T, S>
where
    S: Deref<Target = ComponentStorage>,
{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.storage.components().cast::<T>(), self.storage.len()) }
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

            if !prev.is_null() {
                Some(ptr::read(prev.cast::<T>()))
            } else {
                None
            }
        }
    }

    /// Removes `entity` from the storage and returns its component, if it
    /// exists.
    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        let value = self.storage.remove_and_forget(entity);

        if !value.is_null() {
            unsafe { Some(ptr::read(value.cast::<T>())) }
        } else {
            None
        }
    }

    /// Removes all entities, components and `ChangeTicks` from the storage.
    pub fn clear(&mut self) {
        self.storage.clear();
    }

    /// Returns the component and `ChangeTicks` of `entity`, if it was found in
    /// the storage,
    pub fn get_with_ticks_mut(&mut self, entity: Entity) -> Option<(&mut T, &mut ChangeTicks)> {
        self.storage
            .get_with_ticks_mut(entity)
            .map(|(value, ticks)| unsafe { (&mut *value.cast::<T>(), ticks) })
    }

    /// Splits the storage into slices.
    pub fn split_mut(&mut self) -> (SparseArrayView, &[Entity], &mut [T], &mut [ChangeTicks]) {
        let (sparse, entities, components, ticks) = self.storage.split_mut();
        let components = unsafe { slice::from_raw_parts_mut(components as *mut T, entities.len()) };
        (sparse, entities, components, ticks)
    }
}
