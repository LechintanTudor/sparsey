use crate::components::GroupInfo;
use crate::storage::{ComponentStorage, Entity, SparseArray};
use atomic_refcell::{AtomicRef, AtomicRefMut};
use std::marker::PhantomData;

pub struct Comp<'a, T> {
    storage: AtomicRef<'a, ComponentStorage>,
    group_info: Option<GroupInfo<'a>>,
    _phantom: PhantomData<&'a [T]>,
}

impl<'a, T> Comp<'a, T> {
    pub(crate) unsafe fn new(
        storage: AtomicRef<'a, ComponentStorage>,
        group_info: Option<GroupInfo<'a>>,
    ) -> Self {
        Self { storage, group_info, _phantom: PhantomData }
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        unsafe { self.storage.get::<T>(entity) }
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.storage.contains(entity)
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    pub fn components(&self) -> &[T] {
        unsafe { self.storage.components::<T>() }
    }

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

pub struct CompMut<'a, T> {
    storage: AtomicRefMut<'a, ComponentStorage>,
    group_info: Option<GroupInfo<'a>>,
    _phantom: PhantomData<&'a mut [T]>,
}

impl<'a, T> CompMut<'a, T> {
    pub(crate) unsafe fn new(
        storage: AtomicRefMut<'a, ComponentStorage>,
        group_info: Option<GroupInfo<'a>>,
    ) -> Self {
        Self { storage, group_info, _phantom: PhantomData }
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        unsafe { self.storage.get::<T>(entity) }
    }

    pub fn get_mut(&self, entity: Entity) -> Option<&mut T> {
        unsafe { self.storage.get_mut::<T>(entity) }
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.storage.contains(entity)
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    pub fn components(&self) -> &[T] {
        unsafe { self.storage.components::<T>() }
    }

    pub fn components_mut(&mut self) -> &mut [T] {
        unsafe { self.storage.components_mut::<T>() }
    }

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
