use crate::dispatcher::{
    BorrowCommands, BorrowComp, BorrowCompMut, BorrowRegistry, BorrowRes, BorrowResMut, Commands,
};
use crate::resources::{Res, ResMut, Resource};
use crate::world::{Comp, CompMut, Component};

pub trait ThreadLocalSystemParameter {
    type Borrow: for<'a> BorrowRegistry<'a>;
}

pub unsafe trait SystemParameter
where
    Self: ThreadLocalSystemParameter,
{
}

impl<'a, T> ThreadLocalSystemParameter for Comp<'a, T>
where
    T: Component,
{
    type Borrow = BorrowComp<T>;
}

unsafe impl<'a, T> SystemParameter for Comp<'a, T> where T: Component {}

impl<'a, T> ThreadLocalSystemParameter for CompMut<'a, T>
where
    T: Component,
{
    type Borrow = BorrowCompMut<T>;
}

unsafe impl<'a, T> SystemParameter for CompMut<'a, T> where T: Component {}

impl<'a, T> ThreadLocalSystemParameter for Res<'a, T>
where
    T: Resource,
{
    type Borrow = BorrowRes<T>;
}

unsafe impl<'a, T> SystemParameter for Res<'a, T> where T: Resource + Sync {}

impl<'a, T> ThreadLocalSystemParameter for ResMut<'a, T>
where
    T: Resource + Send,
{
    type Borrow = BorrowResMut<T>;
}

unsafe impl<'a, T> SystemParameter for ResMut<'a, T> where T: Resource + Send {}

impl<'a> ThreadLocalSystemParameter for Commands<'a> {
    type Borrow = BorrowCommands;
}

unsafe impl<'a> SystemParameter for Commands<'a> {}
