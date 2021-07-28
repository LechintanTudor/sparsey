use crate::components::{Component, ComponentTicks, Entity};
use crate::utils::EntityIterator;

#[derive(Clone, Copy)]
pub struct ComponentIter<'a, T> {
	index: usize,
	entities: &'a [Entity],
	components: *const T,
}

impl<'a, T> ComponentIter<'a, T> {
	pub(crate) unsafe fn new(entities: &'a [Entity], components: &'a [T]) -> Self {
		Self {
			index: 0,
			entities,
			components: components.as_ptr(),
		}
	}
}

impl<'a, T> Iterator for ComponentIter<'a, T>
where
	T: Component,
{
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		if self.index >= self.entities.len() {
			return None;
		}

		let current_index = self.index;
		self.index += 1;

		unsafe { Some(&*self.components.add(current_index)) }
	}
}

impl<'a, T> EntityIterator for ComponentIter<'a, T>
where
	T: Component,
{
	fn current_entity(&self) -> Option<Entity> {
		self.entities.get(self.index).copied()
	}
}

#[derive(Clone, Copy)]
pub struct ComponentAndTicksIter<'a, T> {
	index: usize,
	entities: &'a [Entity],
	components: *const T,
	ticks: *const ComponentTicks,
}

impl<'a, T> ComponentAndTicksIter<'a, T> {
	pub(crate) unsafe fn new(
		entities: &'a [Entity],
		components: &'a [T],
		ticks: &'a [ComponentTicks],
	) -> Self {
		Self {
			index: 0,
			entities,
			components: components.as_ptr(),
			ticks: ticks.as_ptr(),
		}
	}
}

impl<'a, T> Iterator for ComponentAndTicksIter<'a, T>
where
	T: Component,
{
	type Item = (&'a T, &'a ComponentTicks);

	fn next(&mut self) -> Option<Self::Item> {
		if self.index >= self.entities.len() {
			return None;
		}

		let current_index = self.index;
		self.index += 1;

		unsafe {
			Some((
				&*self.components.add(current_index),
				&*self.ticks.add(current_index),
			))
		}
	}
}

impl<'a, T> EntityIterator for ComponentAndTicksIter<'a, T>
where
	T: Component,
{
	fn current_entity(&self) -> Option<Entity> {
		self.entities.get(self.index).copied()
	}
}
