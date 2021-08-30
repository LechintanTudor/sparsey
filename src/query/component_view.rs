use crate::components::Component;
use crate::group::GroupInfo;
use crate::storage::{Entity, SparseArrayView};
use crate::utils::{ChangeTicks, Ticks};
use std::ops::Range;
use std::marker::PhantomData;

pub struct SplitComponentView<'a, T> {
	pub sparse: SparseArrayView<'a>,
	pub entities: &'a [Entity],
	pub components: *mut T,
	pub ticks: *mut ChangeTicks,
}

#[derive(Clone, Copy)]
pub struct SparseSplitComponentView<'a, T> {
	pub sparse: SparseArrayView<'a>,
	pub components: *mut T,
	pub ticks: *mut ChangeTicks,
}

impl<'a, T> SparseSplitComponentView<'a, T> {
    pub fn new(sparse: SparseArrayView<'a>, components: *mut T, ticks: *mut ChangeTicks) -> Self {
		Self {
			sparse,
			components,
			ticks,
			lifetime: PhantomData,
		}
	}
}

#[derive(Clone, Copy)]
pub struct DenseSplitComponentView<'a, T> {
	pub components: *mut T,
	pub ticks: *mut ChangeTicks,
	lifetime: PhantomData<&'a ()>,
}

impl<'a, T> DenseSplitComponentView<'a, T> {
    pub fn new(components: *mut T, ticks: *mut ChangeTicks) -> Self {
		Self {
			components,
			ticks,
			lifetime: PhantomData,
		}
	}
}

/// Trait implemented by views over component storages.
pub unsafe trait ComponentView<'a>
where
	Self: Sized,
{
	type Item;
	type Component: Component;

	fn get(self, entity: Entity) -> Option<Self::Item>;

	fn contains(&self, entity: Entity) -> bool;

	fn group_info(&self) -> GroupInfo<'a>;

	fn world_tick(&self) -> Ticks;

	fn change_tick(&self) -> Ticks;

	fn into_parts(self) -> SplitComponentView<'a, T>;

	unsafe fn get_from_parts(
		components: *mut Self::Component,
		ticks: *mut ChangeTicks,
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
