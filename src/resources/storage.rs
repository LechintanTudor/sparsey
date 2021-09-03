use crate::resources::Resource;
use crate::utils::ChangeTicks;
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use rustc_hash::FxHashMap;
use std::any::TypeId;
use std::hint::unreachable_unchecked;

pub struct ResourceCell {
	resource: Box<dyn Resource>,
	pub ticks: ChangeTicks,
}

impl ResourceCell {
	pub fn new(resource: Box<dyn Resource>, ticks: ChangeTicks) -> Self {
		Self { resource, ticks }
	}

	pub fn resource(&self) -> &dyn Resource {
		&self.resource
	}

	pub fn resource_mut(&mut self) -> &mut dyn Resource {
		&mut self.resource
	}
}

/// Maps `TypeIds` to type-erased `Resources`.
#[derive(Default)]
pub(crate) struct ResourceStorage {
	resources: FxHashMap<TypeId, AtomicRefCell<ResourceCell>>,
}

impl ResourceStorage {
	pub fn insert<T>(&mut self, resource: T, ticks: ChangeTicks) -> Option<T>
	where
		T: Resource,
	{
		let cell = ResourceCell::new(Box::new(resource), ticks);

		self.resources
			.insert(TypeId::of::<T>(), AtomicRefCell::new(cell))
			.map(|c| match c.into_inner().resource.downcast() {
				Ok(resource) => *resource,
				Err(_) => unsafe { unreachable_unchecked() },
			})
	}

	pub fn remove<T>(&mut self) -> Option<T>
	where
		T: Resource,
	{
		self.resources.remove(&TypeId::of::<T>()).map(|c| {
			match c.into_inner().resource.downcast() {
				Ok(resource) => *resource,
				Err(_) => unsafe { unreachable_unchecked() },
			}
		})
	}

	pub fn contains(&self, resource_type_id: &TypeId) -> bool {
		self.resources.contains_key(resource_type_id)
	}

	pub fn clear(&mut self) {
		self.resources.clear();
	}

	pub fn borrow(&self, resource_type_id: &TypeId) -> Option<AtomicRef<ResourceCell>> {
		self.resources
			.get(resource_type_id)
			.map(AtomicRefCell::borrow)
	}

	pub fn borrow_mut(&self, resource_type_id: &TypeId) -> Option<AtomicRefMut<ResourceCell>> {
		self.resources
			.get(resource_type_id)
			.map(AtomicRefCell::borrow_mut)
	}
}
