use crate::components::Component;
use crate::group::GroupInfo;
use crate::query::ComponentRefMut;
use crate::storage::{Entity, SparseArrayView};
use crate::utils::{ChangeTicks, Ticks};
use crate::world::{Comp, CompMut};
use std::ops::Range;

/// Component view split into its parts.
pub type SplitComponentView<'a, T> = (SparseArrayView<'a>, &'a [Entity], *mut T, *mut ChangeTicks);

/// Trait implemented by views over component storages.
pub unsafe trait ComponentView<'a>
where
	Self: Sized,
{
	type Item;
	type Component: Component;

	fn get(self, entity: Entity) -> Option<Self::Item>;

	fn get_ticks(&self, entity: Entity) -> Option<&ChangeTicks>;

	fn contains(&self, entity: Entity) -> bool;

	fn group_info(&self) -> GroupInfo<'a>;

	fn world_tick(&self) -> Ticks;

	fn change_tick(&self) -> Ticks;

	fn into_parts(self) -> SplitComponentView<'a, Self::Component>;

	unsafe fn get_from_parts(
		components: *mut Self::Component,
		info: *mut ChangeTicks,
		index: usize,
		world_tick: Ticks,
		change_tick: Ticks,
	) -> Option<Self::Item>;
}

/// Trait implemented by unfiltered component views.
pub unsafe trait UnfilteredComponentView<'a>
where
	Self: ComponentView<'a>,
{
	// Empty
}

/// Trait implemented by immutable unfiltered component views.
pub unsafe trait ImmutableUnfilteredComponentView<'a>
where
	Self: UnfilteredComponentView<'a>,
{
	unsafe fn slice_components(self, range: Range<usize>) -> &'a [Self::Component];

	unsafe fn slice_entities(self, range: Range<usize>) -> &'a [Entity];

	unsafe fn slice_entities_and_components(
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

	fn get_ticks(&self, entity: Entity) -> Option<&ChangeTicks> {
		self.storage.get_ticks(entity)
	}

	fn contains(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	fn group_info(&self) -> GroupInfo<'a> {
		self.group_info
	}

	fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	fn change_tick(&self) -> Ticks {
		self.change_tick
	}

	fn into_parts(self) -> SplitComponentView<'a, Self::Component> {
		let (sparse, entities, components, ticks) = self.storage.split();
		(
			sparse,
			entities,
			components.as_ptr() as _,
			ticks.as_ptr() as _,
		)
	}

	unsafe fn get_from_parts(
		components: *mut Self::Component,
		_info: *mut ChangeTicks,
		index: usize,
		_world_tick: Ticks,
		_last_system_tick: Ticks,
	) -> Option<Self::Item> {
		Some(&*components.add(index))
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
	unsafe fn slice_components(self, range: Range<usize>) -> &'a [Self::Component] {
		self.storage.components().get_unchecked(range)
	}

	unsafe fn slice_entities(self, range: Range<usize>) -> &'a [Entity] {
		self.storage.entities().get_unchecked(range)
	}

	unsafe fn slice_entities_and_components(
		self,
		range: Range<usize>,
	) -> (&'a [Entity], &'a [Self::Component]) {
		(
			self.storage.entities().get_unchecked(range.clone()),
			self.storage.components().get_unchecked(range),
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

	fn get_ticks(&self, entity: Entity) -> Option<&ChangeTicks> {
		self.storage.get_ticks(entity)
	}

	fn contains(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	fn group_info(&self) -> GroupInfo<'a> {
		self.group_info
	}

	fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	fn change_tick(&self) -> Ticks {
		self.change_tick
	}

	fn into_parts(self) -> SplitComponentView<'a, Self::Component> {
		let (sparse, entities, components, ticks) = self.storage.split();
		(
			sparse,
			entities,
			components.as_ptr() as _,
			ticks.as_ptr() as _,
		)
	}

	unsafe fn get_from_parts(
		components: *mut Self::Component,
		_info: *mut ChangeTicks,
		index: usize,
		_world_tick: Ticks,
		_last_system_tick: Ticks,
	) -> Option<Self::Item> {
		Some(&*components.add(index))
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
	unsafe fn slice_components(self, range: Range<usize>) -> &'a [Self::Component] {
		self.storage.components().get_unchecked(range)
	}

	unsafe fn slice_entities(self, range: Range<usize>) -> &'a [Entity] {
		self.storage.entities().get_unchecked(range)
	}

	unsafe fn slice_entities_and_components(
		self,
		range: Range<usize>,
	) -> (&'a [Entity], &'a [Self::Component]) {
		(
			self.storage.entities().get_unchecked(range.clone()),
			self.storage.components().get_unchecked(range),
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
		let (components, ticks) = self.storage.get_with_ticks_mut(entity)?;
		Some(ComponentRefMut::new(components, ticks, self.world_tick))
	}

	fn get_ticks(&self, entity: Entity) -> Option<&ChangeTicks> {
		self.storage.get_ticks(entity)
	}

	fn contains(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	fn group_info(&self) -> GroupInfo<'a> {
		self.group_info
	}

	fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	fn change_tick(&self) -> Ticks {
		self.change_tick
	}

	fn into_parts(self) -> SplitComponentView<'a, Self::Component> {
		let (sparse, entities, components, ticks) = self.storage.split();
		(
			sparse,
			entities,
			components.as_ptr() as _,
			ticks.as_ptr() as _,
		)
	}

	unsafe fn get_from_parts(
		components: *mut Self::Component,
		info: *mut ChangeTicks,
		index: usize,
		world_tick: Ticks,
		_last_system_tick: Ticks,
	) -> Option<Self::Item> {
		Some(ComponentRefMut::new(
			&mut *components.add(index),
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
