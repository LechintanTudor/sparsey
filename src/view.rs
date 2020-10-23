use crate::{Component, Ref, RefMut, Storage, World};
use std::marker::PhantomData;

pub trait FromWorld<'a>
where
    Self: 'a,
{
    fn from_world(world: &'a World) -> Self;
}

impl<'a, T> FromWorld<'a> for Ref<'a, Storage<T>>
where
    T: Component,
{
    fn from_world(world: &'a World) -> Self {
        world.borrow::<T>().unwrap()
    }
}

impl<'a, T> FromWorld<'a> for RefMut<'a, Storage<T>>
where
    T: Component,
{
    fn from_world(world: &'a World) -> Self {
        world.borrow_mut::<T>().unwrap()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ViewType {
    Read,
    Write,
    Maintain,
}

pub trait View<'a> {
    const TYPE: ViewType;
    type Component: Component;
    type Output: FromWorld<'a>;
}

/// Read components
pub struct Read<T>
where
    T: Component,
{
    _phantom: PhantomData<T>,
}

impl<'a, T> View<'a> for Read<T>
where
    T: Component,
{
    const TYPE: ViewType = ViewType::Read;
    type Component = T;
    type Output = Ref<'a, Storage<T>>;
}

/// Write components
pub struct Write<T>
where
    T: Component,
{
    _phantom: PhantomData<T>,
}

impl<'a, T> View<'a> for Write<T>
where
    T: Component,
{
    const TYPE: ViewType = ViewType::Write;
    type Component = T;
    type Output = RefMut<'a, Storage<T>>;
}

/// Insert/delete components
/// Locks whole group.
pub struct Maintain<T>
where
    T: Component,
{
    _phantom: PhantomData<T>,
}

impl<'a, T> View<'a> for Maintain<T>
where
    T: Component,
{
    const TYPE: ViewType = ViewType::Maintain;
    type Component = T;
    type Output = RefMut<'a, Storage<T>>;
}

pub trait SystemData<'a>
where
    Self: 'a,
{
    fn from_world2(world: &'a World) -> Self;
}

impl<'a, T> SystemData<'a> for T
where
    T: FromWorld<'a>,
{
    fn from_world2(world: &'a World) -> Self {
        T::from_world(world)
    }
}

impl<'a, T, U> SystemData<'a> for (T, U)
where
    T: FromWorld<'a>,
    U: FromWorld<'a>,
{
    fn from_world2(world: &'a World) -> Self {
        (T::from_world(world), U::from_world(world))
    }
}

impl<'a, T, U, V> SystemData<'a> for (T, U, V)
where
    T: FromWorld<'a>,
    U: FromWorld<'a>,
    V: FromWorld<'a>,
{
    fn from_world2(world: &'a World) -> Self {
        (
            T::from_world(world),
            U::from_world(world),
            V::from_world(world),
        )
    }
}
