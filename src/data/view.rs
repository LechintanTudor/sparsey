use crate::{entity::Entity, storage::SparseArray};
use std::marker::PhantomData;

pub trait StorageView<'a> {
    const STRICT: bool;
    type Output: 'a;
    type Component: 'a;
    type Data: 'a + Copy;

    unsafe fn split_for_iteration(self) -> (&'a SparseArray, &'a [Entity], Self::Data);

    unsafe fn get_component(data: Self::Data, entity: Entity) -> Self::Component;

    unsafe fn get_from_component(component: Option<Self::Component>) -> Option<Self::Output>;

    unsafe fn get_output(self, entity: Entity) -> Option<Self::Output>;
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

    unsafe fn split_for_iteration(self) -> (&'a SparseArray, &'a [Entity], Self::Data) {
        V::split_for_iteration(self.view)
    }

    unsafe fn get_component(data: Self::Data, entity: Entity) -> Self::Component {
        V::get_component(data, entity)
    }

    unsafe fn get_from_component(component: Option<Self::Component>) -> Option<Self::Output> {
        Some(V::get_from_component(component))
    }

    unsafe fn get_output(self, entity: Entity) -> Option<Self::Output> {
        Some(V::get_output(self.view, entity))
    }
}

pub struct Not<'a, V>
where
    V: StorageView<'a>,
{
    view: V,
    phantom: PhantomData<&'a ()>,
}

pub fn not<'a, V>(view: V) -> Not<'a, V>
where
    V: StorageView<'a>,
{
    Not {
        view,
        phantom: PhantomData,
    }
}

impl<'a, V> StorageView<'a> for Not<'a, V>
where
    V: StorageView<'a>,
{
    const STRICT: bool = false;
    type Output = Void<V::Output>;
    type Component = V::Component;
    type Data = V::Data;

    unsafe fn split_for_iteration(self) -> (&'a SparseArray, &'a [Entity], Self::Data) {
        V::split_for_iteration(self.view)
    }

    unsafe fn get_component(data: Self::Data, entity: Entity) -> Self::Component {
        V::get_component(data, entity)
    }

    unsafe fn get_from_component(component: Option<Self::Component>) -> Option<Self::Output> {
        if V::get_from_component(component).is_some() {
            None
        } else {
            Some(Void::default())
        }
    }

    unsafe fn get_output(self, entity: Entity) -> Option<Self::Output> {
        if V::get_output(self.view, entity).is_some() {
            None
        } else {
            Some(Void::default())
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Void<T> {
    phantom: PhantomData<T>,
}

impl<T> Default for Void<T> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}
