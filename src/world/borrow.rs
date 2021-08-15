use crate::components::Component;
use crate::layout::ComponentInfo;
use crate::resources::Resource;
use crate::systems::RegistryAccess;
use crate::utils::{panic_missing_comp, panic_missing_res, Ticks};
use crate::world::{Comp, CompMut, Res, ResMut, World};
use std::any::TypeId;

pub unsafe trait BorrowWorld<'a> {
	type Item;

	fn access() -> RegistryAccess;

	fn borrow(world: &'a World, change_tick: Ticks) -> Self::Item;
}

unsafe impl<'a, 'b, T> BorrowWorld<'a> for Comp<'b, T>
where
	T: Component,
{
	type Item = Comp<'a, T>;

	fn access() -> RegistryAccess {
		RegistryAccess::Comp(ComponentInfo::new::<T>())
	}

	fn borrow(world: &'a World, change_tick: Ticks) -> Self::Item {
		let (storage, info) = world
			.components
			.borrow_with_info(&TypeId::of::<T>())
			.unwrap_or_else(|| panic_missing_comp::<T>());

		unsafe { Comp::new(storage, info, world.tick.get(), change_tick) }
	}
}

unsafe impl<'a, 'b, T> BorrowWorld<'a> for CompMut<'b, T>
where
	T: Component,
{
	type Item = CompMut<'a, T>;

	fn access() -> RegistryAccess {
		RegistryAccess::CompMut(ComponentInfo::new::<T>())
	}

	fn borrow(world: &'a World, change_tick: Ticks) -> Self::Item {
		let (storage, info) = world
			.components
			.borrow_with_info_mut(&TypeId::of::<T>())
			.unwrap_or_else(|| panic_missing_comp::<T>());

		unsafe { CompMut::new(storage, info, world.tick.get(), change_tick) }
	}
}

unsafe impl<'a, 'b, T> BorrowWorld<'a> for Res<'b, T>
where
	T: Resource,
{
	type Item = Res<'a, T>;

	fn access() -> RegistryAccess {
		RegistryAccess::Res(TypeId::of::<T>())
	}

	fn borrow(world: &'a World, change_tick: Ticks) -> Self::Item {
		let cell = world
			.resources
			.borrow::<T>()
			.unwrap_or_else(|| panic_missing_res::<T>());

		unsafe { Res::new(cell, world.tick.get(), change_tick) }
	}
}

unsafe impl<'a, 'b, T> BorrowWorld<'a> for ResMut<'b, T>
where
	T: Resource,
{
	type Item = ResMut<'a, T>;

	fn access() -> RegistryAccess {
		RegistryAccess::ResMut(TypeId::of::<T>())
	}

	fn borrow(world: &'a World, change_tick: Ticks) -> Self::Item {
		let cell = world
			.resources
			.borrow_mut::<T>()
			.unwrap_or_else(|| panic_missing_res::<T>());

		unsafe { ResMut::new(cell, world.tick.get(), change_tick) }
	}
}
