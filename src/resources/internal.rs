use crate::resources::{Res, ResMut, Resource};
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use rustc_hash::FxHashMap;
use std::any::TypeId;
use std::hint::unreachable_unchecked;

/// Maps `TypeIds` to type-erased `Resources`. Unsafe because the struct itself
/// is `Send + Sync` but doesn't ensure inserted resources are `Send + Sync`
/// themselves.
#[derive(Default)]
pub struct UnsafeResources {
	values: FxHashMap<TypeId, AtomicRefCell<Box<dyn Resource>>>,
}

unsafe impl Send for UnsafeResources {}
unsafe impl Sync for UnsafeResources {}

impl UnsafeResources {
	/// Adds the `Resource` to the container and returns the previous one, if
	/// any.
	pub unsafe fn insert<T>(&mut self, resource: T) -> Option<Box<T>>
	where
		T: Resource,
	{
		self.insert_boxed(Box::new(resource))
	}

	/// Adds the boxed `Resource`to the container and returns the previous one,
	/// if any.
	pub unsafe fn insert_boxed<T>(&mut self, resource: Box<T>) -> Option<Box<T>>
	where
		T: Resource,
	{
		self.insert_dyn(resource).map(|res| match res.downcast() {
			Ok(res) => res,
			Err(_) => unreachable_unchecked(),
		})
	}

	/// Adds the type-erased `Resource` to the container and returns the
	/// previous one, if any.
	pub unsafe fn insert_dyn(&mut self, resource: Box<dyn Resource>) -> Option<Box<dyn Resource>> {
		let type_id = resource.type_id();

		self.values
			.insert(type_id, AtomicRefCell::new(resource))
			.map(|res| res.into_inner())
	}

	/// Removes the `Resource` from the container and returns it if it exists.
	pub unsafe fn remove<T>(&mut self) -> Option<Box<T>>
	where
		T: Resource,
	{
		self.remove_dyn(&TypeId::of::<T>())
			.map(|res| match res.downcast::<T>() {
				Ok(res) => res,
				Err(_) => unreachable_unchecked(),
			})
	}

	/// Removes the `Resource` with the given `TypeId` from the container and
	/// returns it if it exists.
	pub unsafe fn remove_dyn(&mut self, type_id: &TypeId) -> Option<Box<dyn Resource>> {
		self.values.remove(type_id).map(|res| res.into_inner())
	}

	/// Returns `true` if the container contains a `Resource` with the given
	/// `TypeId`.
	pub fn contains(&self, type_id: &TypeId) -> bool {
		self.values.contains_key(type_id)
	}

	/// Returns the number of `Resources` in the container.
	pub fn len(&self) -> usize {
		self.values.len()
	}

	/// Returns `true` if the container is empty.
	pub fn is_empty(&self) -> bool {
		self.values.is_empty()
	}

	/// Immutably borrows the `Resource` if it exists.
	pub unsafe fn borrow<T>(&self) -> Option<Res<T>>
	where
		T: Resource,
	{
		self.borrow_dyn(&TypeId::of::<T>()).map(|res| {
			Res::map(res, |res| match res.downcast_ref::<T>() {
				Some(res) => res,
				None => unreachable_unchecked(),
			})
		})
	}

	/// Mutably borrows the `Resource` if it exists.
	pub unsafe fn borrow_mut<T>(&self) -> Option<ResMut<T>>
	where
		T: Resource,
	{
		self.borrow_dyn_mut(&TypeId::of::<T>()).map(|res| {
			ResMut::map(res, |res| match res.downcast_mut::<T>() {
				Some(res) => res,
				None => unreachable_unchecked(),
			})
		})
	}

	/// Immutably Borrows the `Resource` with the given `TypeId` if it exists.
	pub unsafe fn borrow_dyn(&self, type_id: &TypeId) -> Option<Res<dyn Resource>> {
		self.values
			.get(type_id)
			.map(|res| Res::new(AtomicRef::map(res.borrow(), Box::as_ref)))
	}

	/// Mutably Borrows the `Resource` with the given `TypeId` if it exists.
	pub unsafe fn borrow_dyn_mut(&self, type_id: &TypeId) -> Option<ResMut<dyn Resource>> {
		self.values
			.get(type_id)
			.map(|res| ResMut::new(AtomicRefMut::map(res.borrow_mut(), Box::as_mut)))
	}

	/// Removes all the `Resources` in the container.
	pub unsafe fn clear(&mut self) {
		self.values.clear();
	}
}
