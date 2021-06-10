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
				self.ticks.get_unchecked_mut(index).set_tick_mutated(tick);
				self.data.set_and_forget_prev_unchecked(index, value)
			}
			None => {
				*index_entity = Some(IndexEntity::new(
					self.entities.len() as u32,
					entity.version(),
				));
				self.entities.push(entity);
				self.ticks.push(ComponentTicks::added(tick));
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
				self.ticks.get_unchecked_mut(index).set_tick_mutated(tick);
				self.data.set_and_drop_prev_unchecked(index, value);
			}
			None => {
				*index_entity = Some(IndexEntity::new(
					self.entities.len() as u32,
					entity.version(),
				));
				self.entities.push(entity);
				self.ticks.push(ComponentTicks::added(tick));
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
