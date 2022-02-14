use crate::components::GroupInfo;
use crate::storage::{Component, ComponentStorage, Entity, SparseArray};
use atomic_refcell::{AtomicRef, AtomicRefMut};
use std::marker::PhantomData;

/// Immutable view over all components of type `T` in a world.
pub struct Comp<'a, T> {
    storage: AtomicRef<'a, ComponentStorage>,
    group_info: Option<GroupInfo<'a>>,
    _phantom: PhantomData<&'a [T]>,
}

impl<'a, T> Comp<'a, T>
where
    T: Component,
{
    pub(crate) unsafe fn new(
        storage: AtomicRef<'a, ComponentStorage>,
        group_info: Option<GroupInfo<'a>>,
    ) -> Self {
        Self { storage, group_info, _phantom: PhantomData }
    }

    /// Returns the component mapped to `entity` if it exists.
    pub fn get(&self, entity: Entity) -> Option<&T> {
        unsafe { self.storage.get::<T>(entity) }
    }

    /// Returns `true` if the view contains `entity`.
    pub fn contains(&self, entity: Entity) -> bool {
        self.storage.contains(entity)
    }

    /// Returns the number of components in the view.
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    /// Returns `true` if the view contains no components.
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    /// Returns all components in the view as a slice.
    pub fn components(&self) -> &[T] {
        unsafe { self.storage.components::<T>() }
    }

    /// Returns all entities in the view as a slice.
    pub fn entities(&self) -> &[Entity] {
        self.storage.entities()
    }

    pub(crate) fn group_info(&self) -> Option<GroupInfo<'a>> {
        self.group_info
    }

    pub(crate) fn split(&self) -> (&[Entity], &SparseArray, &[T]) {
        unsafe { self.storage.split::<T>() }
    }
}

/// Mutable view over all components of type `T` in a world.
pub struct CompMut<'a, T> {
    storage: AtomicRefMut<'a, ComponentStorage>,
    group_info: Option<GroupInfo<'a>>,
    _phantom: PhantomData<&'a mut [T]>,
}

impl<'a, T> CompMut<'a, T>
where
    T: Component,
{
    pub(crate) unsafe fn new(
        storage: AtomicRefMut<'a, ComponentStorage>,
        group_info: Option<GroupInfo<'a>>,
    ) -> Self {
        Self { storage, group_info, _phantom: PhantomData }
    }

    /// Returns the component mapped to `entity`, if it exists.
    pub fn get(&self, entity: Entity) -> Option<&T> {
        unsafe { self.storage.get::<T>(entity) }
    }

    /// Mutably returns the component mapped to `entity` if it exists.
    pub fn get_mut(&self, entity: Entity) -> Option<&mut T> {
        unsafe { self.storage.get_mut::<T>(entity) }
    }

    /// Returns `true` if the view contains `entity`.
    pub fn contains(&self, entity: Entity) -> bool {
        self.storage.contains(entity)
    }

    /// Returns the number of components in the view.
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    /// Returns `true` if the view contains no components.
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    /// Returns all commponents in the view as a slice.
    pub fn components(&self) -> &[T] {
        unsafe { self.storage.components::<T>() }
    }

    /// Returns all components in the view as a mutable slice.
    pub fn components_mut(&mut self) -> &mut [T] {
        unsafe { self.storage.components_mut::<T>() }
    }

    /// Returns all entities in the view as a slice.
    pub fn entities(&self) -> &[Entity] {
        self.storage.entities()
    }

    pub(crate) fn group_info(&self) -> Option<GroupInfo<'a>> {
        self.group_info
    }

    pub(crate) fn split(&self) -> (&[Entity], &SparseArray, &[T]) {
        unsafe { self.storage.split::<T>() }
    }

    pub(crate) fn split_mut(&mut self) -> (&[Entity], &SparseArray, &mut [T]) {
        unsafe { self.storage.split_mut::<T>() }
    }
}
