use crate::components::{Component, ComponentStorage};
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use std::any::TypeId;
use std::collections::HashMap;

#[derive(Default)]
pub(crate) struct UngroupedComponentStorages {
	sparse_sets: HashMap<TypeId, AtomicRefCell<ComponentStorage>>,
}

impl UngroupedComponentStorages {
	pub fn from_sparse_sets(sparse_set_map: &mut HashMap<TypeId, ComponentStorage>) -> Self {
		let mut sparse_sets = HashMap::<TypeId, AtomicRefCell<ComponentStorage>>::new();

		for (type_id, sparse_set) in sparse_set_map.drain() {
			sparse_sets.insert(type_id, AtomicRefCell::new(sparse_set));
		}

		Self { sparse_sets }
	}

	pub fn register<T>(&mut self)
	where
		T: Component,
	{
		self.sparse_sets
			.entry(TypeId::of::<T>())
			.or_insert_with(|| AtomicRefCell::new(ComponentStorage::for_type::<T>()));
	}

	pub fn clear(&mut self) {
		for sparse_set in self.sparse_sets.values_mut() {
			sparse_set.get_mut().clear();
		}
	}

	pub fn drain(&mut self) -> impl Iterator<Item = ComponentStorage> + '_ {
		self.sparse_sets
			.drain()
			.map(|(_, sparse_set)| sparse_set.into_inner())
	}

	pub fn borrow(&self, component: &TypeId) -> Option<AtomicRef<ComponentStorage>> {
		self.sparse_sets
			.get(component)
			.map(|sparse_set| sparse_set.borrow())
	}

	pub fn borrow_mut(&self, component: &TypeId) -> Option<AtomicRefMut<ComponentStorage>> {
		self.sparse_sets
			.get(component)
			.map(|sparse_set| sparse_set.borrow_mut())
	}

	pub fn iter_sparse_sets_mut(&mut self) -> impl Iterator<Item = &mut ComponentStorage> {
		self.sparse_sets
			.values_mut()
			.map(|sparse_set| sparse_set.get_mut())
	}
}
