use crate::components::{
	BlobVec, ComponentTicks, Entity, IndexEntity, SparseArray, SparseArrayView, Ticks,
};
use std::alloc::Layout;
use std::ptr;

/// Maps entities to components stored as blobs of data.
pub struct ComponentStorage {
	sparse: SparseArray,
	entities: Vec<Entity>,
	ticks: Vec<ComponentTicks>,
	data: BlobVec,
}

impl ComponentStorage {
	/// Creates a storage suitable for storing components of type `T`.
	pub fn for_type<T>() -> Self
	where
		T: 'static,
	{
		unsafe { Self::new(Layout::new::<T>(), |ptr| ptr::drop_in_place::<T>(ptr as _)) }
	}

	/// Creates a storage suitable for storing components with the given layout
	/// and destructor.
	pub unsafe fn new(item_layout: Layout, drop_item: unsafe fn(*mut u8)) -> Self {
		Self {
			sparse: SparseArray::default(),
			entities: Vec::new(),
			ticks: Vec::new(),
			data: BlobVec::new(item_layout, drop_item),
		}
	}

	/// Sets the component of `entity` to `component` and returns the address of
	/// the previous component, which is valid until the next call to any of the
	/// storage's functions.
	pub unsafe fn insert_and_forget_prev(
		&mut self,
		entity: Entity,
		component: *const u8,
		tick: Ticks,
	) -> *mut u8 {
		let index_entity = self.sparse.get_mut_or_allocate_at(entity.index());

		match index_entity {
			Some(index_entity) => {
				let index = index_entity.index();
				*self.entities.get_unchecked_mut(index) = entity;
				self.ticks.get_unchecked_mut(index).set_tick_mutated(tick);
				self.data.set_and_forget_prev_unchecked(index, component)
			}
			None => {
				*index_entity = Some(IndexEntity::new(
					self.entities.len() as u32,
					entity.version(),
				));
				self.entities.push(entity);
				self.ticks.push(ComponentTicks::added(tick));
				self.data.push(component);
				ptr::null_mut()
			}
		}
	}

	/// Sets the component of `entity` to `component` and calls the desturctor
	/// of the previous component.
	pub unsafe fn insert_and_drop_prev(
		&mut self,
		entity: Entity,
		component: *const u8,
		tick: Ticks,
	) {
		let index_entity = self.sparse.get_mut_or_allocate_at(entity.index());

		match index_entity {
			Some(index_entity) => {
				let index = index_entity.index();
				*self.entities.get_unchecked_mut(index) = entity;
				self.ticks.get_unchecked_mut(index).set_tick_mutated(tick);
				self.data.set_and_drop_prev_unchecked(index, component);
			}
			None => {
				*index_entity = Some(IndexEntity::new(
					self.entities.len() as u32,
					entity.version(),
				));
				self.entities.push(entity);
				self.ticks.push(ComponentTicks::added(tick));
				self.data.push(component);
			}
		}
	}

	/// Removes the component at `entity` and return its address which is valid
	/// until the next call to any of the storage's functions.
	pub fn remove_and_forget(&mut self, entity: Entity) -> *mut u8 {
		let dense_index = match self.sparse.remove(entity) {
			Some(dense_index) => dense_index,
			None => return ptr::null_mut(),
		};

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

	/// Removes the component at `entity` and calls its destructor.
	pub fn remove_and_drop(&mut self, entity: Entity) -> bool {
		let dense_index = match self.sparse.remove(entity) {
			Some(dense_index) => dense_index,
			None => return false,
		};

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

	/// Returns the address of the component mapped to `entity`.
	pub fn get(&self, entity: Entity) -> *const u8 {
		match self.sparse.get_index(entity) {
			Some(index) => unsafe { self.data.get_unchecked(index as usize) },
			None => ptr::null(),
		}
	}

	/// Returns the address of the component mapped to `entity`.
	pub fn get_mut(&mut self, entity: Entity) -> *mut u8 {
		match self.sparse.get_index(entity) {
			Some(index) => unsafe { self.data.get_unchecked(index as usize) },
			None => ptr::null_mut(),
		}
	}

	/// Returns the `ComponentTicks` of the component mapped to `entity`.
	pub fn get_ticks(&self, entity: Entity) -> Option<&ComponentTicks> {
		let index = self.sparse.get_index(entity)? as usize;
		unsafe { Some(self.ticks.get_unchecked(index)) }
	}

	/// Returns the compnent and `ComponentTicks` mapped to `entity`.
	pub fn get_with_ticks(&self, entity: Entity) -> Option<(*const u8, &ComponentTicks)> {
		let index = self.sparse.get_index(entity)? as usize;

		unsafe {
			Some((
				self.data.get_unchecked(index),
				self.ticks.get_unchecked(index),
			))
		}
	}

	/// Returns the compnent and `ComponentTicks` mapped to `entity`.
	pub fn get_with_ticks_mut(&mut self, entity: Entity) -> Option<(*mut u8, &mut ComponentTicks)> {
		let index = self.sparse.get_index(entity)? as usize;

		unsafe {
			Some((
				self.data.get_unchecked(index),
				self.ticks.get_unchecked_mut(index),
			))
		}
	}

	/// Returns `true` if the storage contains `entity`.
	pub fn contains(&self, entity: Entity) -> bool {
		self.sparse.contains(entity)
	}

	/// Returns the index in the dense vector of `entity`.
	pub fn get_index(&self, entity: Entity) -> Option<usize> {
		self.sparse.get_index(entity)
	}

	/// Removes all entities and components in the storage.
	pub fn clear(&mut self) {
		self.sparse.clear();
		self.entities.clear();
		self.data.clear();
	}

	/// Swaps the entities at the given dense indexes.
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

	/// Returns the number of components in the storage.
	pub fn len(&self) -> usize {
		self.entities.len()
	}

	/// Returns `true` if the storage is empty.
	pub fn is_empty(&self) -> bool {
		self.entities.is_empty()
	}

	/// Returns a slice containing all the entities in the storage.
	pub fn entities(&self) -> &[Entity] {
		&self.entities
	}

	/// Returns a pointer to the type-erased components.
	pub fn data(&self) -> *const u8 {
		self.data.as_ptr()
	}

	/// Returns a tuple containing the `SparseArray`, `entities`, `components`
	/// and `component ticks` of the storage.
	pub fn split(&self) -> (SparseArrayView, &[Entity], *const u8, &[ComponentTicks]) {
		(
			self.sparse.as_view(),
			self.entities.as_slice(),
			self.data.as_ptr(),
			self.ticks.as_slice(),
		)
	}

	/// Returns a tuple containing the `SparseArray`, `entities`, `components`
	/// and `component ticks` of the storage.
	pub fn split_mut(&mut self) -> (SparseArrayView, &[Entity], *mut u8, &mut [ComponentTicks]) {
		(
			self.sparse.as_view(),
			self.entities.as_slice(),
			self.data.as_ptr(),
			self.ticks.as_mut_slice(),
		)
	}
}
