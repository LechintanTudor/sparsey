use crate::components::{Component, ComponentStorage, TypedComponentStorage};
use crate::world::GroupInfo;
use atomic_refcell::{AtomicRef, AtomicRefMut};

pub type ComponentStorageRef<'a, T> = TypedComponentStorage<AtomicRef<'a, ComponentStorage>, T>;
pub type ComponentStorageRefMut<'a, T> =
	TypedComponentStorage<AtomicRefMut<'a, ComponentStorage>, T>;

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
		storage: ComponentStorageRef<'a, T>,
		group_info: Option<GroupInfo<'a>>,
	) -> Self {
		Self {
			storage,
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
		storage: ComponentStorageRefMut<'a, T>,
		group_info: Option<GroupInfo<'a>>,
	) -> Self {
		Self {
			storage,
			group_info,
		}
	}
}
