use crate::resources::ResourceCell;
use crate::utils::Ticks;
use atomic_refcell::{AtomicRef, AtomicRefMut};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

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
