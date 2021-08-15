use crate::resources::Resource;
use crate::utils::ChangeTicks;
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use rustc_hash::FxHashMap;
use std::any::TypeId;
use std::hint::unreachable_unchecked;

pub(crate) struct ResourceCell {
	value: Box<dyn Resource>,
	pub ticks: ChangeTicks,
}

impl ResourceCell {
	pub fn value(&self) -> &dyn Resource {
		&self.value
	}

	pub fn value_mut(&mut self) -> &mut dyn Resource {
		&mut self.value
	}
}

/// Maps `TypeIds` to type-erased `Resources`.
#[derive(Default)]
pub(crate) struct Resources {
	resources: FxHashMap<TypeId, AtomicRefCell<ResourceCell>>,
}

impl Resources {
	pub fn insert<T>(&mut self, value: T, ticks: ChangeTicks) -> Option<T>
	where
		T: Resource,
	{
		let cell = ResourceCell {
			value: Box::new(value),
			ticks,
		};

		self.resources
			.insert(TypeId::of::<T>(), AtomicRefCell::new(cell))
			.map(|c| match c.into_inner().value.downcast().map(|c| *c) {
				Ok(value) => value,
				Err(_) => unsafe { unreachable_unchecked() },
			})
	}

	pub fn remove<T>(&mut self) -> Option<T>
	where
		T: Resource,
	{
		self.resources.remove(&TypeId::of::<T>()).map(|c| {
			match c.into_inner().value.downcast().map(|c| *c) {
				Ok(value) => value,
				Err(_) => unsafe { unreachable_unchecked() },
			}
		})
	}

	pub fn clear(&mut self) {
		self.resources.clear();
	}

	pub fn borrow<T>(&self) -> Option<AtomicRef<ResourceCell>>
	where
		T: Resource,
	{
		self.resources.get(&TypeId::of::<T>()).map(|c| c.borrow())
	}

	pub fn borrow_mut<T>(&self) -> Option<AtomicRefMut<ResourceCell>>
	where
		T: Resource,
	{
		self.resources
			.get(&TypeId::of::<T>())
			.map(|c| c.borrow_mut())
	}
}
