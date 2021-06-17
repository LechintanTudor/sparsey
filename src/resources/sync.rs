use crate::resources::{Res, ResMut, Resource, UnsafeResources};
use std::any::TypeId;

/// Safe view over a set of resources.
/// The struct itself is `Send` and `Sync`.
#[derive(Copy, Clone)]
pub struct SyncResources<'a> {
	internal: &'a UnsafeResources,
}

impl<'a> SyncResources<'a> {
	pub(crate) fn new(internal: &'a UnsafeResources) -> Self {
		Self { internal }
	}

	/// Check if the set contains a resource at the given `TypeId`.
	pub fn contains(&self, type_id: &TypeId) -> bool {
		self.internal.contains(type_id)
	}

	/// Get the number of resources in the set.
	pub fn len(&self) -> usize {
		self.internal.len()
	}

	pub fn is_empty(&self) -> bool {
		self.internal.is_empty()
	}

	/// Get a shared borrow of a resource if it exists.
	/// Safe because `T` is guaranteed to be `Sync`.
	pub fn borrow<T>(&self) -> Option<Res<T>>
	where
		T: Resource + Sync,
	{
		unsafe { self.internal.borrow::<T>() }
	}

	/// Get an exclusive borrow of a resource if it exists.
	/// Safe because `T` is guaranteed to be `Send`.
	pub fn borrow_mut<T>(&self) -> Option<ResMut<T>>
	where
		T: Resource + Send,
	{
		unsafe { self.internal.borrow_mut::<T>() }
	}
}
