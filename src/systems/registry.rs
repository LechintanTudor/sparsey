use crate::components::Component;
use crate::layout::ComponentInfo;
use crate::resources::Resource;
use crate::systems::{CommandBuffers, Commands};
use crate::utils::Ticks;
use crate::world::{BorrowWorld, Comp, CompMut, Res, ResMut, World};
use std::any::TypeId;

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
	command_buffers: &'a CommandBuffers,
	change_tick: Ticks,
}

unsafe impl Send for Registry<'_> {}
unsafe impl Sync for Registry<'_> {}

impl<'a> Registry<'a> {
	pub(crate) unsafe fn new(
		world: &'a World,
		command_buffers: &'a CommandBuffers,
		change_tick: Ticks,
	) -> Self {
		Self {
			world,
			command_buffers,
			change_tick,
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

unsafe impl<'a, 'b> BorrowRegistry<'a> for Commands<'b> {
	type Item = Commands<'a>;

	fn access() -> RegistryAccess {
		RegistryAccess::Commands
	}

	unsafe fn borrow(registry: &'a Registry) -> Self::Item {
		Commands::new(
			registry.command_buffers.next().unwrap(),
			&registry.world.entities,
		)
	}
}

unsafe impl<'a, 'b, T> BorrowRegistry<'a> for Comp<'b, T>
where
	T: Component,
{
	type Item = Comp<'a, T>;

	fn access() -> RegistryAccess {
		RegistryAccess::Comp(ComponentInfo::new::<T>())
	}

	unsafe fn borrow(registry: &'a Registry) -> Self::Item {
		<Self as BorrowWorld>::borrow(registry.world, registry.change_tick)
	}
}

unsafe impl<'a, 'b, T> BorrowRegistry<'a> for CompMut<'b, T>
where
	T: Component,
{
	type Item = CompMut<'a, T>;

	fn access() -> RegistryAccess {
		RegistryAccess::CompMut(ComponentInfo::new::<T>())
	}

	unsafe fn borrow(registry: &'a Registry) -> Self::Item {
		<Self as BorrowWorld>::borrow(registry.world, registry.change_tick)
	}
}

unsafe impl<'a, 'b, T> BorrowRegistry<'a> for Res<'b, T>
where
	T: Resource,
{
	type Item = Res<'a, T>;

	fn access() -> RegistryAccess {
		RegistryAccess::Res(TypeId::of::<T>())
	}

	unsafe fn borrow(registry: &'a Registry) -> Self::Item {
		<Self as BorrowWorld>::borrow(registry.world, registry.change_tick)
	}
}

unsafe impl<'a, 'b, T> BorrowRegistry<'a> for ResMut<'b, T>
where
	T: Resource,
{
	type Item = ResMut<'a, T>;

	fn access() -> RegistryAccess {
		RegistryAccess::ResMut(TypeId::of::<T>())
	}

	unsafe fn borrow(registry: &'a Registry) -> Self::Item {
		<Self as BorrowWorld>::borrow(registry.world, registry.change_tick)
	}
}
