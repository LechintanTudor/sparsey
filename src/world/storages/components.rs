use crate::components::{Component, ComponentStorage, Entity};
use crate::world::{
	Comp, CompMut, ComponentStorageRef, ComponentStorageRefMut, GroupedComponentStorages, Layout,
	UngroupedComponentStorages,
};
use atomic_refcell::{AtomicRef, AtomicRefMut};
use std::any::TypeId;

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

	// pub(crate) fn set_layout(&mut self, layout: &Layout, entities: &[Entity]) {
	// 	let mut sparse_sets = HashMap::<TypeId, ComponentStorage>::new();

	// 	for sparse_set in self.grouped.drain().chain(self.ungrouped.drain()) {
	// 		sparse_sets.insert(sparse_set.type_info().id(), sparse_set);
	// 	}

	// 	self.grouped = GroupedComponentStorages::with_layout(&layout, &mut sparse_sets);
	// 	self.ungrouped = UngroupedComponentStorages::from_sparse_sets(&mut sparse_sets);

	// 	for i in 0..self.grouped.group_set_count() {
	// 		for &entity in entities {
	// 			self.grouped.group_components(i, entity);
	// 		}
	// 	}
	// }

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

	pub(crate) fn iter_sparse_sets_mut(
		&mut self,
	) -> impl Iterator<Item = &mut ComponentStorage> + '_ {
		self.grouped
			.iter_sparse_sets_mut()
			.chain(self.ungrouped.iter_sparse_sets_mut())
	}
}
