use crate::components::Component;
use crate::dispatcher::{CommandBuffers, Commands};
use crate::resources::{Res, ResMut, Resource, UnsafeResources};
use crate::utils::{panic_missing_comp, panic_missing_res};
use crate::world::{Comp, CompMut, LayoutComponent, World};
use std::any::TypeId;
use std::marker::PhantomData;

/// Represents the type of data which can be accessed by a `System`.
pub enum SystemAccess {
	/// Get a command buffer for queueing commands.
	Commands,
	/// Get a shared view over a set of components from the `World`.
	Comp(LayoutComponent),
	/// Get an exclusive view over a set of components from the `World`.
	CompMut(LayoutComponent),
	/// Get a shared view over a resource from `Resources`.
	Res(TypeId),
	/// Get an exclusive view over a resource from `Resources`.
	ResMut(TypeId),
}

impl SystemAccess {
	/// Check if two `SystemAccesses` conflict, that is,
	/// preventing two systems from running in parallel.
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

/// Execution environment for `Systems`.
pub struct Environment<'a> {
	world: &'a World,
	resources: &'a UnsafeResources,
	command_buffers: &'a CommandBuffers,
}

impl<'a> Environment<'a> {
	pub(crate) fn new(
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

/// Used by systems to borrow data from `Environments`.
pub unsafe trait BorrowEnvironment<'a> {
	/// The data resulting from the borrow.
	type Item;

	/// The type of data acessed.
	fn access() -> SystemAccess;

	/// Borrow the data from the environment.
	/// Unsafe because it doesn't ensure !Sync or !Send
	/// resources are borrowed correctly.
	unsafe fn borrow(environment: &'a Environment) -> Self::Item;
}

/// Type used to get a command buffer for queueing commands.
pub struct BorrowCommands;

unsafe impl<'a> BorrowEnvironment<'a> for BorrowCommands {
	type Item = Commands<'a>;

	fn access() -> SystemAccess {
		SystemAccess::Commands
	}

	unsafe fn borrow(environment: &'a Environment) -> Self::Item {
		Commands::new(
			environment.command_buffers.next().unwrap(),
			environment.world.entity_storage(),
		)
	}
}

/// Type used to get a shared view over a set of components from the `World`.
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
		environment
			.world
			.borrow_comp::<T>()
			.unwrap_or_else(|| panic_missing_comp::<T>())
	}
}

/// Type used to get an exclusive view over a set of components from the `World`.
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
		environment
			.world
			.borrow_comp_mut::<T>()
			.unwrap_or_else(|| panic_missing_comp::<T>())
	}
}

/// Type used to get a shared view over a resource from `Resources`.
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
		environment
			.resources
			.borrow::<T>()
			.unwrap_or_else(|| panic_missing_res::<T>())
	}
}

/// Type used to get an exclusive view over a resource from `Resources`.
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
		environment
			.resources
			.borrow_mut::<T>()
			.unwrap_or_else(|| panic_missing_res::<T>())
	}
}
