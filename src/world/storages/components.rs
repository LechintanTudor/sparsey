use crate::components::{Component, ComponentStorage, Entity};
use crate::world::{GroupInfo, GroupedComponentStorages, Layout, UngroupedComponentStorages};
use atomic_refcell::{AtomicRef, AtomicRefMut};
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

	pub(crate) unsafe fn register_storage(&mut self, component: TypeId, storage: ComponentStorage) {
		if !self.grouped.contains(&component) {
			self.ungrouped.register_storage(component, storage);
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

	pub fn borrow(&self, component: &TypeId) -> Option<AtomicRef<ComponentStorage>> {
		match self.grouped.borrow(component) {
			storage @ Some(_) => storage,
			None => self.ungrouped.borrow(component),
		}
	}

	pub fn borrow_mut(&self, component: &TypeId) -> Option<AtomicRefMut<ComponentStorage>> {
		match self.grouped.borrow_mut(component) {
			storage @ Some(_) => storage,
			None => self.ungrouped.borrow_mut(component),
		}
	}

	pub(crate) fn borrow_with_info(
		&self,
		component: &TypeId,
	) -> Option<(AtomicRef<ComponentStorage>, Option<GroupInfo>)> {
		match self.grouped.borrow_with_info(component) {
			Some((storage, info)) => Some((storage, Some(info))),
			None => self
				.ungrouped
				.borrow(component)
				.map(|storage| (storage, None)),
		}
	}

	pub(crate) fn borrow_with_info_mut(
		&self,
		component: &TypeId,
	) -> Option<(AtomicRefMut<ComponentStorage>, Option<GroupInfo>)> {
		match self.grouped.borrow_with_info_mut(component) {
			Some((storage, info)) => Some((storage, Some(info))),
			None => self
				.ungrouped
				.borrow_mut(component)
				.map(|storage| (storage, None)),
		}
	}

	pub(crate) fn iter_storages_mut(&mut self) -> impl Iterator<Item = &mut ComponentStorage> + '_ {
		self.grouped
			.iter_storages_mut()
			.chain(self.ungrouped.iter_storages_mut())
	}
}
