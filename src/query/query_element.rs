use crate::components::Component;
use crate::group::GroupInfo;
use crate::query::{Contains, QueryElementFilter};
use crate::storage::{Entity, SparseArrayView};
use crate::utils::{ChangeTicks, Ticks};
use std::marker::PhantomData;
use std::ops::Range;

/// Trait implement by types which can be part of a query.
pub unsafe trait QueryElement<'a> {
	type Item: 'a;
	type Component: Component;
	type Filter: QueryElementFilter<Self::Component>;

	fn get(self, entity: Entity) -> Option<Self::Item>;

	fn get_with_ticks(&self, entity: Entity) -> Option<(&Self::Component, &ChangeTicks)>;

	fn contains(&self, entity: Entity) -> bool;

	fn group_info(&self) -> Option<GroupInfo<'a>>;

	fn world_tick(&self) -> Ticks;

	fn change_tick(&self) -> Ticks;

	fn split(self) -> SplitQueryElement<'a, Self::Component, Self::Filter>;

	unsafe fn get_from_parts(
		component: *mut Self::Component,
		ticks: *mut ChangeTicks,
		world_tick: Ticks,
		change_tick: Ticks,
	) -> Self::Item;
}

/// Trait implemented by immutable `QueryElement`s.
pub unsafe trait ImmutableQueryElement<'a>
where
	Self: QueryElement<'a>,
{
	// Empty
}

/// Trait implemented by unfiltered `QueryElement`s.
pub trait UnfilteredQueryElement<'a>
where
	Self: QueryElement<'a, Filter = Contains>,
{
	// Empty
}

impl<'a, E> UnfilteredQueryElement<'a> for E
where
	E: QueryElement<'a, Filter = Contains>,
{
	// Empty
}

/// Trait implemented by unfiltered immutable `QueryElement`s.
pub trait UnfilteredImmutableQueryElement<'a>
where
	Self: ImmutableQueryElement<'a> + UnfilteredQueryElement<'a>,
{
	// Empty
}

impl<'a, E> UnfilteredImmutableQueryElement<'a> for E
where
	E: ImmutableQueryElement<'a> + UnfilteredQueryElement<'a>,
{
	// Empty
}

/// Trait used for getting slices of entities and components from a
/// `QueryElement`.
pub unsafe trait SliceQueryElement<'a>
where
	Self: UnfilteredImmutableQueryElement<'a>,
{
	unsafe fn slice_components(self, range: Range<usize>) -> &'a [Self::Component];

	unsafe fn slice_entities(self, range: Range<usize>) -> &'a [Entity];

	unsafe fn slice_entities_components(
		self,
		range: Range<usize>,
	) -> (&'a [Entity], &'a [Self::Component]);
}

/// Type returned by `QueryElement::split`.
#[non_exhaustive]
pub struct SplitQueryElement<'a, T, F> {
	pub sparse: SparseArrayView<'a>,
	pub entities: &'a [Entity],
	pub components: *mut T,
	pub ticks: *mut ChangeTicks,
	pub filter: F,
}

impl<'a, T, F> SplitQueryElement<'a, T, F> {
	/// Creates a new `SplitQueryElement`.
	pub fn new(
		sparse: SparseArrayView<'a>,
		entities: &'a [Entity],
		components: *mut T,
		ticks: *mut ChangeTicks,
		filter: F,
	) -> Self {
		Self {
			sparse,
			entities,
			components,
			ticks,
			filter,
		}
	}

	pub fn into_modifier_split(self) -> (&'a [Entity], SparseArrayView<'a>) {
		(self.entities, self.sparse)
	}

	pub fn into_sparse_split(self) -> (&'a [Entity], SparseSplitQueryElement<'a, T, F>) {
		(
			self.entities,
			SparseSplitQueryElement {
				sparse: self.sparse,
				components: self.components,
				ticks: self.ticks,
				filter: self.filter,
			},
		)
	}

	pub fn into_dense_split(self) -> (&'a [Entity], DenseSplitQueryElement<'a, T, F>) {
		(
			self.entities,
			DenseSplitQueryElement {
				components: self.components,
				ticks: self.ticks,
				filter: self.filter,
				lifetime: PhantomData,
			},
		)
	}
}

/// Used to form `QueryBase::SparseSplit`.
#[non_exhaustive]
pub struct SparseSplitQueryElement<'a, T, F> {
	pub sparse: SparseArrayView<'a>,
	pub components: *mut T,
	pub ticks: *mut ChangeTicks,
	pub filter: F,
}

impl<'a, T, F> SparseSplitQueryElement<'a, T, F> {
	pub unsafe fn get<V>(
		&mut self,
		entity: Entity,
		world_tick: Ticks,
		change_tick: Ticks,
	) -> Option<V::Item>
	where
		T: Component,
		V: QueryElement<'a, Component = T>,
		F: QueryElementFilter<T>,
	{
		let index = self.sparse.get_index(entity)?;
		let component = self.components.add(index);
		let ticks = self.ticks.add(index);

		self.filter
			.matches(&*component, &*ticks, world_tick, change_tick)
			.then(|| V::get_from_parts(component, ticks, world_tick, change_tick))
	}
}

/// Used to form `QueryBase::DenseSplit`.
pub struct DenseSplitQueryElement<'a, T, F> {
	pub components: *mut T,
	pub ticks: *mut ChangeTicks,
	pub filter: F,
	lifetime: PhantomData<&'a ()>,
}

impl<'a, T, F> DenseSplitQueryElement<'a, T, F> {
	pub unsafe fn get<V>(
		&mut self,
		index: usize,
		world_tick: Ticks,
		change_tick: Ticks,
	) -> Option<V::Item>
	where
		T: Component,
		V: QueryElement<'a, Component = T>,
		F: QueryElementFilter<T>,
	{
		let component = self.components.add(index);
		let ticks = self.ticks.add(index);

		self.filter
			.matches(&*component, &*ticks, world_tick, change_tick)
			.then(|| V::get_from_parts(component, ticks, world_tick, change_tick))
	}
}
