use crate::data::{IterableView, ParentGroup};
use crate::entity::Entity;
use crate::registry::{BorrowFromWorld, Component, World};
use crate::storage::{ComponentFlags, ComponentRefMut, SparseArray, SparseSet};
use atomic_refcell::{AtomicRef, AtomicRefMut};

pub struct Comp<'a, T>
where
    T: 'static,
{
    set: AtomicRef<'a, SparseSet<T>>,
    group: Option<ParentGroup>,
}

impl<'a, T> Comp<'a, T> {
    pub(crate) unsafe fn new(set: AtomicRef<'a, SparseSet<T>>, group: Option<ParentGroup>) -> Self {
        Self { set, group }
    }
}

impl<'a, T> BorrowFromWorld<'a> for Comp<'a, T>
where
    T: Component,
{
    fn borrow(world: &'a World) -> Self {
        world.borrow_comp().unwrap()
    }
}

impl<'a, T> IterableView<'a> for &'a Comp<'a, T> {
    type Data = *const T;
    type Flags = *const ComponentFlags;
    type Output = &'a T;

    fn parent_group(&self) -> Option<ParentGroup> {
        self.group
    }

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags) {
        let (sparse, dense, data, flags) = self.set.split();
        (sparse, dense, data.as_ptr(), flags.as_ptr())
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
    group: Option<ParentGroup>,
}

impl<'a, T> CompMut<'a, T> {
    pub unsafe fn new(set: AtomicRefMut<'a, SparseSet<T>>, group: Option<ParentGroup>) -> Self {
        Self { set, group }
    }
}

impl<'a, T> BorrowFromWorld<'a> for CompMut<'a, T>
where
    T: Component,
{
    fn borrow(world: &'a World) -> Self {
        world.borrow_comp_mut().unwrap()
    }
}

impl<'a, T> IterableView<'a> for &'a CompMut<'a, T> {
    type Data = *const T;
    type Flags = *const ComponentFlags;
    type Output = &'a T;

    fn parent_group(&self) -> Option<ParentGroup> {
        self.group
    }

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags) {
        let (sparse, dense, data, flags) = self.set.split();
        (sparse, dense, data.as_ptr(), flags.as_ptr())
    }

    unsafe fn get(data: Self::Data, _: Self::Flags, index: usize) -> Option<Self::Output> {
        Some(&*data.add(index))
    }
}

impl<'a: 'b, 'b, T> IterableView<'b> for &'b mut CompMut<'a, T> {
    type Data = *mut T;
    type Flags = *mut ComponentFlags;
    type Output = ComponentRefMut<'b, T>;

    fn parent_group(&self) -> Option<ParentGroup> {
        self.group
    }

    unsafe fn split(self) -> (&'b SparseArray, &'b [Entity], Self::Data, Self::Flags) {
        let (sparse, dense, data, flags) = self.set.split_raw();
        (sparse, dense, data.as_mut_ptr(), flags.as_mut_ptr())
    }

    unsafe fn get(data: Self::Data, flags: Self::Flags, index: usize) -> Option<Self::Output> {
        Some(ComponentRefMut::new(
            &mut *data.add(index),
            &mut *flags.add(index),
        ))
    }
}

pub struct SparseSetMut<'a, T>(pub(crate) AtomicRefMut<'a, SparseSet<T>>);

impl<'a, T> BorrowFromWorld<'a> for SparseSetMut<'a, T>
where
    T: Component,
{
    fn borrow(world: &'a World) -> Self {
        unsafe { Self(world.borrow_sparse_set_mut().unwrap()) }
    }
}
