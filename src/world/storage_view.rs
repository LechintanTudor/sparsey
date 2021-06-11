use crate::components::{Component, ComponentStorage, Ticks, TypedComponentStorage};
use crate::world::GroupInfo;
use atomic_refcell::{AtomicRef, AtomicRefMut};

pub(crate) type ComponentStorageRef<'a, T> =
	TypedComponentStorage<AtomicRef<'a, ComponentStorage>, T>;

pub(crate) type ComponentStorageRefMut<'a, T> =
	TypedComponentStorage<AtomicRefMut<'a, ComponentStorage>, T>;

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
}