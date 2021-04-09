use crate::components::Component;
use std::marker::PhantomData;

pub struct ComponentStorageRef<'a, T>
where
	T: Component,
{
	_marker: PhantomData<&'a [T]>,
}

pub struct ComponentStorageRefMut<'a, T>
where
	T: Component,
{
	_marker: PhantomData<&'a mut [T]>,
}
