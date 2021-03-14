use crate::data::Component;
use crate::dispatcher::{
    BorrowCommands, BorrowComp, BorrowCompMut, BorrowEnvironment, BorrowRes, BorrowResMut, Commands,
};
use crate::query::{Comp, CompMut};
use crate::resources::{Res, ResMut, Resource};

/// Trait used for marking system parameters and borrowing data from the `Environment`.
pub trait LocalSystemParam {
    type Borrow: for<'a> BorrowEnvironment<'a>;
}

/// Marker trait for parameters of systems which are safe to run
/// from threads other than the one in which they were created.
pub unsafe trait SystemParam
where
    Self: LocalSystemParam,
{
}

impl<'a, T> LocalSystemParam for Comp<'a, T>
where
    T: Component,
{
    type Borrow = BorrowComp<T>;
}

unsafe impl<'a, T> SystemParam for Comp<'a, T> where T: Component {}

impl<'a, T> LocalSystemParam for CompMut<'a, T>
where
    T: Component,
{
    type Borrow = BorrowCompMut<T>;
}

unsafe impl<'a, T> SystemParam for CompMut<'a, T> where T: Component {}

impl<'a, T> LocalSystemParam for Res<'a, T>
where
    T: Resource,
{
    type Borrow = BorrowRes<T>;
}

unsafe impl<'a, T> SystemParam for Res<'a, T> where T: Resource + Sync {}

impl<'a, T> LocalSystemParam for ResMut<'a, T>
where
    T: Resource + Send,
{
    type Borrow = BorrowResMut<T>;
}

unsafe impl<'a, T> SystemParam for ResMut<'a, T> where T: Resource + Send {}

impl<'a> LocalSystemParam for Commands<'a> {
    type Borrow = BorrowCommands;
}

unsafe impl<'a> SystemParam for Commands<'a> {}
