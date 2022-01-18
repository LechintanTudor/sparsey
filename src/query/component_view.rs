use crate::components::{Component, GroupInfo};
use crate::query::QueryElement;
use crate::storage::{ComponentStorage, Entity, SparseArray};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, RangeBounds};

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

unsafe impl<'a, T, S> QueryElement<'a> for &'a ComponentView<'a, T, S>
where
    T: Component,
    S: Deref<Target = ComponentStorage>,
{
    type Item = &'a T;
    type Component = T;
    type ComponentSlice = &'a [T];

    fn len(&self) -> usize {
        self.storage.len()
    }

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
        let (entities, sparse, components) = unsafe { self.storage.split::<T>() };
        (entities, sparse, components.as_ptr() as *mut _)
    }

    unsafe fn get_from_component_ptr(component: *mut Self::Component) -> Self::Item {
        &*component
    }

    unsafe fn get_entities_unchecked<R>(self, range: R) -> &'a [Entity]
    where
        R: RangeBounds<usize>,
    {
        let bounds = (range.start_bound().cloned(), range.end_bound().cloned());
        self.storage.entities().get_unchecked(bounds)
    }

    unsafe fn get_components_unchecked<R>(self, range: R) -> Self::ComponentSlice
    where
        R: RangeBounds<usize>,
    {
        let bounds = (range.start_bound().cloned(), range.end_bound().cloned());
        self.storage.components::<T>().get_unchecked(bounds)
    }

    unsafe fn get_entities_components_unchecked<R>(
        self,
        range: R,
    ) -> (&'a [Entity], Self::ComponentSlice)
    where
        R: RangeBounds<usize>,
    {
        let bounds = (range.start_bound().cloned(), range.end_bound().cloned());

        (
            self.storage.entities().get_unchecked(bounds),
            self.storage.components::<T>().get_unchecked(bounds),
        )
    }
}

unsafe impl<'a, 'b, T, S> QueryElement<'a> for &'a mut ComponentView<'b, T, S>
where
    T: Component,
    S: Deref<Target = ComponentStorage> + DerefMut,
{
    type Item = &'a mut T;
    type Component = T;
    type ComponentSlice = &'a mut [T];

    fn len(&self) -> usize {
        self.storage.len()
    }

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
        let (entities, sparse, components) = unsafe { self.storage.split_mut() };
        (entities, sparse, components.as_mut_ptr())
    }

    unsafe fn get_from_component_ptr(component: *mut Self::Component) -> Self::Item {
        &mut *component
    }

    unsafe fn get_entities_unchecked<R>(self, range: R) -> &'a [Entity]
    where
        R: RangeBounds<usize>,
    {
        let bounds = (range.start_bound().cloned(), range.end_bound().cloned());
        self.storage.entities().get_unchecked(bounds)
    }

    unsafe fn get_components_unchecked<R>(self, range: R) -> Self::ComponentSlice
    where
        R: RangeBounds<usize>,
    {
        let bounds = (range.start_bound().cloned(), range.end_bound().cloned());
        self.storage.components_mut::<T>().get_unchecked_mut(bounds)
    }

    unsafe fn get_entities_components_unchecked<R>(
        self,
        range: R,
    ) -> (&'a [Entity], Self::ComponentSlice)
    where
        R: RangeBounds<usize>,
    {
        let (entities, _, components) = self.storage.split_mut::<T>();
        let bounds = (range.start_bound().cloned(), range.end_bound().cloned());

        (entities.get_unchecked(bounds), components.get_unchecked_mut(bounds))
    }
}
