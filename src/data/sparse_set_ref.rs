use crate::data::{
	Component, ComponentFlags, ComponentRefMut, Entity, IndexEntity, SparseVec, VecRef, VecRefMut,
};
use std::mem;
use std::ops::{Deref, DerefMut};

/// Strongly-typed shared reference to a `TypeErasedSparseSet`.
pub struct SparseSetRef<'a, T>
where
	T: Component,
{
	sparse: &'a SparseVec,
	dense: &'a [Entity],
	flags: &'a [ComponentFlags],
	data: VecRef<'a, T>,
}

impl<'a, T> SparseSetRef<'a, T>
where
	T: Component,
{
	pub(crate) unsafe fn new(
		sparse: &'a SparseVec,
		dense: &'a [Entity],
		flags: &'a [ComponentFlags],
		data: VecRef<'a, T>,
	) -> Self {
		Self {
			sparse,
			dense,
			flags,
			data,
		}
	}

	/// Get s shared reference to the component at the given `Entity`.
	pub fn get(&self, entity: Entity) -> Option<&T> {
		self.sparse
			.get_index_entity(entity)
			.map(|e| unsafe { self.data.get_unchecked(e.index()) })
	}

	/// Get a slice of all entities which hold components in this sparse set.
	pub fn entities(&self) -> &[Entity] {
		self.dense
	}

	/// Split the sparse set into its sparse, dense, flags and data arrays.
	pub fn split(&self) -> (&SparseVec, &[Entity], &[ComponentFlags], &[T]) {
		(self.sparse, self.dense, self.flags, &self.data)
	}
}

impl<T> AsRef<[T]> for SparseSetRef<'_, T>
where
	T: Component,
{
	fn as_ref(&self) -> &[T] {
		&self.data
	}
}

impl<T> Deref for SparseSetRef<'_, T>
where
	T: Component,
{
	type Target = [T];

	fn deref(&self) -> &Self::Target {
		&self.data
	}
}

/// Strongly-typed exclusive reference to a `TypeErasedSparseSet`.
/// Enables adding and removing components.
pub struct SparseSetRefMut<'a, T>
where
	T: Component,
{
	sparse: &'a mut SparseVec,
	dense: &'a mut Vec<Entity>,
	flags: &'a mut Vec<ComponentFlags>,
	data: VecRefMut<'a, T>,
}

impl<'a, T> SparseSetRefMut<'a, T>
where
	T: Component,
{
	pub(crate) unsafe fn new(
		sparse: &'a mut SparseVec,
		dense: &'a mut Vec<Entity>,
		flags: &'a mut Vec<ComponentFlags>,
		data: VecRefMut<'a, T>,
	) -> Self {
		Self {
			sparse,
			dense,
			flags,
			data,
		}
	}

	/// Insert a new component at the given `Entity`
	/// and return the previous component, if any.
	pub fn insert(&mut self, entity: Entity, value: T) -> Option<T> {
		let index_entity = self.sparse.get_mut_or_allocate_at(entity.index());

		match index_entity {
			Some(e) => unsafe {
				if e.id() == entity.id() {
					self.flags
						.get_unchecked_mut(e.index())
						.insert(ComponentFlags::CHANGED);
				} else {
					self.flags
						.get_unchecked_mut(e.index())
						.insert(ComponentFlags::ADDED);
				}

				*e = IndexEntity::new(e.id(), entity.ver());
				Some(mem::replace(self.data.get_unchecked_mut(e.index()), value))
			},
			None => {
				*index_entity = Some(IndexEntity::new(self.dense.len() as u32, entity.ver()));
				self.dense.push(entity);
				self.flags.push(ComponentFlags::ADDED);
				self.data.push(value);
				None
			}
		}
	}

	/// Remove the component at the given `Entity`, if any, and return it.
	pub fn remove(&mut self, entity: Entity) -> Option<T> {
		let index_entity = self.sparse.get_index_entity(entity)?;

		let last_index = self.dense.last()?.index();
		self.dense.swap_remove(index_entity.index());
		self.flags.swap_remove(index_entity.index());

		unsafe {
			*self.sparse.get_unchecked_mut(last_index) = Some(index_entity);
			*self.sparse.get_unchecked_mut(entity.index()) = None;
		}

		Some(self.data.swap_remove(index_entity.index()))
	}

	/// Get a shared reference to the component at the given `Entity`, if any.
	pub fn get(&self, entity: Entity) -> Option<&T> {
		self.sparse
			.get_index_entity(entity)
			.map(|e| unsafe { self.data.get_unchecked(e.index()) })
	}

	/// Get an exclusive reference to the component at the given `Entity`, if any.
	pub fn get_mut(&mut self, entity: Entity) -> Option<ComponentRefMut<T>> {
		self.sparse.get_index_entity(entity).map(move |e| unsafe {
			ComponentRefMut::new(
				self.data.get_unchecked_mut(e.index()),
				self.flags.get_unchecked_mut(e.index()),
			)
		})
	}

	/// Get a slice of all entities which hold components in this sparse set.
	pub fn entities(&self) -> &[Entity] {
		self.dense
	}

	/// Split the sparse set into its sparse, dense, flags and data arrays.
	pub fn split(&self) -> (&SparseVec, &[Entity], &[ComponentFlags], &[T]) {
		(
			self.sparse,
			self.dense.as_slice(),
			self.flags.as_slice(),
			self.data.as_ref(),
		)
	}

	/// Mutably split the sparse set into its sparse, dense, flags and data arrays.
	pub fn split_mut(&mut self) -> (&SparseVec, &[Entity], &mut [ComponentFlags], &mut [T]) {
		(
			self.sparse,
			self.dense.as_slice(),
			self.flags.as_mut_slice(),
			self.data.as_mut(),
		)
	}
}

impl<T> AsRef<[T]> for SparseSetRefMut<'_, T>
where
	T: Component,
{
	fn as_ref(&self) -> &[T] {
		self.data.as_ref()
	}
}

impl<T> AsMut<[T]> for SparseSetRefMut<'_, T>
where
	T: Component,
{
	fn as_mut(&mut self) -> &mut [T] {
		self.data.as_mut()
	}
}

impl<T> Deref for SparseSetRefMut<'_, T>
where
	T: Component,
{
	type Target = [T];

	fn deref(&self) -> &Self::Target {
		self.data.as_ref()
	}
}

impl<T> DerefMut for SparseSetRefMut<'_, T>
where
	T: Component,
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data.as_mut()
	}
}
