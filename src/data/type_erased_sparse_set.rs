use crate::data::{
	Component, ComponentFlags, Entity, IndexEntity, SparseSetRef, SparseSetRefMut, SparseVec,
	TypeErasedVec, TypeInfo,
};

/// Data structure which uses type erasure to map `Entities` to components.
/// Keeps components in a tightly packed array for fast iterations.
pub struct TypeErasedSparseSet {
	sparse: SparseVec,
	dense: Vec<Entity>,
	flags: Vec<ComponentFlags>,
	data: TypeErasedVec,
}

impl TypeErasedSparseSet {
	/// Create a new `TypeErasedSparseSet` for components of type `T`.
	pub fn new<T>() -> Self
	where
		T: Component,
	{
		Self {
			sparse: Default::default(),
			dense: Default::default(),
			flags: Default::default(),
			data: TypeErasedVec::new::<T>(),
		}
	}

	/// Get the `TypeInfo` for the type stored.
	pub fn type_info(&self) -> &TypeInfo {
		self.data.type_info()
	}

	/// Remove all the components.
	pub fn clear(&mut self) {
		self.sparse.clear();
		self.dense.clear();
		self.flags.clear();
		self.data.clear();
	}

	/// Clear flags for all components.
	pub fn clear_flags(&mut self) {
		self.flags
			.iter_mut()
			.for_each(|flags| *flags = ComponentFlags::empty());
	}

	/// Swap the components and the given indexes.
	pub fn swap(&mut self, a: usize, b: usize) {
		if a == b {
			return;
		}

		let sparse_index_a = self.dense[a].index();
		let sparse_index_b = self.dense[b].index();

		unsafe {
			self.sparse.swap_unchecked(sparse_index_a, sparse_index_b);
		}

		self.dense.swap(a, b);
		self.flags.swap(a, b);
		self.data.swap(a, b);
	}

	/// Delete the component at the given `Entity`.
	/// Return whether or not a component was removed.
	pub fn delete(&mut self, entity: Entity) -> bool {
		let index_entity = match self.sparse.get_index_entity(entity) {
			Some(index_entity) => index_entity,
			None => return false,
		};

		let last_index = match self.dense.last() {
			Some(entity) => entity.index(),
			None => return false,
		};

		self.dense.swap_remove(index_entity.index());
		self.flags.swap_remove(index_entity.index());

		unsafe {
			*self.sparse.get_unchecked_mut(last_index) = Some(index_entity);
			*self.sparse.get_unchecked_mut(entity.index()) = None;
		}

		self.data.swap_delete(index_entity.index());
		true
	}

	/// Get the number of components in the `TypeErasedSparseSet`.
	pub fn len(&self) -> usize {
		self.dense.len()
	}

	/// Check if the `TypeErasedSparseSet` contains the given `Entity`.
	pub fn contains(&self, entity: Entity) -> bool {
		self.sparse.contains(entity)
	}

	/// Get the `IndexEntity` for the given `Entity`, if any.
	pub fn get_index_entity(&self, entity: Entity) -> Option<IndexEntity> {
		self.sparse.get_index_entity(entity)
	}

	/// Get a strongly-typed shared reference to the `TypeErasedSparseSet`.
	/// Panics if the given type is not the same as the one used to create
	/// the `TypeErasedSparseSet`.
	pub fn to_ref<T>(&self) -> SparseSetRef<T>
	where
		T: Component,
	{
		unsafe { SparseSetRef::new(&self.sparse, &self.dense, &self.flags, self.data.as_ref()) }
	}

	/// Get a strongly-typed exclusive reference to the `TypeErasedSparseSet`.
	/// Panics if the given type is not the same as the one used to create
	/// the `TypeErasedSparseSet`.
	pub fn to_ref_mut<T>(&mut self) -> SparseSetRefMut<T>
	where
		T: Component,
	{
		unsafe {
			SparseSetRefMut::new(
				&mut self.sparse,
				&mut self.dense,
				&mut self.flags,
				self.data.as_mut(),
			)
		}
	}
}
