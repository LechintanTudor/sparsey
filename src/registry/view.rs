use crate::data::{IterableView, ParentGroup, UnfilteredIterableView};
use crate::registry::{BorrowWorld, Component, World};
use crate::storage::{ComponentFlags, ComponentRefMut, Entity, SparseArray, SparseSet};
use atomic_refcell::{AtomicRef, AtomicRefMut};

pub struct Comp<'a, T>
where
    T: 'static,
{
    set: AtomicRef<'a, SparseSet<T>>,
    group: Option<ParentGroup>,
}

impl<'a, T> Comp<'a, T> {
    pub(crate) fn ungrouped(set: AtomicRef<'a, SparseSet<T>>) -> Self {
        Self { set, group: None }
    }

    pub(crate) unsafe fn grouped(
        set: AtomicRef<'a, SparseSet<T>>,
        group: Option<ParentGroup>,
    ) -> Self {
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

    fn parent_group(&self) -> Option<ParentGroup> {
        self.group
    }

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags) {
        let (sparse, dense, data, flags) = self.set.split();
        (sparse, dense, data.as_ptr(), flags.as_ptr())
    }

    unsafe fn matches_flags(_flags: Self::Flags, _index: usize) -> bool {
        true
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags {
        *flags.add(index)
    }

    unsafe fn get(data: Self::Data, _: Self::Flags, index: usize) -> Self::Output {
        &*data.add(index)
    }
}

unsafe impl<'a, T> UnfilteredIterableView<'a> for &'a Comp<'a, T> {}

pub struct CompMut<'a, T>
where
    T: 'static,
{
    set: AtomicRefMut<'a, SparseSet<T>>,
    group: Option<ParentGroup>,
}

impl<'a, T> CompMut<'a, T> {
    pub(crate) fn ungrouped(set: AtomicRefMut<'a, SparseSet<T>>) -> Self {
        Self { set, group: None }
    }

    pub unsafe fn grouped(set: AtomicRefMut<'a, SparseSet<T>>, group: Option<ParentGroup>) -> Self {
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

    fn parent_group(&self) -> Option<ParentGroup> {
        self.group
    }

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags) {
        let (sparse, dense, data, flags) = self.set.split();
        (sparse, dense, data.as_ptr(), flags.as_ptr())
    }

    unsafe fn matches_flags(_flags: Self::Flags, _index: usize) -> bool {
        true
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags {
        *flags.add(index)
    }

    unsafe fn get(data: Self::Data, _: Self::Flags, index: usize) -> Self::Output {
        &*data.add(index)
    }
}

unsafe impl<'a, T> UnfilteredIterableView<'a> for &'a CompMut<'a, T> {}

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

    unsafe fn matches_flags(_flags: Self::Flags, _index: usize) -> bool {
        true
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags {
        *flags.add(index)
    }

    unsafe fn get(data: Self::Data, flags: Self::Flags, index: usize) -> Self::Output {
        ComponentRefMut::new(&mut *data.add(index), &mut *flags.add(index))
    }
}

unsafe impl<'a: 'b, 'b, T> UnfilteredIterableView<'b> for &'b mut CompMut<'a, T> {}

pub(crate) struct BorrowSparseSetMut<'a, T>(pub(crate) AtomicRefMut<'a, SparseSet<T>>);

impl<'a, T> BorrowWorld<'a> for BorrowSparseSetMut<'a, T>
where
    T: Component,
{
    fn borrow_world(world: &'a World) -> Self {
        unsafe { Self(world.borrow_sparse_set_mut().unwrap()) }
    }
}
