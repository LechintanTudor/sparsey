use crate::components::{BlobVec, ComponentInfo, Entity, SparseArray};
use std::alloc::Layout;
use std::{mem, ptr};

pub struct ComponentStorage {
	indexes: SparseArray,
	entities: Vec<Entity>,
	data: BlobVec,
	info: Vec<ComponentInfo>,
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
			indexes: SparseArray::new(),
			entities: Vec::new(),
			data: BlobVec::new(item_layout, drop_item),
			info: Vec::new(),
		}
	}

	pub unsafe fn insert_and_forget_prev(
		&mut self,
		entity: Entity,
		value: *const u8,
		tick: u32,
	) -> *mut u8 {
		let index = self.indexes.get_mut_or_invalid(entity.index());

		if *index == SparseArray::INVALID_INDEX {
			*index = self.entities.len();
			self.entities.push(entity);
			self.info.push(ComponentInfo::new(tick));
			self.data.push(value);
			ptr::null_mut()
		} else {
			let index = *index;
			*self.entities.get_unchecked_mut(index) = entity;
			self.info.get_unchecked_mut(index).tick_mutated = tick;
			self.data.set_and_forget_prev_unchecked(index, value)
		}
	}

	pub unsafe fn insert_and_drop_prev(
		&mut self,
		entity: Entity,
		value: *const u8,
		tick: u32,
	) -> bool {
		let index = self.indexes.get_mut_or_invalid(entity.index());

		if *index == SparseArray::INVALID_INDEX {
			*index = self.entities.len();
			self.entities.push(entity);
			self.info.push(ComponentInfo::new(tick));
			self.data.push(value);
			true
		} else {
			let index = *index;
			*self.entities.get_unchecked_mut(index) = entity;
			self.info.get_unchecked_mut(index).tick_mutated = tick;
			self.data.set_and_drop_prev_unchecked(index, value);
			false
		}
	}

	pub fn remove_and_forget(&mut self, entity: Entity) -> *mut u8 {
		let dense_index = match self.indexes.get_mut(entity.index()) {
			Some(dense_index) => unsafe {
				if self.entities.get_unchecked(*dense_index).version() == entity.version() {
					dense_index
				} else {
					return ptr::null_mut();
				}
			},
			None => return ptr::null_mut(),
		};

		let dense_index = mem::replace(dense_index, SparseArray::INVALID_INDEX);
		self.entities.swap_remove(dense_index);
		self.info.swap_remove(dense_index);

		if let Some(entity) = self.entities.last() {
			unsafe {
				*self.indexes.get_unchecked_mut(entity.index()) = self.entities.len() - 1;
			}
		}

		unsafe { self.data.swap_remove_and_forget_unchecked(dense_index) }
	}

	pub fn remove_and_drop(&mut self, entity: Entity) -> bool {
		let dense_index = match self.indexes.get_mut(entity.index()) {
			Some(dense_index) => unsafe {
				if self.entities.get_unchecked(*dense_index).version() == entity.version() {
					dense_index
				} else {
					return false;
				}
			},
			None => return false,
		};

		let dense_index = mem::replace(dense_index, SparseArray::INVALID_INDEX);
		self.entities.swap_remove(dense_index);
		self.info.swap_remove(dense_index);

		if let Some(entity) = self.entities.last() {
			unsafe {
				*self.indexes.get_unchecked_mut(entity.index()) = self.entities.len() - 1;
			}
		}

		unsafe {
			self.data.swap_remove_and_drop_unchecked(dense_index);
		}

		true
	}

	pub fn swap(&mut self, a: usize, b: usize) {
		let sparse_a = self.entities[a].index();
		let sparse_b = self.entities[b].index();

		unsafe {
			self.indexes.swap_unchecked(sparse_a, sparse_b);
			self.data.swap_unchecked(a, b);
		}

		self.entities.swap(a, b);
		self.info.swap(a, b);
	}

	pub fn get_with_info(&self, entity: Entity) -> Option<(*const u8, &ComponentInfo)> {
		let index = *self.dense_index_of(entity)?;

		unsafe {
			Some((
				self.data.get_unchecked(index),
				self.info.get_unchecked(index),
			))
		}
	}

	pub fn get_with_info_mut(&mut self, entity: Entity) -> Option<(*mut u8, &mut ComponentInfo)> {
		let index = *self.dense_index_of(entity)?;

		unsafe {
			Some((
				self.data.get_unchecked(index),
				self.info.get_unchecked_mut(index),
			))
		}
	}

	pub fn contains(&self, entity: Entity) -> bool {
		self.dense_index_of(entity).is_some()
	}

	pub fn clear(&mut self) {
		self.indexes.clear();
		self.entities.clear();
		self.data.clear();
	}

	pub fn len(&self) -> usize {
		self.entities.len()
	}

	pub fn is_empty(&self) -> bool {
		self.entities.is_empty()
	}

	pub fn dense_index_of(&self, entity: Entity) -> Option<&usize> {
		self.indexes
			.get(entity.index())
			.filter(|&&i| unsafe { self.entities.get_unchecked(i).version() == entity.version() })
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
		tick: u32,
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
			.get_with_info(entity)
			.map(|(value, _)| unsafe { *value.cast::<i32>() })
	}

	fn get_info(storage: &ComponentStorage, entity: Entity) -> Option<ComponentInfo> {
		storage.get_with_info(entity).map(|(_, info)| *info)
	}

	fn print_info(storage: &ComponentStorage) {
		unsafe {
			println!("e1: {:?}", storage.indexes.get(10));
			println!("e2: {:?}", storage.indexes.get(20));
			println!(
				"Slice: {:?}",
				std::slice::from_raw_parts(storage.data.as_ptr() as *const i32, storage.len(),)
			);
		}
	}

	#[test]
	fn component_storage() {
		let mut storage = new();
		let e1 = Entity::with_index(10);
		let e2 = Entity::with_index(20);

		// Insert
		assert!(insert(&mut storage, e1, 1, 1).is_none());
		assert_eq!(get(&storage, e1).unwrap(), 1);
		assert_eq!(get_info(&storage, e1).unwrap(), ComponentInfo::new(1));

		assert!(insert(&mut storage, e2, 2, 2).is_none());
		assert_eq!(get(&storage, e1).unwrap(), 1);
		assert_eq!(get(&storage, e2).unwrap(), 2);
		assert_eq!(get_info(&storage, e1).unwrap(), ComponentInfo::new(1));
		assert_eq!(get_info(&storage, e2).unwrap(), ComponentInfo::new(2));

		// Swap
		storage.swap(0, 1);
		assert_eq!(get(&storage, e1).unwrap(), 1);
		assert_eq!(get(&storage, e2).unwrap(), 2);
		assert_eq!(get_info(&storage, e1).unwrap(), ComponentInfo::new(1));
		assert_eq!(get_info(&storage, e2).unwrap(), ComponentInfo::new(2));

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
