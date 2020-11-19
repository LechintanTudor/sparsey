use crate::{
    entity::Entity,
    storage::{SparseArray, SparseSet, SparseSetLike},
};
use std::marker::PhantomData;

pub trait StorageView<'a> {
    const STRICT: bool;
    type Output: 'a;
    type Component: 'a;
    type Data: 'a + Copy;

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data);

    unsafe fn get_component(data: Self::Data, entity: Entity) -> Self::Component;

    unsafe fn get_from_component(component: Option<Self::Component>) -> Option<Self::Output>;

    unsafe fn get(self, entity: Entity) -> Option<Self::Output>;
}

impl<'a, T> StorageView<'a> for &'a SparseSet<T> {
    const STRICT: bool = true;
    type Output = &'a T;
    type Component = &'a T;
    type Data = *const T;

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data) {
        let (sparse, dense, data) = SparseSetLike::split(self);
        (sparse, dense, data.as_ptr())
    }

    unsafe fn get_component(data: Self::Data, entity: Entity) -> Self::Component {
        &*data.add(entity.index())
    }

    unsafe fn get_from_component(component: Option<Self::Component>) -> Option<Self::Output> {
        component
    }

    unsafe fn get(self, entity: Entity) -> Option<Self::Output> {
        SparseSetLike::get(self, entity)
    }
}

impl<'a, T> StorageView<'a> for &'a mut SparseSet<T> {
    const STRICT: bool = true;
    type Output = &'a mut T;
    type Component = &'a mut T;
    type Data = *mut T;

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data) {
        let (sparse, dense, data) = SparseSetLike::split_mut(self);
        (sparse, dense, data.as_mut_ptr())
    }

    unsafe fn get_component(data: Self::Data, entity: Entity) -> Self::Component {
        &mut *data.add(entity.index())
    }

    unsafe fn get_from_component(component: Option<Self::Component>) -> Option<Self::Output> {
        component
    }

    unsafe fn get(self, entity: Entity) -> Option<Self::Output> {
        SparseSetLike::get_mut(self, entity)
    }
}

pub struct Maybe<'a, V>
where
    V: StorageView<'a>,
{
    view: V,
    phantom: PhantomData<&'a ()>,
}

pub fn maybe<'a, V>(view: V) -> Maybe<'a, V>
where
    V: StorageView<'a>,
{
    Maybe {
        view,
        phantom: PhantomData,
    }
}

impl<'a, V> StorageView<'a> for Maybe<'a, V>
where
    V: StorageView<'a>,
{
    const STRICT: bool = false;
    type Output = Option<V::Output>;
    type Component = V::Component;
    type Data = V::Data;

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data) {
        V::split(self.view)
    }

    unsafe fn get_component(data: Self::Data, entity: Entity) -> Self::Component {
        V::get_component(data, entity)
    }

    unsafe fn get_from_component(component: Option<Self::Component>) -> Option<Self::Output> {
        Some(V::get_from_component(component))
    }

    unsafe fn get(self, entity: Entity) -> Option<Self::Output> {
        Some(V::get(self.view, entity))
    }
}
