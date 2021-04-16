use crate::components::{Component, ComponentStorage, Entity};
use crate::world::{
	Comp, CompMut, ComponentStorageRef, ComponentStorageRefMut, GroupedComponentStorages, Layout,
	UngroupedComponentStorages,
};
use std::any::TypeId;
use std::collections::HashMap;

/// Container for grouped and ungrouped component storages.
#[derive(Default)]
pub struct ComponentStorages {
	pub(crate) grouped: GroupedComponentStorages,
	pub(crate) ungrouped: UngroupedComponentStorages,
}

impl ComponentStorages {
	pub(crate) fn clear(&mut self) {
		self.grouped.clear();
		self.ungrouped.clear();
	}

	pub(crate) fn register<T>(&mut self)
	where
		T: Component,
	{
		if !self.grouped.contains(&TypeId::of::<T>()) {
			self.ungrouped.register::<T>();
		}
	}

	pub(crate) fn register_storage(&mut self, type_id: TypeId, storage: ComponentStorage) {
		if !self.grouped.contains(&type_id) {
			self.ungrouped.register_storage(type_id, storage);
		}
	}

	pub(crate) fn set_layout(&mut self, layout: &Layout, entities: &[Entity]) {
		let mut storages = HashMap::<TypeId, ComponentStorage>::new();
		self.grouped.drain_into(&mut storages);
		self.ungrouped.drain_into(&mut storages);

		self.grouped = GroupedComponentStorages::with_layout(&layout, &mut storages);
		self.ungrouped = UngroupedComponentStorages::from_storages(&mut storages);

		for i in 0..self.grouped.group_set_count() {
			for &entity in entities {
				self.grouped.group_components(i, entity);
			}
		}
	}

	pub(crate) fn borrow_comp<T>(&self) -> Option<Comp<T>>
	where
		T: Component,
	{
		match self.grouped.borrow(&TypeId::of::<T>()) {
			Some(storage) => unsafe {
				Some(Comp::new(
					ComponentStorageRef::new(storage),
					self.grouped.get_group_info(&TypeId::of::<T>()),
				))
			},
			None => match self.ungrouped.borrow(&TypeId::of::<T>()) {
				Some(storage) => unsafe {
					Some(Comp::new(ComponentStorageRef::new(storage), None))
				},
				None => None,
			},
		}
	}

	pub(crate) fn borrow_comp_mut<T>(&self) -> Option<CompMut<T>>
	where
		T: Component,
	{
		match self.grouped.borrow_mut(&TypeId::of::<T>()) {
			Some(storage) => unsafe {
				Some(CompMut::new(
					ComponentStorageRefMut::new(storage),
					self.grouped.get_group_info(&TypeId::of::<T>()),
				))
			},
			None => match self.ungrouped.borrow_mut(&TypeId::of::<T>()) {
				Some(storage) => unsafe {
					Some(CompMut::new(ComponentStorageRefMut::new(storage), None))
				},
				None => None,
			},
		}
	}

	pub(crate) fn borrow_storage_mut<T>(&self) -> Option<ComponentStorageRefMut<T>>
	where
		T: Component,
	{
		match self.grouped.borrow_mut(&TypeId::of::<T>()) {
			Some(storage) => unsafe { Some(ComponentStorageRefMut::new(storage)) },
			None => match self.ungrouped.borrow_mut(&TypeId::of::<T>()) {
				Some(storage) => unsafe { Some(ComponentStorageRefMut::new(storage)) },
				None => None,
			},
		}
	}

	pub(crate) fn iter_storages_mut(&mut self) -> impl Iterator<Item = &mut ComponentStorage> + '_ {
		self.grouped
			.iter_storages_mut()
			.chain(self.ungrouped.iter_storages_mut())
	}
}
