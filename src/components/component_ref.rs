use crate::components::{Component, ComponentInfo, Ticks};
use std::ops::{Deref, DerefMut};

pub struct ComponentRefMut<'a, T>
where
	T: Component,
{
	data: &'a mut T,
	info: &'a mut ComponentInfo,
	tick: Ticks,
}

impl<'a, T> ComponentRefMut<'a, T>
where
	T: Component,
{
	pub fn new(data: &'a mut T, info: &'a mut ComponentInfo, tick: Ticks) -> Self {
		Self { data, info, tick }
	}
}

impl<T> Deref for ComponentRefMut<'_, T>
where
	T: Component,
{
	type Target = T;

	fn deref(&self) -> &Self::Target {
		self.data
	}
}

impl<T> DerefMut for ComponentRefMut<'_, T>
where
	T: Component,
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.info.tick_mutated = self.tick;
		self.data
	}
}
