use bitflags::bitflags;
use std::ops::{Deref, DerefMut};

bitflags! {
	/// Bitflags used to mark component state changes.
	pub struct ComponentFlags: u8 {
		const ADDED   = 0b00000001;
		const CHANGED = 0b00000010;
	}
}

/// Wrapper for `&mut T` returned by `&mut CompMut<T>` queries
/// to enable granular change detection.
pub struct ComponentRefMut<'a, T> {
	data: &'a mut T,
	flags: &'a mut ComponentFlags,
}

impl<'a, T> ComponentRefMut<'a, T> {
	// Create a new `ComponentRefMut` with the given data and flags.
	pub fn new(data: &'a mut T, flags: &'a mut ComponentFlags) -> Self {
		Self { data, flags }
	}
}

impl<T> Deref for ComponentRefMut<'_, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.data
	}
}

impl<T> DerefMut for ComponentRefMut<'_, T> {
	/// Get an exclusive borrow to the data and mark it as changed.
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.flags.insert(ComponentFlags::CHANGED);
		self.data
	}
}
