use crate::components::{
	Component, ComponentStorage, ComponentTicks, Entity, Ticks, TypedComponentStorage,
};
use crate::world::GroupInfo;
use atomic_refcell::{AtomicRef, AtomicRefMut};
use std::ops::Deref;

type ComponentStorageRef<'a, T> = TypedComponentStorage<AtomicRef<'a, ComponentStorage>, T>;
type ComponentStorageRefMut<'a, T> = TypedComponentStorage<AtomicRefMut<'a, ComponentStorage>, T>;

pub struct Comp<'a, T>
where
	T: Component,
{
	pub(crate) storage: ComponentStorageRef<'a, T>,
	pub(crate) group_info: GroupInfo<'a>,
	pub(crate) world_tick: Ticks,
	pub(crate) last_system_tick: Ticks,
}

impl<'a, T> Comp<'a, T>
where
	T: Component,
{
	pub(crate) unsafe fn new(
		storage: AtomicRef<'a, ComponentStorage>,
		group_info: GroupInfo<'a>,
		world_tick: Ticks,
		last_system_tick: Ticks,
	) -> Self {
		Self {
			storage: ComponentStorageRef::new(storage),
			group_info,
			world_tick,
			last_system_tick,
		}
	}

	pub fn entities(&self) -> &[Entity] {
		self.storage.entities()
	}

	pub fn components(&self) -> &[T] {
		self.storage.components()
	}

	pub fn ticks(&self) -> &[ComponentTicks] {
		self.storage.ticks()
	}
}

impl<T> Deref for Comp<'_, T>
where
	T: Component,
{
	type Target = [T];

	fn deref(&self) -> &Self::Target {
		self.storage.components()
	}
}

pub struct CompMut<'a, T>
where
	T: Component,
{
	pub(crate) storage: ComponentStorageRefMut<'a, T>,
	pub(crate) group_info: GroupInfo<'a>,
	pub(crate) world_tick: Ticks,
	pub(crate) last_system_tick: Ticks,
}

impl<'a, T> CompMut<'a, T>
where
	T: Component,
{
	pub(crate) unsafe fn new(
		storage: AtomicRefMut<'a, ComponentStorage>,
		group_info: GroupInfo<'a>,
		world_tick: Ticks,
		last_system_tick: Ticks,
	) -> Self {
		Self {
			storage: ComponentStorageRefMut::new(storage),
			group_info,
			world_tick,
			last_system_tick,
		}
	}

	pub fn entities(&self) -> &[Entity] {
		self.storage.entities()
	}

	pub fn components(&self) -> &[T] {
		self.storage.components()
	}

	pub fn ticks(&self) -> &[ComponentTicks] {
		self.storage.ticks()
	}
}

impl<T> Deref for CompMut<'_, T>
where
	T: Component,
{
	type Target = [T];

	fn deref(&self) -> &Self::Target {
		self.storage.components()
	}
}
