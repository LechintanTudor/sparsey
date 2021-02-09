use crate::dispatcher::{CommandBuffers, Commands};
use crate::resources::{Res, ResMut, Resource, UnsafeResources};
use crate::world::{Comp, CompMut, Component, LayoutComponent, World};
use std::any::TypeId;
use std::marker::PhantomData;

pub enum RegistryAccess {
    Commands,
    Comp(LayoutComponent),
    CompMut(LayoutComponent),
    Res(TypeId),
    ResMut(TypeId),
}

impl RegistryAccess {
    pub fn conflicts(&self, other: &RegistryAccess) -> bool {
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

pub struct Registry<'a> {
    world: &'a World,
    resources: &'a UnsafeResources,
    command_buffers: &'a CommandBuffers,
}

impl<'a> Registry<'a> {
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

pub trait BorrowRegistry<'a> {
    type Item;

    fn registry_access() -> RegistryAccess;

    unsafe fn borrow_registry(registry: &'a Registry) -> Self::Item;
}

pub struct BorrowComp<T>(PhantomData<*const T>);

impl<'a, T> BorrowRegistry<'a> for BorrowComp<T>
where
    T: Component,
{
    type Item = Comp<'a, T>;

    fn registry_access() -> RegistryAccess {
        RegistryAccess::Comp(LayoutComponent::new::<T>())
    }

    unsafe fn borrow_registry(registry: &'a Registry) -> Self::Item {
        registry.world.borrow_comp::<T>().unwrap()
    }
}

pub struct BorrowCompMut<T>(PhantomData<*const T>);

impl<'a, T> BorrowRegistry<'a> for BorrowCompMut<T>
where
    T: Component,
{
    type Item = CompMut<'a, T>;

    fn registry_access() -> RegistryAccess {
        RegistryAccess::CompMut(LayoutComponent::new::<T>())
    }

    unsafe fn borrow_registry(registry: &'a Registry) -> Self::Item {
        registry.world.borrow_comp_mut::<T>().unwrap()
    }
}

pub struct BorrowRes<T>(PhantomData<*const T>);

impl<'a, T> BorrowRegistry<'a> for BorrowRes<T>
where
    T: Resource,
{
    type Item = Res<'a, T>;

    fn registry_access() -> RegistryAccess {
        RegistryAccess::Res(TypeId::of::<T>())
    }

    unsafe fn borrow_registry(registry: &'a Registry) -> Self::Item {
        registry.resources.borrow::<T>().unwrap()
    }
}

pub struct BorrowResMut<T>(PhantomData<*const T>);

impl<'a, T> BorrowRegistry<'a> for BorrowResMut<T>
where
    T: Resource,
{
    type Item = ResMut<'a, T>;

    fn registry_access() -> RegistryAccess {
        RegistryAccess::ResMut(TypeId::of::<T>())
    }

    unsafe fn borrow_registry(registry: &'a Registry) -> Self::Item {
        registry.resources.borrow_mut::<T>().unwrap()
    }
}

pub struct BorrowCommands(PhantomData<*const ()>);

impl<'a> BorrowRegistry<'a> for BorrowCommands {
    type Item = Commands<'a>;

    fn registry_access() -> RegistryAccess {
        RegistryAccess::Commands
    }

    unsafe fn borrow_registry(registry: &'a Registry) -> Self::Item {
        Commands::new(
            registry.command_buffers.next().unwrap(),
            registry.world.entities(),
        )
    }
}
