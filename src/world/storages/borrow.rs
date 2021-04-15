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
	storage: ComponentStorageRef<'a, T>,
	group: Option<GroupInfo<'a>>,
}

impl<'a, T> Comp<'a, T>
where
	T: Component,
{
	pub(crate) unsafe fn new(
		storage: ComponentStorageRef<'a, T>,
		group: Option<GroupInfo<'a>>,
	) -> Self {
		Self { storage, group }
	}
}

pub struct CompMut<'a, T>
where
	T: Component,
{
	storage: ComponentStorageRefMut<'a, T>,
	group: Option<GroupInfo<'a>>,
}

impl<'a, T> CompMut<'a, T>
where
	T: Component,
{
	pub(crate) unsafe fn new(
		storage: ComponentStorageRefMut<'a, T>,
		group: Option<GroupInfo<'a>>,
	) -> Self {
		Self { storage, group }
	}
}
