use crate::data::Component;
use crate::dispatcher::{CommandBuffers, Commands};
use crate::query::{Comp, CompMut};
use crate::resources::{Res, ResMut, Resource, UnsafeResources};
use crate::world::{LayoutComponent, World};
use std::any::TypeId;
use std::marker::PhantomData;

pub enum SystemAccess {
    Commands,
    Comp(LayoutComponent),
    CompMut(LayoutComponent),
    Res(TypeId),
    ResMut(TypeId),
}

impl SystemAccess {
    pub fn conflicts(&self, other: &SystemAccess) -> bool {
        match (self, other) {
            (Self::Comp(comp1), Self::CompMut(comp2)) => comp1 == comp2,
            (Self::CompMut(comp1), Self::Comp(comp2)) => comp1 == comp2,
            (Self::CompMut(comp1), Self::CompMut(comp2)) => comp1 == comp2,
            (Self::Res(res1), Self::ResMut(res2)) => res1 == res2,
            (Self::ResMut(res1), Self::Res(res2)) => res1 == res2,
            (Self::ResMut(res1), Self::ResMut(res2)) => res1 == res2,
            _ => false,
        }
    }
}

pub struct Environment<'a> {
    world: &'a World,
    resources: &'a UnsafeResources,
    command_buffers: &'a CommandBuffers,
}

impl<'a> Environment<'a> {
    pub fn new(
        world: &'a World,
        resources: &'a UnsafeResources,
        command_buffers: &'a CommandBuffers,
    ) -> Self {
        Self {
            world,
            resources,
            command_buffers,
        }
    }
}

pub unsafe trait BorrowEnvironment<'a> {
    type Item;

    fn access() -> SystemAccess;

    unsafe fn borrow(environment: &'a Environment) -> Self::Item;
}

pub struct BorrowComp<T>(PhantomData<*const T>);

unsafe impl<'a, T> BorrowEnvironment<'a> for BorrowComp<T>
where
    T: Component,
{
    type Item = Comp<'a, T>;

    fn access() -> SystemAccess {
        SystemAccess::Comp(LayoutComponent::new::<T>())
    }

    unsafe fn borrow(environment: &'a Environment) -> Self::Item {
        environment.world.borrow_comp::<T>().unwrap()
    }
}

pub struct BorrowCompMut<T>(PhantomData<*const T>);

unsafe impl<'a, T> BorrowEnvironment<'a> for BorrowCompMut<T>
where
    T: Component,
{
    type Item = CompMut<'a, T>;

    fn access() -> SystemAccess {
        SystemAccess::CompMut(LayoutComponent::new::<T>())
    }

    unsafe fn borrow(environment: &'a Environment) -> Self::Item {
        environment.world.borrow_comp_mut::<T>().unwrap()
    }
}

pub struct BorrowRes<T>(PhantomData<*const T>);

unsafe impl<'a, T> BorrowEnvironment<'a> for BorrowRes<T>
where
    T: Resource,
{
    type Item = Res<'a, T>;

    fn access() -> SystemAccess {
        SystemAccess::Res(TypeId::of::<T>())
    }

    unsafe fn borrow(environment: &'a Environment) -> Self::Item {
        environment.resources.borrow::<T>().unwrap()
    }
}

pub struct BorrowResMut<T>(PhantomData<*const T>);

unsafe impl<'a, T> BorrowEnvironment<'a> for BorrowResMut<T>
where
    T: Resource,
{
    type Item = ResMut<'a, T>;

    fn access() -> SystemAccess {
        SystemAccess::ResMut(TypeId::of::<T>())
    }

    unsafe fn borrow(environment: &'a Environment) -> Self::Item {
        environment.resources.borrow_mut::<T>().unwrap()
    }
}

pub struct BorrowCommands;

unsafe impl<'a> BorrowEnvironment<'a> for BorrowCommands {
    type Item = Commands<'a>;

    fn access() -> SystemAccess {
        SystemAccess::Commands
    }

    unsafe fn borrow(environment: &'a Environment) -> Self::Item {
        Commands::new(
            environment.command_buffers.next().unwrap(),
            environment.world.entities(),
        )
    }
}
