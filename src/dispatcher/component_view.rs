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
	pub(crate) group_info: Option<GroupInfo<'a>>,
}

impl<'a, T> Comp<'a, T>
where
	T: Component,
{
	pub(crate) unsafe fn new(
		storage: AtomicRef<'a, ComponentStorage>,
		group_info: Option<GroupInfo<'a>>,
	) -> Self {
		Self {
			storage: ComponentStorageRef::new(storage),
			group_info,
		}
	}
}

pub struct CompMut<'a, T>
where
	T: Component,
{
	pub(crate) storage: ComponentStorageRefMut<'a, T>,
	pub(crate) group_info: Option<GroupInfo<'a>>,
}

impl<'a, T> CompMut<'a, T>
where
	T: Component,
{
	pub(crate) unsafe fn new(
		storage: AtomicRefMut<'a, ComponentStorage>,
		group_info: Option<GroupInfo<'a>>,
	) -> Self {
		Self {
			storage: ComponentStorageRefMut::new(storage),
			group_info,
		}
	}
}
