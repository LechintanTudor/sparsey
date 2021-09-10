use crate::storage::{BlobVec, Entity, IndexEntity, SparseArray, SparseArrayView};
use crate::utils::ChangeTicks;
use std::alloc::Layout;
use std::ptr;

/// Maps entities to components stored as blobs of data.
pub struct ComponentStorage {
	sparse: SparseArray,
	entities: Vec<Entity>,
	components: BlobVec,
	ticks: Vec<ChangeTicks>,
}

impl ComponentStorage {
	/// Creates a storage suitable for storing components of type `T`.
	pub fn new<T>() -> Self
	where
		T: 'static,
	{
		unsafe {
			Self::from_layout_drop(Layout::new::<T>(), |ptr| ptr::drop_in_place::<T>(ptr as _))
		}
	}

	/// Creates a storage suitable for storing components with the given layout
	/// and destructor.
	pub unsafe fn from_layout_drop(layout: Layout, drop: unsafe fn(*mut u8)) -> Self {
		Self {
			sparse: SparseArray::default(),
			entities: Vec::new(),
			components: BlobVec::new(layout, drop),
			ticks: Vec::new(),
		}
	}

	/// Sets the component of `entity` to `component` and returns the address of
	/// the previous component, which is valid until the next call to any of the
	/// storage's functions.
	pub unsafe fn insert_and_forget_prev(
		&mut self,
		entity: Entity,
		component: *const u8,
		ticks: ChangeTicks,
	) -> *mut u8 {
		let index_entity = self.sparse.get_mut_or_allocate_at(entity.index());

		match index_entity {
			Some(index_entity) => {
				let index = index_entity.index();
				*self.entities.get_unchecked_mut(index) = entity;
				*self.ticks.get_unchecked_mut(index) = ticks;
				self.components
					.set_and_forget_prev_unchecked(index, component)
			}
			None => {
				*index_entity = Some(IndexEntity::new(
					self.entities.len() as u32,
					entity.version(),
				));
				self.entities.push(entity);
				self.ticks.push(ticks);
				self.components.push(component);
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
		ticks: ChangeTicks,
	) {
		let index_entity = self.sparse.get_mut_or_allocate_at(entity.index());

		match index_entity {
			Some(index_entity) => {
				let index = index_entity.index();
				*self.entities.get_unchecked_mut(index) = entity;
				*self.ticks.get_unchecked_mut(index) = ticks;
				self.components
					.set_and_drop_prev_unchecked(index, component);
			}
			None => {
				*index_entity = Some(IndexEntity::new(
					self.entities.len() as u32,
					entity.version(),
				));
				self.entities.push(entity);
				self.ticks.push(ticks);
				self.components.push(component);
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

		if let Some(entity) = self.entities.get(dense_index) {
			let new_index_entity = IndexEntity::new(dense_index as u32, entity.version());

			unsafe {
				*self.sparse.get_unchecked_mut(entity.index()) = Some(new_index_entity);
			}
		}

		unsafe {
			self.components
				.swap_remove_and_forget_unchecked(dense_index)
		}
	}

	/// Removes the component at `entity` and calls its destructor.
	pub fn remove_and_drop(&mut self, entity: Entity) -> bool {
		let dense_index = match self.sparse.remove(entity) {
			Some(dense_index) => dense_index,
			None => return false,
		};

		self.entities.swap_remove(dense_index);
		self.ticks.swap_remove(dense_index);

		if let Some(entity) = self.entities.get(dense_index) {
			let new_index_entity = IndexEntity::new(dense_index as u32, entity.version());

			unsafe {
				*self.sparse.get_unchecked_mut(entity.index()) = Some(new_index_entity);
			}
		}

		unsafe {
			self.components.swap_remove_and_drop_unchecked(dense_index);
		}

		true
	}

	/// Returns the address of the component mapped to `entity`.
	pub fn get(&self, entity: Entity) -> *const u8 {
		match self.sparse.get_index(entity) {
			Some(index) => unsafe { self.components.get_unchecked(index as usize) },
			None => ptr::null(),
		}
	}

	/// Returns the address of the component mapped to `entity`.
	pub fn get_mut(&mut self, entity: Entity) -> *mut u8 {
		match self.sparse.get_index(entity) {
			Some(index) => unsafe { self.components.get_unchecked(index as usize) },
			None => ptr::null_mut(),
		}
	}

	/// Returns the `ChangeTicks` of the component mapped to `entity`.
	pub fn get_ticks(&self, entity: Entity) -> Option<&ChangeTicks> {
		let index = self.sparse.get_index(entity)? as usize;
		unsafe { Some(self.ticks.get_unchecked(index)) }
	}

	/// Returns the compnent and `ChangeTicks` mapped to `entity`.
	pub fn get_with_ticks(&self, entity: Entity) -> Option<(*const u8, &ChangeTicks)> {
		let index = self.sparse.get_index(entity)? as usize;

		unsafe {
			Some((
				self.components.get_unchecked(index),
				self.ticks.get_unchecked(index),
			))
		}
	}

	/// Returns the compnent and `ChangeTicks` mapped to `entity`.
	pub fn get_with_ticks_mut(&mut self, entity: Entity) -> Option<(*mut u8, &mut ChangeTicks)> {
		let index = self.sparse.get_index(entity)? as usize;

		unsafe {
			Some((
				self.components.get_unchecked(index),
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
		self.ticks.clear();
		self.components.clear();
	}

	/// Swaps the entities at the given dense indexes.
	pub fn swap(&mut self, a: usize, b: usize) {
		let sparse_a = self.entities[a].index();
		let sparse_b = self.entities[b].index();

		unsafe {
			self.sparse.swap_unchecked(sparse_a, sparse_b);
			self.components.swap_unchecked(a, b);
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
	pub fn components(&self) -> *const u8 {
		self.components.as_ptr()
	}

	/// Returns a slice containing the ticks for all components in the storage.
	pub fn ticks(&self) -> &[ChangeTicks] {
		&self.ticks
	}

	/// Returns a tuple containing the `SparseArray`, `entities`, `components`
	/// and `component ticks` of the storage.
	pub fn split(&self) -> (SparseArrayView, &[Entity], *const u8, &[ChangeTicks]) {
		(
			self.sparse.as_view(),
			self.entities.as_slice(),
			self.components.as_ptr(),
			self.ticks.as_slice(),
		)
	}

	/// Returns a tuple containing the `SparseArray`, `entities`, `components`
	/// and `component ticks` of the storage.
	pub fn split_mut(&mut self) -> (SparseArrayView, &[Entity], *mut u8, &mut [ChangeTicks]) {
		(
			self.sparse.as_view(),
			self.entities.as_slice(),
			self.components.as_ptr(),
			self.ticks.as_mut_slice(),
		)
	}
}
