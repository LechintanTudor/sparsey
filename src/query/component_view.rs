use crate::components::GroupInfo;
use crate::storage::{Component, Entity, SparseArray};
use crate::world::{Comp, CompMut};
use std::ops::RangeBounds;

/// Building block for queries.
#[allow(clippy::len_without_is_empty)]
pub trait ComponentView {
    /// Reference to the component returned by the view.
    type Ref<'a>
    where
        Self: 'a;

    /// Pointer to the component returned by the view.
    type Ptr: Copy;

    /// Slice of components returned by the view.
    type Slice<'a>
    where
        Self: 'a;

    /// Returns the component mapped to `entity` if it exists.
    fn get<'a>(self, entity: Entity) -> Option<Self::Ref<'a>>
    where
        Self: 'a;

    /// Returns whether the view contains `entity`.
    fn contains(self, entity: Entity) -> bool;

    /// Returns the number of entities in the view.
    fn len(&self) -> usize;

    /// Returns the gorup info of the view.
    fn group_info<'a>(&'a self) -> Option<GroupInfo<'a>>
    where
        Self: 'a;

    /// Splits the view for iteration.
    fn split_for_iteration<'a>(self) -> (&'a [Entity], &'a SparseArray, Self::Ptr)
    where
        Self: 'a;

    /// Applies an offsets to the given pointer.
    unsafe fn offset_ptr(ptr: Self::Ptr, offset: usize) -> Self::Ptr;

    /// Returns the component at the given `index`.
    unsafe fn get_from_ptr<'a>(ptr: Self::Ptr, index: usize) -> Self::Ref<'a>
    where
        Self: 'a;

    /// Returns a slice containing all entities in the given `range`.
    unsafe fn get_entities_unchecked<'a, R>(self, range: R) -> &'a [Entity]
    where
        Self: 'a,
        R: RangeBounds<usize>;

    /// Returns a slice containing all components in the given `range`.
    unsafe fn get_components_unchecked<'a, R>(self, range: R) -> Self::Slice<'a>
    where
        Self: 'a,
        R: RangeBounds<usize>;

    /// Returns all entities and components in the given `range` as slices.
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
