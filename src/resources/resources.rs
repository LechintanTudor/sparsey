use crate::resources::{Res, ResMut, Resource, SyncResources, UnsafeResources};
use std::any::TypeId;
use std::marker::PhantomData;

/// Maps `TypeIds` to type-erased `Resources`.
#[derive(Default)]
pub struct Resources {
	internal: UnsafeResources,
	_non_send_sync: PhantomData<*const ()>,
}

impl Resources {
	/// Returns a view over the `Resources` which can be safely shared across
	/// threads.
	pub fn sync(&self) -> SyncResources {
		SyncResources::new(&self.internal)
	}

	/// Returns the `UnsafeResources` used internally by this container.
	pub unsafe fn internal(&self) -> &UnsafeResources {
		&self.internal
	}

	/// Adds the `Resource` to the container and returns the previous one, if
	/// any.
	pub fn insert<T>(&mut self, resource: T) -> Option<Box<T>>
	where
		T: Resource,
	{
		unsafe { self.internal.insert(resource) }
	}

	/// Adds the boxed `Resource`to the container and returns the previous one,
	/// if any.
	pub fn insert_boxed<T>(&mut self, resource: Box<T>) -> Option<Box<T>>
	where
		T: Resource,
	{
		unsafe { self.internal.insert_boxed(resource) }
	}

	/// Adds the type-erased `Resource` to the container and returns the
	/// previous one, if any.
	pub unsafe fn insert_dyn(&mut self, resource: Box<dyn Resource>) -> Option<Box<dyn Resource>> {
		self.internal.insert_dyn(resource)
	}

	/// Removes the `Resource` from the container and returns it if it exists.
	pub fn remove<T>(&mut self) -> Option<Box<T>>
	where
		T: Resource,
	{
		unsafe { self.internal.remove::<T>() }
	}

	/// Removes the `Resource` with the given `TypeId` from the container and
	/// returns it if it exists.
	pub fn remove_dyn(&mut self, type_id: &TypeId) -> Option<Box<dyn Resource>> {
		unsafe { self.internal.remove_dyn(type_id) }
	}

	/// Returns `true` if the container contains a `Resource` with the given
	/// `TypeId`.
	pub fn contains(&self, type_id: &TypeId) -> bool {
		self.internal.contains(type_id)
	}

	/// Returns the number of `Resources` in the container.
	pub fn len(&self) -> usize {
		self.internal.len()
	}

	/// Returns `true` if the container is empty.
	pub fn is_empty(&self) -> bool {
		self.internal.is_empty()
	}

	/// Immutably borrows the `Resource` if it exists.
	pub fn borrow<T>(&self) -> Option<Res<T>>
	where
		T: Resource,
	{
		unsafe { self.internal.borrow() }
	}

	/// Mutably borrows the `Resource` if it exists.
	pub fn borrow_mut<T>(&self) -> Option<ResMut<T>>
	where
		T: Resource,
	{
		unsafe { self.internal.borrow_mut() }
	}

	/// Immutably Borrows the `Resource` with the given `TypeId` if it exists.
	pub fn borrow_dyn(&self, type_id: &TypeId) -> Option<Res<dyn Resource>> {
		unsafe { self.internal.borrow_dyn(type_id) }
	}

	/// Mutably Borrows the `Resource` with the given `TypeId` if it exists.
	pub fn borrow_dyn_mut(&self, type_id: &TypeId) -> Option<ResMut<dyn Resource>> {
		unsafe { self.internal.borrow_dyn_mut(type_id) }
	}

	/// Removes all the `Resources` in the container.
	pub fn clear(&mut self) {
		unsafe {
			self.internal.clear();
		}
	}
}
