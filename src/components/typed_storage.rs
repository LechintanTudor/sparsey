use crate::components::{ComponentInfo, ComponentStorage, Entity, SparseArray};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::{mem, ptr, slice};

pub struct TypedComponentStorage<S, T>
where
	S: Deref<Target = ComponentStorage>,
	T: 'static,
{
	storage: S,
	_marker: PhantomData<T>,
}

unsafe impl<S, T> Send for TypedComponentStorage<S, T>
where
	S: Deref<Target = ComponentStorage>,
	T: Send + 'static,
{
}

unsafe impl<S, T> Sync for TypedComponentStorage<S, T>
where
	S: Deref<Target = ComponentStorage>,
	T: Sync + 'static,
{
}

impl<S, T> TypedComponentStorage<S, T>
where
	S: Deref<Target = ComponentStorage>,
	T: 'static,
{
	pub(crate) unsafe fn new(storage: S) -> Self {
		Self {
			storage,
			_marker: PhantomData,
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

	pub fn get_info(&self, entity: Entity) -> Option<&ComponentInfo> {
		self.storage.get_info(entity)
	}

	pub fn get_with_info(&self, entity: Entity) -> Option<(&T, &ComponentInfo)> {
		self.storage
			.get_with_info(entity)
			.map(|(value, info)| unsafe { (&*value.cast::<T>(), info) })
	}

	pub fn split(&self) -> (&SparseArray, &[Entity], &[ComponentInfo], &[T]) {
		let (sparse, entities, info, data) = self.storage.split();
		let data = unsafe { slice::from_raw_parts(data as *const T, entities.len()) };
		(sparse, entities, info, data)
	}
}

impl<S, T> TypedComponentStorage<S, T>
where
	S: Deref<Target = ComponentStorage> + DerefMut,
	T: 'static,
{
	pub(crate) fn insert(&mut self, entity: Entity, value: T, tick: u32) -> Option<T> {
		unsafe {
			let raw_value = &value as *const _ as *const _;
			let prev = self.storage.insert_and_forget_prev(entity, raw_value, tick);
			mem::forget(value);

			if !prev.is_null() {
				Some(ptr::read(prev.cast::<T>()))
			} else {
				None
			}
		}
	}

	pub(crate) fn remove(&mut self, entity: Entity) -> Option<T> {
		let value = self.storage.remove_and_forget(entity);

		if !value.is_null() {
			unsafe { Some(ptr::read(value.cast::<T>())) }
		} else {
			None
		}
	}

	pub(crate) fn delete(&mut self, entity: Entity) -> bool {
		self.storage.remove_and_drop(entity)
	}

	pub(crate) fn clear(&mut self) {
		self.storage.clear();
	}

	pub(crate) fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
		let value = self.storage.get_mut(entity);

		if !value.is_null() {
			unsafe { Some(&mut *value.cast::<T>()) }
		} else {
			None
		}
	}

	pub(crate) fn get_with_info_mut(
		&mut self,
		entity: Entity,
	) -> Option<(&mut T, &mut ComponentInfo)> {
		self.storage
			.get_with_info_mut(entity)
			.map(|(value, info)| unsafe { (&mut *value.cast::<T>(), info) })
	}

	pub fn split_mut(&mut self) -> (&SparseArray, &[Entity], &mut [ComponentInfo], &mut [T]) {
		let (sparse, entities, info, data) = self.storage.split_mut();
		let data = unsafe { slice::from_raw_parts_mut(data as *mut T, entities.len()) };
		(sparse, entities, info, data)
	}
}
