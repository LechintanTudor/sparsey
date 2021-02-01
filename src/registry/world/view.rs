use crate::data::IterableView;
use crate::registry::{BorrowWorld, BorrowWorldUnsafe, Component, Group, World};
use crate::storage::{ComponentFlags, ComponentRefMut, Entity, SparseArray, SparseSet};
use atomic_refcell::{AtomicRef, AtomicRefMut};
use std::ops::{Deref, DerefMut};

pub struct Comp<'a, T>
where
    T: 'static,
{
    set: AtomicRef<'a, SparseSet<T>>,
    group: Option<Group>,
}

impl<'a, T> Comp<'a, T> {
    pub(crate) unsafe fn new(set: AtomicRef<'a, SparseSet<T>>, group: Option<Group>) -> Self {
        Self { set, group }
    }
}

impl<'a, T> BorrowWorld<'a> for Comp<'a, T>
where
    T: Component,
{
    fn borrow_world(world: &'a World) -> Self {
        world.borrow_comp().unwrap()
    }
}

impl<'a, T> IterableView<'a> for &'a Comp<'a, T> {
    type Data = *const T;
    type Flags = *const ComponentFlags;
    type Output = &'a T;

    unsafe fn group(&self) -> Option<Group> {
        self.group
    }

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags) {
        let (sparse, dense, data, flags) = self.set.split();
        (sparse, dense, data.as_ptr(), flags.as_ptr())
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags {
        *flags.add(index)
    }

    unsafe fn get(data: Self::Data, _: Self::Flags, index: usize) -> Option<Self::Output> {
        Some(&*data.add(index))
    }
}

pub struct CompMut<'a, T>
where
    T: 'static,
{
    set: AtomicRefMut<'a, SparseSet<T>>,
    group: Option<Group>,
}

impl<'a, T> CompMut<'a, T> {
    pub(crate) unsafe fn new(set: AtomicRefMut<'a, SparseSet<T>>, group: Option<Group>) -> Self {
        Self { set, group }
    }
}

impl<'a, T> BorrowWorld<'a> for CompMut<'a, T>
where
    T: Component,
{
    fn borrow_world(world: &'a World) -> Self {
        world.borrow_comp_mut().unwrap()
    }
}

impl<'a, T> IterableView<'a> for &'a CompMut<'a, T> {
    type Data = *const T;
    type Flags = *const ComponentFlags;
    type Output = &'a T;

    unsafe fn group(&self) -> Option<Group> {
        self.group
    }

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags) {
        let (sparse, dense, data, flags) = self.set.split();
        (sparse, dense, data.as_ptr(), flags.as_ptr())
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags {
        *flags.add(index)
    }

    unsafe fn get(data: Self::Data, _: Self::Flags, index: usize) -> Option<Self::Output> {
        Some(&*data.add(index))
    }
}

impl<'a: 'b, 'b, T> IterableView<'b> for &'b mut CompMut<'a, T> {
    type Data = *mut T;
    type Flags = *mut ComponentFlags;
    type Output = ComponentRefMut<'b, T>;

    unsafe fn group(&self) -> Option<Group> {
        self.group
    }

    unsafe fn split(self) -> (&'b SparseArray, &'b [Entity], Self::Data, Self::Flags) {
        let (sparse, dense, data, flags) = self.set.split_raw();
        (sparse, dense, data.as_mut_ptr(), flags.as_mut_ptr())
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags {
        *flags.add(index)
    }

    unsafe fn get(data: Self::Data, flags: Self::Flags, index: usize) -> Option<Self::Output> {
        Some(ComponentRefMut::new(
            &mut *data.add(index),
            &mut *flags.add(index),
        ))
    }
}

pub(crate) struct SparseSetRefMut<'a, T>(AtomicRefMut<'a, SparseSet<T>>);

impl<'a, T> BorrowWorldUnsafe<'a> for SparseSetRefMut<'a, T>
where
    T: Component,
{
    unsafe fn borrow_world_unsafe(world: &'a World) -> Self {
        SparseSetRefMut(world.borrow_sparse_set_mut::<T>().unwrap())
    }
}

impl<T> Deref for SparseSetRefMut<'_, T> {
    type Target = SparseSet<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for SparseSetRefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
