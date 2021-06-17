use crate::components::{Component, ComponentStorage};
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use std::any::TypeId;
use std::collections::HashMap;

#[derive(Default)]
pub(crate) struct UngroupedComponentStorages {
	storages: HashMap<TypeId, AtomicRefCell<ComponentStorage>>,
}

unsafe impl Send for UngroupedComponentStorages {}
unsafe impl Sync for UngroupedComponentStorages {}

impl UngroupedComponentStorages {
	pub fn from_storages(storage_map: &mut HashMap<TypeId, ComponentStorage>) -> Self {
		let mut storages = HashMap::<TypeId, AtomicRefCell<ComponentStorage>>::new();

		for (type_id, storage) in storage_map.drain() {
			storages.insert(type_id, AtomicRefCell::new(storage));
		}

		Self { storages }
	}

	pub fn drain_into(&mut self, storages: &mut HashMap<TypeId, ComponentStorage>) {
		for (type_id, storage) in self
			.storages
			.drain()
			.map(|(type_id, storage)| (type_id, storage.into_inner()))
		{
			storages.insert(type_id, storage);
		}
	}

	pub fn register<T>(&mut self)
	where
		T: Component,
	{
		self.storages
			.entry(TypeId::of::<T>())
			.or_insert_with(|| AtomicRefCell::new(ComponentStorage::for_type::<T>()));
	}

	pub unsafe fn register_storage(&mut self, type_id: TypeId, storage: ComponentStorage) {
		self.storages
			.entry(type_id)
			.or_insert_with(|| AtomicRefCell::new(storage));
	}

	pub fn clear(&mut self) {
		for storage in self.storages.values_mut() {
			storage.get_mut().clear();
		}
	}

	pub fn borrow(&self, component: &TypeId) -> Option<AtomicRef<ComponentStorage>> {
		self.storages.get(component).map(|storage| storage.borrow())
	}

	pub fn borrow_mut(&self, component: &TypeId) -> Option<AtomicRefMut<ComponentStorage>> {
		self.storages
			.get(component)
			.map(|storage| storage.borrow_mut())
	}

	pub fn iter_storages_mut(&mut self) -> impl Iterator<Item = &mut ComponentStorage> {
		self.storages.values_mut().map(|storage| storage.get_mut())
	}
}
