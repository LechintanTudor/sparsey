use crate::components::{Component, ComponentStorage};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub struct TypedComponentStorage<S, T>
where
	S: Deref<Target = ComponentStorage>,
	T: Component,
{
	storage: S,
	_marker: PhantomData<T>,
}

unsafe impl<S, T> Send for TypedComponentStorage<S, T>
where
	S: Deref<Target = ComponentStorage>,
	T: Component,
{
}

unsafe impl<S, T> Sync for TypedComponentStorage<S, T>
where
	S: Deref<Target = ComponentStorage>,
	T: Component,
{
}

impl<S, T> TypedComponentStorage<S, T>
where
	S: Deref<Target = ComponentStorage>,
	T: Component,
{
	pub(crate) unsafe fn new(storage: S) -> Self {
		Self {
			storage,
			_marker: PhantomData,
		}
	}
}

impl<S, T> TypedComponentStorage<S, T>
where
	S: Deref<Target = ComponentStorage> + DerefMut,
	T: Component,
{
}
