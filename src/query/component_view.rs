use crate::components::GroupInfo;
use crate::storage::{Component, Entity, SparseArray};
use crate::world::{Comp, CompMut};
use std::ops::RangeBounds;

#[allow(clippy::len_without_is_empty)]
pub trait ComponentView {
    type Ref<'a>
    where
        Self: 'a;

    type Ptr: Copy;

    type Slice<'a>
    where
        Self: 'a;

    fn get<'a>(self, entity: Entity) -> Option<Self::Ref<'a>>
    where
        Self: 'a;

    fn contains(self, entity: Entity) -> bool;

    fn len(&self) -> usize;

    fn group_info<'a>(&'a self) -> Option<GroupInfo<'a>>
    where
        Self: 'a;

    fn split_for_iteration<'a>(self) -> (&'a [Entity], &'a SparseArray, Self::Ptr)
    where
        Self: 'a;

    unsafe fn offset_ptr(ptr: Self::Ptr, offset: usize) -> Self::Ptr;

    unsafe fn get_from_ptr<'a>(ptr: Self::Ptr, index: usize) -> Self::Ref<'a>
    where
        Self: 'a;

    unsafe fn get_entities_unchecked<'a, R>(self, range: R) -> &'a [Entity]
    where
        Self: 'a,
        R: RangeBounds<usize>;

    unsafe fn get_components_unchecked<'a, R>(self, range: R) -> Self::Slice<'a>
    where
        Self: 'a,
        R: RangeBounds<usize>;

    unsafe fn get_entities_and_components_unchecked<'a, R>(
        self,
        range: R,
    ) -> (&'a [Entity], Self::Slice<'a>)
    where
        Self: 'a,
        R: RangeBounds<usize>;
}

impl<T> ComponentView for &'_ Comp<'_, T>
where
    T: Component,
{
    type Ref<'a> = &'a T where Self: 'a;
    type Ptr = *const T;
    type Slice<'a> = &'a [T] where Self: 'a;

    fn get<'a>(self, entity: Entity) -> Option<Self::Ref<'a>>
    where
        Self: 'a,
    {
        Comp::get(self, entity)
    }

    fn contains(self, entity: Entity) -> bool {
        Comp::contains(self, entity)
    }

    fn len(&self) -> usize {
        Comp::len(self)
    }

    fn group_info<'a>(&'a self) -> Option<GroupInfo<'a>>
    where
        Self: 'a,
    {
        Comp::group_info(self)
    }

    fn split_for_iteration<'a>(self) -> (&'a [Entity], &'a SparseArray, Self::Ptr)
    where
        Self: 'a,
    {
        let (entities, sparse, dense) = Comp::split(self);
        (entities, sparse, dense.as_ptr())
    }

    unsafe fn offset_ptr(ptr: Self::Ptr, offset: usize) -> Self::Ptr {
        ptr.add(offset)
    }

    unsafe fn get_from_ptr<'a>(ptr: Self::Ptr, index: usize) -> Self::Ref<'a>
    where
        Self: 'a,
    {
        &*ptr.add(index)
    }

    unsafe fn get_entities_unchecked<'a, R>(self, range: R) -> &'a [Entity]
    where
        Self: 'a,
        R: RangeBounds<usize>,
    {
        let range = (range.start_bound().cloned(), range.end_bound().cloned());
        Comp::entities(self).get_unchecked(range)
    }

    unsafe fn get_components_unchecked<'a, R>(self, range: R) -> Self::Slice<'a>
    where
        Self: 'a,
        R: RangeBounds<usize>,
    {
        let range = (range.start_bound().cloned(), range.end_bound().cloned());
        Comp::components(self).get_unchecked(range)
    }

    unsafe fn get_entities_and_components_unchecked<'a, R>(
        self,
        range: R,
    ) -> (&'a [Entity], Self::Slice<'a>)
    where
        Self: 'a,
        R: RangeBounds<usize>,
    {
        let range = (range.start_bound().cloned(), range.end_bound().cloned());
        let (entities, _, components) = Comp::split(self);
        (entities.get_unchecked(range), components.get_unchecked(range))
    }
}

impl<T> ComponentView for &'_ CompMut<'_, T>
where
    T: Component,
{
    type Ref<'a> = &'a T where Self: 'a;
    type Ptr = *const T;
    type Slice<'a> = &'a [T] where Self: 'a;

    fn get<'a>(self, entity: Entity) -> Option<Self::Ref<'a>>
    where
        Self: 'a,
    {
        CompMut::get(self, entity)
    }

    fn contains(self, entity: Entity) -> bool {
        CompMut::contains(self, entity)
    }

    fn len(&self) -> usize {
        CompMut::len(self)
    }

    fn group_info<'a>(&'a self) -> Option<GroupInfo<'a>>
    where
        Self: 'a,
    {
        CompMut::group_info(self)
    }

    fn split_for_iteration<'a>(self) -> (&'a [Entity], &'a SparseArray, Self::Ptr)
    where
        Self: 'a,
    {
        let (entities, sparse, dense) = CompMut::split(self);
        (entities, sparse, dense.as_ptr())
    }

    unsafe fn offset_ptr(ptr: Self::Ptr, offset: usize) -> Self::Ptr {
        ptr.add(offset)
    }

    unsafe fn get_from_ptr<'a>(ptr: Self::Ptr, index: usize) -> Self::Ref<'a>
    where
        Self: 'a,
    {
        &*ptr.add(index)
    }

    unsafe fn get_entities_unchecked<'a, R>(self, range: R) -> &'a [Entity]
    where
        Self: 'a,
        R: RangeBounds<usize>,
    {
        let range = (range.start_bound().cloned(), range.end_bound().cloned());
        CompMut::entities(self).get_unchecked(range)
    }

    unsafe fn get_components_unchecked<'a, R>(self, range: R) -> Self::Slice<'a>
    where
        Self: 'a,
        R: RangeBounds<usize>,
    {
        let range = (range.start_bound().cloned(), range.end_bound().cloned());
        CompMut::components(self).get_unchecked(range)
    }

    unsafe fn get_entities_and_components_unchecked<'a, R>(
        self,
        range: R,
    ) -> (&'a [Entity], Self::Slice<'a>)
    where
        Self: 'a,
        R: RangeBounds<usize>,
    {
        let range = (range.start_bound().cloned(), range.end_bound().cloned());
        let (entities, _, components) = CompMut::split(self);
        (entities.get_unchecked(range), components.get_unchecked(range))
    }
}

impl<T> ComponentView for &'_ mut CompMut<'_, T>
where
    T: Component,
{
    type Ref<'a> = &'a mut T where Self: 'a;
    type Ptr = *mut T;
    type Slice<'a> = &'a mut [T] where Self: 'a;

    fn get<'a>(self, entity: Entity) -> Option<Self::Ref<'a>>
    where
        Self: 'a,
    {
        CompMut::get_mut(self, entity)
    }

    fn contains(self, entity: Entity) -> bool {
        CompMut::contains(self, entity)
    }

    fn len(&self) -> usize {
        CompMut::len(self)
    }

    fn group_info<'a>(&'a self) -> Option<GroupInfo<'a>>
    where
        Self: 'a,
    {
        CompMut::group_info(self)
    }

    fn split_for_iteration<'a>(self) -> (&'a [Entity], &'a SparseArray, Self::Ptr)
    where
        Self: 'a,
    {
        let (dense, sparse, slice) = CompMut::split_mut(self);
        (dense, sparse, slice.as_mut_ptr())
    }

    unsafe fn offset_ptr(ptr: Self::Ptr, offset: usize) -> Self::Ptr {
        ptr.add(offset)
    }

    unsafe fn get_from_ptr<'a>(ptr: Self::Ptr, index: usize) -> Self::Ref<'a>
    where
        Self: 'a,
    {
        &mut *ptr.add(index)
    }

    unsafe fn get_entities_unchecked<'a, R>(self, range: R) -> &'a [Entity]
    where
        Self: 'a,
        R: RangeBounds<usize>,
    {
        let range = (range.start_bound().cloned(), range.end_bound().cloned());
        CompMut::entities(self).get_unchecked(range)
    }

    unsafe fn get_components_unchecked<'a, R>(self, range: R) -> Self::Slice<'a>
    where
        Self: 'a,
        R: RangeBounds<usize>,
    {
        let range = (range.start_bound().cloned(), range.end_bound().cloned());
        CompMut::components_mut(self).get_unchecked_mut(range)
    }

    unsafe fn get_entities_and_components_unchecked<'a, R>(
        self,
        range: R,
    ) -> (&'a [Entity], Self::Slice<'a>)
    where
        Self: 'a,
        R: RangeBounds<usize>,
    {
        let range = (range.start_bound().cloned(), range.end_bound().cloned());
        let (entities, _, components) = CompMut::split_mut(self);
        (entities.get_unchecked(range), components.get_unchecked_mut(range))
    }
}
