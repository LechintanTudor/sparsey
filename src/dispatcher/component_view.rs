use crate::components::{Component, ComponentStorage, Ticks, TypedComponentStorage};
use crate::world::GroupInfo;
use atomic_refcell::{AtomicRef, AtomicRefMut};

pub type ComponentStorageRef<'a, T> = TypedComponentStorage<AtomicRef<'a, ComponentStorage>, T>;
pub type ComponentStorageRefMut<'a, T> =
	TypedComponentStorage<AtomicRefMut<'a, ComponentStorage>, T>;

// TODO: Store system ticks in Comp/CompMut

pub struct Comp<'a, T>
where
	T: Component,
{
	pub(crate) storage: ComponentStorageRef<'a, T>,
	group_info: Option<GroupInfo<'a>>,
	world_tick: Ticks,
	last_system_tick: Ticks,
}

impl<'a, T> Comp<'a, T>
where
	T: Component,
{
	pub(crate) unsafe fn new(
		storage: AtomicRef<'a, ComponentStorage>,
		group_info: Option<GroupInfo<'a>>,
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

	pub(crate) fn group_info(&self) -> Option<&GroupInfo> {
		self.group_info.as_ref()
	}

	pub(crate) fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	pub(crate) fn last_system_tick(&self) -> Ticks {
		self.last_system_tick
	}
}

pub struct CompMut<'a, T>
where
	T: Component,
{
	pub(crate) storage: ComponentStorageRefMut<'a, T>,
	group_info: Option<GroupInfo<'a>>,
	world_tick: Ticks,
	last_system_tick: Ticks,
}

impl<'a, T> CompMut<'a, T>
where
	T: Component,
{
	pub(crate) unsafe fn new(
		storage: AtomicRefMut<'a, ComponentStorage>,
		group_info: Option<GroupInfo<'a>>,
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

	pub(crate) fn group_info(&self) -> Option<&GroupInfo> {
		self.group_info.as_ref()
	}

	pub(crate) fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	pub(crate) fn last_system_tick(&self) -> Ticks {
		self.last_system_tick
	}
}
