use crate::components::{ComponentStorage, ComponentTicks, Entity, SparseArrayView};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::{mem, ptr, slice};

pub(crate) struct TypedComponentStorage<S, T>
where
	S: Deref<Target = ComponentStorage>,
	T: 'static,
{
	storage: S,
	component: PhantomData<T>,
}

unsafe impl<S, T> Send for TypedComponentStorage<S, T>
where
	S: Deref<Target = ComponentStorage>,
	T: Send + 'static,
{
	// Empty
}

unsafe impl<S, T> Sync for TypedComponentStorage<S, T>
where
	S: Deref<Target = ComponentStorage>,
	T: Sync + 'static,
{
	// Empty
}

impl<S, T> TypedComponentStorage<S, T>
where
	S: Deref<Target = ComponentStorage>,
	T: 'static,
{
	pub unsafe fn new(storage: S) -> Self {
		Self {
			storage,
			component: PhantomData,
		}
	}

	pub fn contains(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	pub fn get(&self, entity: Entity) -> Option<&T> {
		let value = self.storage.get(entity);

		if !value.is_null() {
			unsafe { Some(&*value.cast::<T>()) }
		} else {
			None
		}
	}

	pub fn get_ticks(&self, entity: Entity) -> Option<&ComponentTicks> {
		self.storage.get_ticks(entity)
	}

	pub fn get_with_ticks(&self, entity: Entity) -> Option<(&T, &ComponentTicks)> {
		self.storage
			.get_with_ticks(entity)
			.map(|(value, ticks)| unsafe { (&*value.cast::<T>(), ticks) })
	}

	pub fn entities(&self) -> &[Entity] {
		self.storage.entities()
	}

	pub fn components(&self) -> &[T] {
		unsafe { slice::from_raw_parts(self.storage.components().cast::<T>(), self.storage.len()) }
	}

	pub fn ticks(&self) -> &[ComponentTicks] {
		self.storage.ticks()
	}

	pub fn split(&self) -> (SparseArrayView, &[Entity], &[T], &[ComponentTicks]) {
		let (sparse, entities, components, ticks) = self.storage.split();
		let components = unsafe { slice::from_raw_parts(components.cast::<T>(), entities.len()) };
		(sparse, entities, components, ticks)
	}
}

impl<S, T> Deref for TypedComponentStorage<S, T>
where
	S: Deref<Target = ComponentStorage>,
	T: 'static,
{
	type Target = [T];

	fn deref(&self) -> &Self::Target {
		unsafe { slice::from_raw_parts(self.storage.components().cast::<T>(), self.storage.len()) }
	}
}

impl<S, T> TypedComponentStorage<S, T>
where
	S: Deref<Target = ComponentStorage> + DerefMut,
	T: 'static,
{
	pub fn insert(&mut self, entity: Entity, value: T, ticks: ComponentTicks) -> Option<T> {
		unsafe {
			let raw_value = &value as *const _ as *const _;
			let prev = self
				.storage
				.insert_and_forget_prev(entity, raw_value, ticks);
			mem::forget(value);

			if !prev.is_null() {
				Some(ptr::read(prev.cast::<T>()))
			} else {
				None
			}
		}
	}

	pub fn remove(&mut self, entity: Entity) -> Option<T> {
		let value = self.storage.remove_and_forget(entity);

		if !value.is_null() {
			unsafe { Some(ptr::read(value.cast::<T>())) }
		} else {
			None
		}
	}

	pub fn get_with_ticks_mut(&mut self, entity: Entity) -> Option<(&mut T, &mut ComponentTicks)> {
		self.storage
			.get_with_ticks_mut(entity)
			.map(|(value, ticks)| unsafe { (&mut *value.cast::<T>(), ticks) })
	}

	#[allow(dead_code)]
	pub fn split_mut(&mut self) -> (SparseArrayView, &[Entity], &mut [T], &mut [ComponentTicks]) {
		let (sparse, entities, components, ticks) = self.storage.split_mut();
		let components = unsafe { slice::from_raw_parts_mut(components as *mut T, entities.len()) };
		(sparse, entities, components, ticks)
	}
}
