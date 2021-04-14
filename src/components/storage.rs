use crate::components::{BlobVec, ComponentInfo, Entity, SparseArray};
use std::alloc::Layout;
use std::ptr;

pub struct ComponentStorage {
	indexes: SparseArray,
	entities: Vec<Entity>,
	info: Vec<ComponentInfo>,
	data: BlobVec,
}

impl ComponentStorage {
	pub unsafe fn new(item_layout: Layout, drop_item: unsafe fn(*mut u8)) -> Self {
		Self {
			indexes: SparseArray::new(),
			entities: Vec::new(),
			info: Vec::new(),
			data: BlobVec::new(item_layout, drop_item),
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

	pub unsafe fn remove_and_forget(&mut self, entity: Entity) -> *mut u8 {
		// Get index only if entity versions match
		let index_ref = match self.indexes.get_mut(entity.index()) {
			Some(index_ref) => {
				if self.entities.get_unchecked(*index_ref).version() == entity.version() {
					index_ref
				} else {
					return ptr::null_mut();
				}
			}
			None => return ptr::null_mut(),
		};

		let index = *index_ref;
		*index_ref = SparseArray::INVALID_INDEX;

		self.entities.swap_remove(index);
		self.data.swap_remove_and_forget_unchecked(index)
	}

	pub unsafe fn remove_and_drop(&mut self, entity: Entity) -> bool {
		// Get index only if entity versions match
		let index_ref = match self.indexes.get_mut(entity.index()) {
			Some(index_ref) => {
				if self.entities.get_unchecked(*index_ref).version() == entity.version() {
					index_ref
				} else {
					return false;
				}
			}
			None => return false,
		};

		let index = *index_ref;
		*index_ref = SparseArray::INVALID_INDEX;

		self.entities.swap_remove(index);
		self.data.swap_remove_and_drop_unchecked(index);
		true
	}

	pub fn get_with_info(&self, entity: Entity) -> Option<(*const u8, &ComponentInfo)> {
		let index = *self.index_for(entity)?;

		unsafe {
			Some((
				self.data.get_unchecked(index),
				self.info.get_unchecked(index),
			))
		}
	}

	pub fn get_with_info_mut(&mut self, entity: Entity) -> Option<(*mut u8, &ComponentInfo)> {
		let index = *self.index_for(entity)?;

		unsafe {
			Some((
				self.data.get_unchecked(index),
				self.info.get_unchecked_mut(index),
			))
		}
	}

	pub fn contains(&self, entity: Entity) -> bool {
		self.index_for(entity).is_some()
	}

	pub fn clear(&mut self) {
		self.indexes.clear();
		self.entities.clear();
		self.data.clear();
	}

	pub fn len(&self) -> usize {
		self.entities.len()
	}

	fn index_for(&self, entity: Entity) -> Option<&usize> {
		self.indexes
			.get(entity.index())
			.filter(|&&i| unsafe { self.entities.get_unchecked(i).version() == entity.version() })
	}
}
