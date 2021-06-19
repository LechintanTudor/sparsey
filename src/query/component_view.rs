use crate::components::{Component, ComponentTicks, Entity, SparseArrayView, Ticks};
use crate::query::ComponentRefMut;
use crate::world::{Comp, CompMut, GroupInfo};
use std::ops::Range;

pub type SplitComponentView<'a, T> = (
	SparseArrayView<'a>,
	&'a [Entity],
	*mut T,
	*mut ComponentTicks,
);

pub unsafe trait ComponentView<'a>
where
	Self: Sized,
{
	type Item;
	type Component: Component;

	fn get(self, entity: Entity) -> Option<Self::Item>;

	fn get_ticks(&self, entity: Entity) -> Option<&ComponentTicks>;

	fn matches(&self, entity: Entity) -> bool;

	fn group_info(&self) -> GroupInfo<'a>;

	fn world_tick(&self) -> Ticks;

	fn last_system_tick(&self) -> Ticks;

	fn into_parts(self) -> SplitComponentView<'a, Self::Component>;

	unsafe fn get_from_parts(
		data: *mut Self::Component,
		info: *mut ComponentTicks,
		index: usize,
		world_tick: Ticks,
		last_system_tick: Ticks,
	) -> Option<Self::Item>;
}

pub unsafe trait UnfilteredComponentView<'a>
where
	Self: ComponentView<'a>,
{
	// Empty
}

pub unsafe trait ImmutableUnfilteredComponentView<'a>
where
	Self: UnfilteredComponentView<'a>,
{
	unsafe fn slice_data(self, range: Range<usize>) -> &'a [Self::Component];

	unsafe fn slice_entities(self, range: Range<usize>) -> &'a [Entity];

	unsafe fn slice_entities_and_data(
		self,
		range: Range<usize>,
	) -> (&'a [Entity], &'a [Self::Component]);
}

unsafe impl<'a, T> ComponentView<'a> for &'a Comp<'a, T>
where
	T: Component,
{
	type Item = &'a T;
	type Component = T;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		self.storage.get(entity)
	}

	fn get_ticks(&self, entity: Entity) -> Option<&ComponentTicks> {
		self.storage.get_ticks(entity)
	}

	fn matches(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	fn group_info(&self) -> GroupInfo<'a> {
		self.group_info
	}

	fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	fn last_system_tick(&self) -> Ticks {
		self.last_system_tick
	}

	fn into_parts(self) -> SplitComponentView<'a, Self::Component> {
		let (sparse, entities, data, ticks) = self.storage.split();
		(sparse, entities, data.as_ptr() as _, ticks.as_ptr() as _)
	}

	unsafe fn get_from_parts(
		data: *mut Self::Component,
		_info: *mut ComponentTicks,
		index: usize,
		_world_tick: Ticks,
		_last_system_tick: Ticks,
	) -> Option<Self::Item> {
		Some(&*data.add(index))
	}
}

unsafe impl<'a, T> UnfilteredComponentView<'a> for &'a Comp<'a, T>
where
	T: Component,
{
	// Empty
}

unsafe impl<'a, T> ImmutableUnfilteredComponentView<'a> for &'a Comp<'a, T>
where
	T: Component,
{
	unsafe fn slice_data(self, range: Range<usize>) -> &'a [Self::Component] {
		self.storage.data().get_unchecked(range)
	}

	unsafe fn slice_entities(self, range: Range<usize>) -> &'a [Entity] {
		self.storage.entities().get_unchecked(range)
	}

	unsafe fn slice_entities_and_data(
		self,
		range: Range<usize>,
	) -> (&'a [Entity], &'a [Self::Component]) {
		(
			self.storage.entities().get_unchecked(range.clone()),
			self.storage.data().get_unchecked(range),
		)
	}
}

unsafe impl<'a, T> ComponentView<'a> for &'a CompMut<'a, T>
where
	T: Component,
{
	type Item = &'a T;
	type Component = T;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		self.storage.get(entity)
	}

	fn get_ticks(&self, entity: Entity) -> Option<&ComponentTicks> {
		self.storage.get_ticks(entity)
	}

	fn matches(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	fn group_info(&self) -> GroupInfo<'a> {
		self.group_info
	}

	fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	fn last_system_tick(&self) -> Ticks {
		self.last_system_tick
	}

	fn into_parts(self) -> SplitComponentView<'a, Self::Component> {
		let (sparse, entities, data, ticks) = self.storage.split();
		(sparse, entities, data.as_ptr() as _, ticks.as_ptr() as _)
	}

	unsafe fn get_from_parts(
		data: *mut Self::Component,
		_info: *mut ComponentTicks,
		index: usize,
		_world_tick: Ticks,
		_last_system_tick: Ticks,
	) -> Option<Self::Item> {
		Some(&*data.add(index))
	}
}

unsafe impl<'a, T> UnfilteredComponentView<'a> for &'a CompMut<'a, T>
where
	T: Component,
{
	// Empty
}

unsafe impl<'a, T> ImmutableUnfilteredComponentView<'a> for &'a CompMut<'a, T>
where
	T: Component,
{
	unsafe fn slice_data(self, range: Range<usize>) -> &'a [Self::Component] {
		self.storage.data().get_unchecked(range)
	}

	unsafe fn slice_entities(self, range: Range<usize>) -> &'a [Entity] {
		self.storage.entities().get_unchecked(range)
	}

	unsafe fn slice_entities_and_data(
		self,
		range: Range<usize>,
	) -> (&'a [Entity], &'a [Self::Component]) {
		(
			self.storage.entities().get_unchecked(range.clone()),
			self.storage.data().get_unchecked(range),
		)
	}
}

unsafe impl<'a, 'b, T> ComponentView<'a> for &'a mut CompMut<'b, T>
where
	T: Component,
{
	type Item = ComponentRefMut<'a, T>;
	type Component = T;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		let (data, ticks) = self.storage.get_with_ticks_mut(entity)?;
		Some(ComponentRefMut::new(data, ticks, self.world_tick))
	}

	fn get_ticks(&self, entity: Entity) -> Option<&ComponentTicks> {
		self.storage.get_ticks(entity)
	}

	fn matches(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	fn group_info(&self) -> GroupInfo<'a> {
		self.group_info
	}

	fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	fn last_system_tick(&self) -> Ticks {
		self.last_system_tick
	}

	fn into_parts(self) -> SplitComponentView<'a, Self::Component> {
		let (sparse, entities, data, ticks) = self.storage.split();
		(sparse, entities, data.as_ptr() as _, ticks.as_ptr() as _)
	}

	unsafe fn get_from_parts(
		data: *mut Self::Component,
		info: *mut ComponentTicks,
		index: usize,
		world_tick: Ticks,
		_last_system_tick: Ticks,
	) -> Option<Self::Item> {
		Some(ComponentRefMut::new(
			&mut *data.add(index),
			&mut *info.add(index),
			world_tick,
		))
	}
}

unsafe impl<'a, 'b, T> UnfilteredComponentView<'a> for &'a mut CompMut<'b, T>
where
	T: Component,
{
	// Empty
}
