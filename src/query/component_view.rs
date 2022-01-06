use crate::components::{Component, GroupInfo};
use crate::query::SimpleQueryElement;
use crate::storage::{ComponentStorage, Entity, SparseArray};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub struct ComponentView<'a, T, S> {
    storage: S,
    group_info: Option<GroupInfo<'a>>,
    _phantom: PhantomData<*const T>,
}

impl<'a, T, S> ComponentView<'a, T, S>
where
    T: Component,
    S: Deref<Target = ComponentStorage>,
{
    pub(crate) unsafe fn new(storage: S, group_info: Option<GroupInfo<'a>>) -> Self {
        Self { storage, group_info, _phantom: PhantomData }
    }
}

unsafe impl<'a, T, S> SimpleQueryElement<'a> for &'a ComponentView<'a, T, S>
where
    T: Component,
    S: Deref<Target = ComponentStorage>,
{
    type Item = &'a T;
    type Component = T;

    fn group_info(&self) -> Option<GroupInfo<'a>> {
        self.group_info.clone()
    }

    fn get(self, entity: Entity) -> Option<Self::Item> {
        unsafe { self.storage.get(entity) }
    }

    fn contains(self, entity: Entity) -> bool {
        self.storage.contains(entity)
    }

    fn split(self) -> (&'a [Entity], &'a SparseArray, *mut Self::Component) {
        self.storage.split()
    }

    unsafe fn get_from_components_unchecked(
        components: *mut Self::Component,
        index: usize,
    ) -> Self::Item {
        &*components.add(index)
    }
}

unsafe impl<'a, 'b, T, S> SimpleQueryElement<'a> for &'a mut ComponentView<'b, T, S>
where
    T: Component,
    S: Deref<Target = ComponentStorage> + DerefMut,
{
    type Item = &'a mut T;
    type Component = T;

    fn group_info(&self) -> Option<GroupInfo<'a>> {
        self.group_info.clone()
    }

    fn get(self, entity: Entity) -> Option<Self::Item> {
        unsafe { self.storage.get_mut(entity) }
    }

    fn contains(self, entity: Entity) -> bool {
        self.storage.contains(entity)
    }

    fn split(self) -> (&'a [Entity], &'a SparseArray, *mut Self::Component) {
        self.storage.split()
    }

    unsafe fn get_from_components_unchecked(
        components: *mut Self::Component,
        index: usize,
    ) -> Self::Item {
        &mut *components.add(index)
    }
}
