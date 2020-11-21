use crate::{
    atomic_ref_cell::{Ref, RefMut},
    data::view::*,
    entity::Entity,
    registry::{Component, World},
    storage::{SparseArray, SparseSet},
};

pub trait BorrowFromWorld<'a> {
    fn borrow(world: &'a World) -> Self;
}

pub struct Comp<'a, T> {
    set: Ref<'a, SparseSet<T>>,
}

impl<'a, T> BorrowFromWorld<'a> for Comp<'a, T>
where
    T: Component,
{
    fn borrow(world: &'a World) -> Self {
        Self {
            set: world.borrow().unwrap(),
        }
    }
}

impl<'a, T> StorageView<'a> for &'a Comp<'a, T> {
    const STRICT: bool = true;
    type Output = &'a T;
    type Component = &'a T;
    type Data = *const T;

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data) {
        <&'a SparseSet<T> as StorageView<'a>>::split(&self.set)
    }

    unsafe fn get_component(data: Self::Data, entity: Entity) -> Self::Component {
        <&'a SparseSet<T> as StorageView<'a>>::get_component(data, entity)
    }

    unsafe fn get_from_component(component: Option<Self::Component>) -> Option<Self::Output> {
        <&'a SparseSet<T> as StorageView<'a>>::get_from_component(component)
    }

    unsafe fn get(self, entity: Entity) -> Option<Self::Output> {
        <&'a SparseSet<T> as StorageView<'a>>::get(&self.set, entity)
    }
}

pub struct CompMut<'a, T> {
    set: RefMut<'a, SparseSet<T>>,
}

impl<'a, T> StorageView<'a> for &'a CompMut<'a, T> {
    const STRICT: bool = true;
    type Output = &'a T;
    type Component = &'a T;
    type Data = *const T;

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data) {
        <&'a SparseSet<T> as StorageView<'a>>::split(&self.set)
    }

    unsafe fn get_component(data: Self::Data, entity: Entity) -> Self::Component {
        <&'a SparseSet<T> as StorageView<'a>>::get_component(data, entity)
    }

    unsafe fn get_from_component(component: Option<Self::Component>) -> Option<Self::Output> {
        <&'a SparseSet<T> as StorageView<'a>>::get_from_component(component)
    }

    unsafe fn get(self, entity: Entity) -> Option<Self::Output> {
        <&'a SparseSet<T> as StorageView<'a>>::get(&self.set, entity)
    }
}

impl<'a, 'b, T> StorageView<'a> for &'a mut CompMut<'b, T>
where
    'b: 'a,
{
    const STRICT: bool = true;
    type Output = &'a mut T;
    type Component = &'a mut T;
    type Data = *mut T;

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data) {
        let set = &mut *self.set as *mut _;
        drop(self);

        <&'a mut SparseSet<T> as StorageView<'a>>::split(&mut *set)
    }

    unsafe fn get_component(data: Self::Data, entity: Entity) -> Self::Component {
        <&'a mut SparseSet<T> as StorageView<'a>>::get_component(data, entity)
    }

    unsafe fn get_from_component(component: Option<Self::Component>) -> Option<Self::Output> {
        <&'a mut SparseSet<T> as StorageView<'a>>::get_from_component(component)
    }

    unsafe fn get(self, entity: Entity) -> Option<Self::Output> {
        let set = &mut *self.set as *mut _;
        drop(self);

        <&'a mut SparseSet<T> as StorageView<'a>>::get(&mut *set, entity)
    }
}

impl<'a, T> BorrowFromWorld<'a> for CompMut<'a, T>
where
    T: Component,
{
    fn borrow(world: &'a World) -> Self {
        Self {
            set: world.borrow_mut().unwrap(),
        }
    }
}

macro_rules! impl_borrow_from_world {
    ($($b:ident),+) => {
        impl<'a, $($b,)+> BorrowFromWorld<'a> for ($($b,)+)
        where
            $($b: BorrowFromWorld<'a>,)+
        {
            fn borrow(world: &'a World) -> Self {
                ($(<$b as BorrowFromWorld<'a>>::borrow(world),)+)
            }
        }
    };
}

impl_borrow_from_world!(A);
impl_borrow_from_world!(A, B);
impl_borrow_from_world!(A, B, C);
impl_borrow_from_world!(A, B, C, D);
impl_borrow_from_world!(A, B, C, D, E);
impl_borrow_from_world!(A, B, C, D, E, F);
impl_borrow_from_world!(A, B, C, D, E, F, G);
impl_borrow_from_world!(A, B, C, D, E, F, G, H);
