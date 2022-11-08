use crate::components::GroupInfo;
use crate::storage::{Component, Entity, SparseArray};
use crate::world::{Comp, CompMut};
use std::ops::RangeBounds;

#[doc(hidden)]
#[allow(clippy::len_without_is_empty)]
pub unsafe trait ComponentView<'a> {
    type Item: 'a;
    type Component: Component;
    type ComponentSlice: 'a;

    fn len(&self) -> usize;

    fn group_info(&self) -> Option<GroupInfo<'a>>;

    fn get(self, entity: Entity) -> Option<Self::Item>;

    fn contains(self, entity: Entity) -> bool;

    fn split(self) -> (&'a [Entity], &'a SparseArray, *mut Self::Component);

    unsafe fn get_from_component_ptr(component: *mut Self::Component) -> Self::Item;

    unsafe fn get_entities_unchecked<R>(self, range: R) -> &'a [Entity]
    where
        R: RangeBounds<usize>;

    unsafe fn get_components_unchecked<R>(self, range: R) -> Self::ComponentSlice
    where
        R: RangeBounds<usize>;

    unsafe fn get_entities_components_unchecked<R>(
        self,
        range: R,
    ) -> (&'a [Entity], Self::ComponentSlice)
    where
        R: RangeBounds<usize>;
}

macro_rules! impl_shared_component_view {
    ($ty:ident) => {
        unsafe impl<'a, T> ComponentView<'a> for &'a $ty<'a, T>
        where
            T: Component,
        {
            type Item = &'a T;
            type Component = T;
            type ComponentSlice = &'a [T];

            fn len(&self) -> usize {
                $ty::len(self)
            }

            fn group_info(&self) -> Option<GroupInfo<'a>> {
                $ty::group_info(self)
            }

            fn get(self, entity: Entity) -> Option<Self::Item> {
                $ty::get(self, entity)
            }

            fn contains(self, entity: Entity) -> bool {
                $ty::contains(self, entity)
            }

            fn split(self) -> (&'a [Entity], &'a SparseArray, *mut Self::Component) {
                let (entities, sparse, components) = $ty::split(self);
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
                $ty::entities(self).get_unchecked(bounds)
            }

            unsafe fn get_components_unchecked<R>(self, range: R) -> Self::ComponentSlice
            where
                R: RangeBounds<usize>,
            {
                let bounds = (range.start_bound().cloned(), range.end_bound().cloned());
                $ty::components(self).get_unchecked(bounds)
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
                    $ty::entities(self).get_unchecked(bounds),
                    $ty::components(self).get_unchecked(bounds),
                )
            }
        }
    };
}

impl_shared_component_view!(Comp);
impl_shared_component_view!(CompMut);

unsafe impl<'a, 'b, T> ComponentView<'a> for &'a mut CompMut<'b, T>
where
    T: Component,
{
    type Item = &'a mut T;
    type Component = T;
    type ComponentSlice = &'a mut [T];

    fn len(&self) -> usize {
        CompMut::len(self)
    }

    fn group_info(&self) -> Option<GroupInfo<'a>> {
        CompMut::group_info(self)
    }

    fn get(self, entity: Entity) -> Option<Self::Item> {
        CompMut::get_mut(self, entity)
    }

    fn contains(self, entity: Entity) -> bool {
        CompMut::contains(self, entity)
    }

    fn split(self) -> (&'a [Entity], &'a SparseArray, *mut Self::Component) {
        let (entities, sparse, components) = CompMut::split_mut(self);
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
        CompMut::entities(self).get_unchecked(bounds)
    }

    unsafe fn get_components_unchecked<R>(self, range: R) -> Self::ComponentSlice
    where
        R: RangeBounds<usize>,
    {
        let bounds = (range.start_bound().cloned(), range.end_bound().cloned());
        CompMut::components_mut(self).get_unchecked_mut(bounds)
    }

    unsafe fn get_entities_components_unchecked<R>(
        self,
        range: R,
    ) -> (&'a [Entity], Self::ComponentSlice)
    where
        R: RangeBounds<usize>,
    {
        let (entities, _, components) = CompMut::split_mut(self);
        let bounds = (range.start_bound().cloned(), range.end_bound().cloned());

        (entities.get_unchecked(bounds), components.get_unchecked_mut(bounds))
    }
}
