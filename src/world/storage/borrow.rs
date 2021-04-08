use crate::data::{Component, MappedAtomicRef, MappedAtomicRefMut, SparseSetRef, SparseSetRefMut};
use crate::world::GroupInfo;
use std::ops::{Deref, DerefMut};

pub struct Comp<'a, T>
where
	T: Component,
{
	pub(crate) storage: MappedAtomicRef<'a, SparseSetRef<'a, T>>,
	pub(crate) group_info: Option<GroupInfo<'a>>,
}

impl<'a, T> Comp<'a, T>
where
	T: Component,
{
	pub(crate) unsafe fn new(
		storage: MappedAtomicRef<'a, SparseSetRef<'a, T>>,
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
	pub(crate) storage: MappedAtomicRefMut<'a, SparseSetRefMut<'a, T>>,
	pub(crate) group_info: Option<GroupInfo<'a>>,
}

impl<'a, T> CompMut<'a, T>
where
	T: Component,
{
	pub(crate) unsafe fn new(
		storage: MappedAtomicRefMut<'a, SparseSetRefMut<'a, T>>,
		group_info: Option<GroupInfo<'a>>,
	) -> Self {
		Self {
			storage,
			group_info,
		}
	}
}

pub struct ComponentStorageRefMut<'a, T>
where
	T: Component,
{
	storage: MappedAtomicRefMut<'a, SparseSetRefMut<'a, T>>,
}

impl<'a, T> ComponentStorageRefMut<'a, T>
where
	T: Component,
{
	pub(crate) fn new(storage: MappedAtomicRefMut<'a, SparseSetRefMut<'a, T>>) -> Self {
		Self { storage }
	}
}

impl<'a, T> Deref for ComponentStorageRefMut<'a, T>
where
	T: Component,
{
	type Target = SparseSetRefMut<'a, T>;

	fn deref(&self) -> &Self::Target {
		self.storage.deref()
	}
}

impl<'a, T> DerefMut for ComponentStorageRefMut<'a, T>
where
	T: Component,
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.storage.deref_mut()
	}
}
