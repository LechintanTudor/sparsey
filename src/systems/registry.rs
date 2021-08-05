use crate::components::Component;
use crate::layout::ComponentInfo;
use crate::resources::{Res, ResMut, Resource, UnsafeResources};
use crate::systems::{CommandBuffers, Commands};
use crate::utils::{panic_missing_comp, panic_missing_res, Ticks};
use crate::world::{Comp, CompMut, World};
use std::any::TypeId;
use std::marker::PhantomData;

/// Represents the type of data which can be accessed by a `System`.
/// Get a command buffer for queueing commands.
pub enum RegistryAccess {
	Commands,
	/// Get a shared view over a set of components from the `World`.
	Comp(ComponentInfo),
	/// Get an exclusive view over a set of components from the `World`.
	CompMut(ComponentInfo),
	/// Get a shared view over a resource from `Resources`.
	Res(TypeId),
	/// Get an exclusive view over a resource from `Resources`.
	ResMut(TypeId),
}

impl RegistryAccess {
	/// Check if two `RegistryAccesses` conflict, that is,
	/// preventing two systems from running in parallel.
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

/// Execution registry for `Systems`.
pub struct Registry<'a> {
	world: &'a World,
	resources: &'a UnsafeResources,
	command_buffers: &'a CommandBuffers,
	last_system_tick: Ticks,
}

impl<'a> Registry<'a> {
	pub(crate) fn new(
		world: &'a World,
		resources: &'a UnsafeResources,
		command_buffers: &'a CommandBuffers,
		last_system_tick: Ticks,
	) -> Self {
		Self {
			world,
			resources,
			command_buffers,
			last_system_tick,
		}
	}
}

/// Used by systems to borrow data from `Registrys`.
pub unsafe trait BorrowRegistry<'a> {
	/// The data resulting from the borrow.
	type Item;

	/// The type of data acessed.
	fn access() -> RegistryAccess;

	/// Borrow the data from the registry.
	/// Unsafe because it doesn't ensure !Sync or !Send
	/// resources are borrowed correctly.
	unsafe fn borrow(registry: &'a Registry) -> Self::Item;
}

/// Type used to get a command buffer for queueing commands.
pub struct BorrowCommands;

unsafe impl<'a> BorrowRegistry<'a> for BorrowCommands {
	type Item = Commands<'a>;

	fn access() -> RegistryAccess {
		RegistryAccess::Commands
	}

	unsafe fn borrow(registry: &'a Registry) -> Self::Item {
		Commands::new(
			registry.command_buffers.next().unwrap(),
			registry.world.entity_storage(),
		)
	}
}

/// Type used to get a shared view over a set of components from the `World`.
pub struct BorrowComp<T>(PhantomData<*const T>);

unsafe impl<'a, T> BorrowRegistry<'a> for BorrowComp<T>
where
	T: Component,
{
	type Item = Comp<'a, T>;

	fn access() -> RegistryAccess {
		RegistryAccess::Comp(ComponentInfo::new::<T>())
	}

	unsafe fn borrow(registry: &'a Registry) -> Self::Item {
		let (storage, info) = registry
			.world
			.component_storages()
			.borrow_with_info(&TypeId::of::<T>())
			.unwrap_or_else(|| panic_missing_comp::<T>());

		Comp::<T>::new(
			storage,
			info,
			registry.world.tick(),
			registry.last_system_tick,
		)
	}
}

/// Type used to get an exclusive view over a set of components from the
/// `World`.
pub struct BorrowCompMut<T>(PhantomData<*const T>);

unsafe impl<'a, T> BorrowRegistry<'a> for BorrowCompMut<T>
where
	T: Component,
{
	type Item = CompMut<'a, T>;

	fn access() -> RegistryAccess {
		RegistryAccess::CompMut(ComponentInfo::new::<T>())
	}

	unsafe fn borrow(registry: &'a Registry) -> Self::Item {
		let (storage, info) = registry
			.world
			.component_storages()
			.borrow_with_info_mut(&TypeId::of::<T>())
			.unwrap_or_else(|| panic_missing_comp::<T>());

		CompMut::<T>::new(
			storage,
			info,
			registry.world.tick(),
			registry.last_system_tick,
		)
	}
}

/// Type used to get a shared view over a resource from `Resources`.
pub struct BorrowRes<T>(PhantomData<*const T>);

unsafe impl<'a, T> BorrowRegistry<'a> for BorrowRes<T>
where
	T: Resource,
{
	type Item = Res<'a, T>;

	fn access() -> RegistryAccess {
		RegistryAccess::Res(TypeId::of::<T>())
	}

	unsafe fn borrow(registry: &'a Registry) -> Self::Item {
		registry
			.resources
			.borrow::<T>()
			.unwrap_or_else(|| panic_missing_res::<T>())
	}
}

/// Type used to get an exclusive view over a resource from `Resources`.
pub struct BorrowResMut<T>(PhantomData<*const T>);

unsafe impl<'a, T> BorrowRegistry<'a> for BorrowResMut<T>
where
	T: Resource,
{
	type Item = ResMut<'a, T>;

	fn access() -> RegistryAccess {
		RegistryAccess::ResMut(TypeId::of::<T>())
	}

	unsafe fn borrow(registry: &'a Registry) -> Self::Item {
		registry
			.resources
			.borrow_mut::<T>()
			.unwrap_or_else(|| panic_missing_res::<T>())
	}
}
