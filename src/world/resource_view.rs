use crate::resources::ResourceCell;
use crate::utils::{ChangeTicks, Ticks};
use atomic_refcell::{AtomicRef, AtomicRefMut};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub trait ResourceView {
	fn ticks(&self) -> ChangeTicks;

	fn world_tick(&self) -> Ticks;

	fn change_tick(&self) -> Ticks;
}

pub fn res_added<R>(resource_view: &R) -> bool
where
	R: ResourceView,
{
	resource_view.ticks().tick_added() == resource_view.world_tick()
}

pub fn res_mutated<R>(resource_view: &R) -> bool
where
	R: ResourceView,
{
	resource_view.ticks().tick_mutated() > resource_view.change_tick()
}

pub fn res_changed<R>(resource_view: &R) -> bool
where
	R: ResourceView,
{
	res_added(resource_view) || res_mutated(resource_view)
}

pub struct Res<'a, T> {
	cell: AtomicRef<'a, ResourceCell>,
	world_tick: Ticks,
	change_tick: Ticks,
	phantom: PhantomData<&'a T>,
}

impl<'a, T> Res<'a, T> {
	pub(crate) unsafe fn new(
		cell: AtomicRef<'a, ResourceCell>,
		world_tick: Ticks,
		change_tick: Ticks,
	) -> Self {
		Self {
			cell,
			world_tick,
			change_tick,
			phantom: PhantomData,
		}
	}
}

impl<T> ResourceView for Res<'_, T> {
	fn ticks(&self) -> ChangeTicks {
		self.cell.ticks()
	}

	fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	fn change_tick(&self) -> Ticks {
		self.change_tick
	}
}

impl<T> Deref for Res<'_, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		unsafe { &*(&self.cell.value() as *const _ as *const T) }
	}
}

pub struct ResMut<'a, T> {
	cell: AtomicRefMut<'a, ResourceCell>,
	world_tick: Ticks,
	change_tick: Ticks,
	phantom: PhantomData<&'a T>,
}

impl<'a, T> ResMut<'a, T> {
	pub(crate) unsafe fn new(
		cell: AtomicRefMut<'a, ResourceCell>,
		world_tick: Ticks,
		change_tick: Ticks,
	) -> Self {
		Self {
			cell,
			world_tick,
			change_tick,
			phantom: PhantomData,
		}
	}
}

impl<T> ResourceView for ResMut<'_, T> {
	fn ticks(&self) -> ChangeTicks {
		self.cell.ticks()
	}

	fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	fn change_tick(&self) -> Ticks {
		self.change_tick
	}
}

impl<T> Deref for ResMut<'_, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		unsafe { &*(&self.cell.value() as *const _ as *const T) }
	}
}

impl<T> DerefMut for ResMut<'_, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.cell.ticks.set_tick_mutated(self.world_tick);

		unsafe { &mut *(&mut self.cell.value_mut() as *mut _ as *mut T) }
	}
}
