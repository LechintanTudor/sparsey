use crate::data::{GroupInfo, IterableView};
use crate::storage::Entity;
use crate::storage::{ComponentFlags, ComponentRefMut, SparseArray, SparseSet};
use crate::world::World;
use atomic_refcell::{AtomicRef, AtomicRefMut};
use std::any::TypeId;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub trait Component
where
    Self: Send + Sync + 'static,
{
}

impl<T> Component for T where T: Send + Sync + 'static {}

pub struct Comp<'a, T>
where
    T: 'static,
{
    set: AtomicRef<'a, SparseSet<T>>,
    group: Option<GroupInfo>,
}

impl<'a, T> Comp<'a, T> {
    pub(crate) unsafe fn new(set: AtomicRef<'a, SparseSet<T>>, group: Option<GroupInfo>) -> Self {
        Self { set, group }
    }
}

impl<'a, T> IterableView<'a> for &'a Comp<'a, T>
where
    T: Send + Sync + 'static,
{
    type Data = *const T;
    type Flags = *const ComponentFlags;
    type Output = &'a T;

    unsafe fn group(&self) -> Option<GroupInfo> {
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
    group: Option<GroupInfo>,
}

impl<'a, T> CompMut<'a, T> {
    pub(crate) unsafe fn new(
        set: AtomicRefMut<'a, SparseSet<T>>,
        group: Option<GroupInfo>,
    ) -> Self {
        Self { set, group }
    }
}

impl<'a, T> IterableView<'a> for &'a CompMut<'a, T>
where
    T: Send + Sync + 'static,
{
    type Data = *const T;
    type Flags = *const ComponentFlags;
    type Output = &'a T;

    unsafe fn group(&self) -> Option<GroupInfo> {
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

impl<'a: 'b, 'b, T> IterableView<'b> for &'b mut CompMut<'a, T>
where
    T: Send + Sync + 'static,
{
    type Data = *mut T;
    type Flags = *mut ComponentFlags;
    type Output = ComponentRefMut<'b, T>;

    unsafe fn group(&self) -> Option<GroupInfo> {
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

pub struct SparseSetRefMut<'a, T>(AtomicRefMut<'a, SparseSet<T>>);

impl<'a, T> SparseSetRefMut<'a, T> {
    pub(crate) fn new(sparse_set: AtomicRefMut<'a, SparseSet<T>>) -> Self {
        Self(sparse_set)
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

pub trait ComponentSet
where
    Self: Sized,
{
    type Components: AsRef<[TypeId]>;
    type Borrow: for<'a> BorrowSparseSetSet<'a>;

    unsafe fn components() -> Self::Components;

    unsafe fn append(
        sparse_sets: &mut <Self::Borrow as BorrowSparseSetSet>::SparseSetSet,
        entity: Entity,
        components: Self,
    );

    unsafe fn remove(
        sparse_sets: &mut <Self::Borrow as BorrowSparseSetSet>::SparseSetSet,
        entity: Entity,
    ) -> Option<Self>;

    unsafe fn delete(
        sparse_sets: &mut <Self::Borrow as BorrowSparseSetSet>::SparseSetSet,
        entity: Entity,
    );
}

pub trait BorrowSparseSetSet<'a> {
    type SparseSetSet: 'a;

    unsafe fn borrow(world: &'a World) -> Self::SparseSetSet;
}

pub struct SparseSetSetBorrower<S> {
    _phantom: PhantomData<S>,
}

macro_rules! impl_component_set {
    ($len:tt, $(($comp:ident, $idx:tt)),+) => {
        impl<$($comp),+> ComponentSet for ($($comp,)+)
        where
            $($comp: Component,)+
        {
            type Components = [TypeId; $len];
            type Borrow = SparseSetSetBorrower<Self>;

            unsafe fn components() -> Self::Components {
                [
                    $(TypeId::of::<$comp>()),+
                ]
            }

            unsafe fn append(
                sparse_sets: &mut <Self::Borrow as BorrowSparseSetSet>::SparseSetSet,
                entity: Entity,
                components: Self,
            ) {
                $(
                    sparse_sets.$idx.insert(entity, components.$idx);
                )+
            }

            unsafe fn remove(
                sparse_sets: &mut <Self::Borrow as BorrowSparseSetSet>::SparseSetSet,
                entity: Entity,
            ) -> Option<Self> {
                let removed_components = (
                    $(sparse_sets.$idx.remove(entity),)+
                );

                Some((
                    $(removed_components.$idx?,)+
                ))
            }

            unsafe fn delete(
                sparse_sets: &mut <Self::Borrow as BorrowSparseSetSet>::SparseSetSet,
                entity: Entity,
            ) {
                $(sparse_sets.$idx.remove(entity);)+
            }
        }

        impl<'a, $($comp),+> BorrowSparseSetSet<'a> for SparseSetSetBorrower<($($comp,)+)>
        where
            $($comp: Component,)+
        {
            type SparseSetSet = ($(SparseSetRefMut<'a, $comp>,)+);

            unsafe fn borrow(world: &'a World) -> Self::SparseSetSet {
                ($(world.borrow_sparse_set_mut::<$comp>().unwrap(),)+)
            }
        }
    };
}

impl_component_set!(1, (A, 0));
impl_component_set!(2, (A, 0), (B, 1));
impl_component_set!(3, (A, 0), (B, 1), (C, 2));
impl_component_set!(4, (A, 0), (B, 1), (C, 2), (D, 3));
