use crate::storage::{Component, Entity, SparseArray};
use crate::world::{Comp, CompMut};

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

    fn split_for_iteration<'a>(self) -> (&'a SparseArray, &'a [Entity], Self::Ptr)
    where
        Self: 'a;

    unsafe fn get_from_ptr<'a>(ptr: Self::Ptr, index: usize) -> Self::Ref<'a>
    where
        Self: 'a;
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

    fn split_for_iteration<'a>(self) -> (&'a SparseArray, &'a [Entity], Self::Ptr)
    where
        Self: 'a,
    {
        let (sparse, dense, slice) = Comp::split(self);
        (dense, sparse, slice.as_ptr())
    }

    unsafe fn get_from_ptr<'a>(ptr: Self::Ptr, index: usize) -> Self::Ref<'a>
    where
        Self: 'a,
    {
        &*ptr.add(index)
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

    fn split_for_iteration<'a>(self) -> (&'a SparseArray, &'a [Entity], Self::Ptr)
    where
        Self: 'a,
    {
        let (sparse, dense, slice) = CompMut::split(self);
        (dense, sparse, slice.as_ptr())
    }

    unsafe fn get_from_ptr<'a>(ptr: Self::Ptr, index: usize) -> Self::Ref<'a>
    where
        Self: 'a,
    {
        &*ptr.add(index)
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

    fn split_for_iteration<'a>(self) -> (&'a SparseArray, &'a [Entity], Self::Ptr)
    where
        Self: 'a,
    {
        let (sparse, dense, slice) = CompMut::split_mut(self);
        (dense, sparse, slice.as_mut_ptr())
    }

    unsafe fn get_from_ptr<'a>(ptr: Self::Ptr, index: usize) -> Self::Ref<'a>
    where
        Self: 'a,
    {
        &mut *ptr.add(index)
    }
}
