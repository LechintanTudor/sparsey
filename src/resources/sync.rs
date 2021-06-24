use crate::resources::{Res, ResMut, Resource, UnsafeResources};
use std::any::TypeId;

/// `Send + Sync` view over `Resources` which only allows safe accesses to
/// `Resources`.
#[derive(Copy, Clone)]
pub struct SyncResources<'a> {
	internal: &'a UnsafeResources,
}

impl<'a> SyncResources<'a> {
	pub(crate) fn new(internal: &'a UnsafeResources) -> Self {
		Self { internal }
	}

	/// Returns `true` if
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
		T: Resource + Sync,
	{
		unsafe { self.internal.borrow::<T>() }
	}

	/// Mutably borrows the `Resource` if it exists.
	pub fn borrow_mut<T>(&self) -> Option<ResMut<T>>
	where
		T: Resource + Send,
	{
		unsafe { self.internal.borrow_mut::<T>() }
	}
}
