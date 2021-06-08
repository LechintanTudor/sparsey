use crate::components::{
	BlobVec, ComponentTicks, Entity, IndexEntity, SparseArray, SparseArrayView, Ticks,
};
use std::alloc::Layout;
use std::{ptr, u32};

pub struct ComponentStorage {
	sparse: SparseArray,
	entities: Vec<Entity>,
	ticks: Vec<ComponentTicks>,
	data: BlobVec,
}

impl ComponentStorage {
	pub fn for_type<T>() -> Self
	where
		T: 'static,
	{
		unsafe { Self::new(Layout::new::<T>(), |ptr| ptr::drop_in_place::<T>(ptr as _)) }
	}

	pub unsafe fn new(item_layout: Layout, drop_item: unsafe fn(*mut u8)) -> Self {
		Self {
			sparse: SparseArray::default(),
			entities: Vec::new(),
			ticks: Vec::new(),
			data: BlobVec::new(item_layout, drop_item),
		}
	}

	pub unsafe fn insert_and_forget_prev(
		&mut self,
		entity: Entity,
		value: *const u8,
		tick: Ticks,
	) -> *mut u8 {
		let index_entity = self.sparse.get_mut_or_allocate_at(entity.index());

		match index_entity {
			Some(index_entity) => {
				let index = index_entity.index();
				*self.entities.get_unchecked_mut(index) = entity;
				self.ticks.get_unchecked_mut(index).tick_mutated = tick;
				self.data.set_and_forget_prev_unchecked(index, value)
			}
			None => {
				*index_entity = Some(IndexEntity::new(
					self.entities.len() as u32,
					entity.version(),
				));
				self.entities.push(entity);
				self.ticks.push(ComponentTicks::new(tick));
				self.data.push(value);
				ptr::null_mut()
			}
		}
	}

	pub unsafe fn insert_and_drop_prev(&mut self, entity: Entity, value: *const u8, tick: Ticks) {
		let index_entity = self.sparse.get_mut_or_allocate_at(entity.index());

		match index_entity {
			Some(index_entity) => {
				let index = index_entity.index();
				*self.entities.get_unchecked_mut(index) = entity;
				self.ticks.get_unchecked_mut(index).tick_mutated = tick;
				self.data.set_and_drop_prev_unchecked(index, value);
			}
			None => {
				*index_entity = Some(IndexEntity::new(
					self.entities.len() as u32,
					entity.version(),
				));
				self.entities.push(entity);
				self.ticks.push(ComponentTicks::new(tick));
				self.data.push(value);
			}
		}
	}

	pub fn remove_and_forget(&mut self, entity: Entity) -> *mut u8 {
		let index_entity = match self.sparse.remove(entity) {
			Some(index_entity) => index_entity,
			None => return ptr::null_mut(),
		};

		let dense_index = index_entity.index();
		self.entities.swap_remove(dense_index);
		self.ticks.swap_remove(dense_index);

		if let Some(entity) = self.entities.last() {
			let new_index = (self.entities.len() - 1) as u32;
			let new_index_entity = IndexEntity::new(new_index, entity.version());

			unsafe {
				*self.sparse.get_unchecked_mut(entity.index()) = Some(new_index_entity);
			}
		}

		unsafe { self.data.swap_remove_and_forget_unchecked(dense_index) }
	}

	pub fn remove_and_drop(&mut self, entity: Entity) -> bool {
		let index_entity = match self.sparse.remove(entity) {
			Some(index_entity) => index_entity,
			None => return false,
		};

		let dense_index = index_entity.index();
		self.entities.swap_remove(dense_index);
		self.ticks.swap_remove(dense_index);

		if let Some(entity) = self.entities.last() {
			let new_index = (self.entities.len() - 1) as u32;
			let new_index_entity = IndexEntity::new(new_index, entity.version());

			unsafe {
				*self.sparse.get_unchecked_mut(entity.index()) = Some(new_index_entity);
			}
		}

		unsafe {
			self.data.swap_remove_and_drop_unchecked(dense_index);
		}

		true
	}

	pub fn entities(&self) -> &[Entity] {
		&self.entities
	}

	pub fn data(&self) -> *const u8 {
		self.data.as_ptr()
	}

	pub fn clear(&mut self) {
		self.sparse.clear();
		self.entities.clear();
		self.data.clear();
	}

	pub fn swap(&mut self, a: usize, b: usize) {
		let sparse_a = self.entities[a].index();
		let sparse_b = self.entities[b].index();

		unsafe {
			self.sparse.swap_unchecked(sparse_a, sparse_b);
			self.data.swap_unchecked(a, b);
		}

		self.entities.swap(a, b);
		self.ticks.swap(a, b);
	}

	pub fn get(&self, entity: Entity) -> *const u8 {
		match self.sparse.get_index(entity) {
			Some(index) => unsafe { self.data.get_unchecked(index as usize) },
			None => ptr::null(),
		}
	}

	pub fn get_mut(&mut self, entity: Entity) -> *mut u8 {
		match self.sparse.get_index(entity) {
			Some(index) => unsafe { self.data.get_unchecked_mut(index as usize) },
			None => ptr::null_mut(),
		}
	}

	pub fn get_ticks(&self, entity: Entity) -> Option<&ComponentTicks> {
		let index = self.sparse.get_index(entity)? as usize;
		unsafe { Some(self.ticks.get_unchecked(index)) }
	}

	pub fn get_with_ticks(&self, entity: Entity) -> Option<(*const u8, &ComponentTicks)> {
		let index = self.sparse.get_index(entity)? as usize;

		unsafe {
			Some((
				self.data.get_unchecked(index),
				self.ticks.get_unchecked(index),
			))
		}
	}

	pub fn get_with_ticks_mut(&mut self, entity: Entity) -> Option<(*mut u8, &mut ComponentTicks)> {
		let index = self.sparse.get_index(entity)? as usize;

		unsafe {
			Some((
				self.data.get_unchecked(index),
				self.ticks.get_unchecked_mut(index),
			))
		}
	}

	pub fn get_index(&self, entity: Entity) -> Option<u32> {
		self.sparse.get_index(entity)
	}

	pub fn contains(&self, entity: Entity) -> bool {
		self.sparse.contains(entity)
	}

	pub fn len(&self) -> usize {
		self.entities.len()
	}

	pub fn is_empty(&self) -> bool {
		self.entities.is_empty()
	}

	pub fn split(&self) -> (SparseArrayView, &[Entity], *const u8, &[ComponentTicks]) {
		(
			self.sparse.as_view(),
			self.entities.as_slice(),
			self.data.as_ptr(),
			self.ticks.as_slice(),
		)
	}

	pub fn split_mut(&mut self) -> (SparseArrayView, &[Entity], *mut u8, &mut [ComponentTicks]) {
		(
			self.sparse.as_view(),
			self.entities.as_slice(),
			self.data.as_ptr(),
			self.ticks.as_mut_slice(),
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn new() -> ComponentStorage {
		ComponentStorage::for_type::<i32>()
	}

	fn insert(
		storage: &mut ComponentStorage,
		entity: Entity,
		value: i32,
		tick: Ticks,
	) -> Option<i32> {
		unsafe {
			let prev = storage.insert_and_forget_prev(entity, &value as *const _ as *const _, tick);

			if !prev.is_null() {
				Some(ptr::read(prev.cast::<i32>()))
			} else {
				None
			}
		}
	}

	fn remove(storage: &mut ComponentStorage, entity: Entity) -> Option<i32> {
		let prev = storage.remove_and_forget(entity);

		if !prev.is_null() {
			unsafe { Some(ptr::read(prev.cast::<i32>())) }
		} else {
			None
		}
	}

	fn get(storage: &ComponentStorage, entity: Entity) -> Option<i32> {
		storage
			.get_with_ticks(entity)
			.map(|(value, _)| unsafe { *value.cast::<i32>() })
	}

	fn get_ticks(storage: &ComponentStorage, entity: Entity) -> Option<ComponentTicks> {
		storage.get_with_ticks(entity).map(|(_, ticks)| *ticks)
	}

	#[test]
	fn component_storage() {
		let mut storage = new();
		let e1 = Entity::with_index(10);
		let e2 = Entity::with_index(20);

		// Insert
		assert!(insert(&mut storage, e1, 1, 1).is_none());
		assert_eq!(get(&storage, e1).unwrap(), 1);
		assert_eq!(get_ticks(&storage, e1).unwrap(), ComponentTicks::new(1));

		assert!(insert(&mut storage, e2, 2, 2).is_none());
		assert_eq!(get(&storage, e1).unwrap(), 1);
		assert_eq!(get(&storage, e2).unwrap(), 2);
		assert_eq!(get_ticks(&storage, e1).unwrap(), ComponentTicks::new(1));
		assert_eq!(get_ticks(&storage, e2).unwrap(), ComponentTicks::new(2));

		// Swap
		storage.swap(0, 1);
		assert_eq!(get(&storage, e1).unwrap(), 1);
		assert_eq!(get(&storage, e2).unwrap(), 2);
		assert_eq!(get_ticks(&storage, e1).unwrap(), ComponentTicks::new(1));
		assert_eq!(get_ticks(&storage, e2).unwrap(), ComponentTicks::new(2));

		// Remove
		assert_eq!(remove(&mut storage, e1), Some(1));
		assert_eq!(storage.len(), 1);
		assert!(!storage.contains(e1));
		assert!(storage.contains(e2));

		assert_eq!(remove(&mut storage, e2), Some(2));
		assert_eq!(storage.len(), 0);
		assert!(!storage.contains(e1));
		assert!(!storage.contains(e2));
	}
}
