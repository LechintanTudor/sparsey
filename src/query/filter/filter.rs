use crate::group::GroupInfo;
use crate::query::{
	ComponentTicksFilter, ComponentView, ImmutableUnfilteredComponentView, SplitComponentView,
	UnfilteredComponentView,
};
use crate::storage::Entity;
use crate::utils::{ChangeTicks, Ticks};
use std::marker::PhantomData;

/// Query filter that matches all inputs.
pub type PassthroughFilter = Filter<(), Passthrough>;

/// Trait implemented by types which can be used as query filters.
pub trait QueryFilter {
	fn matches(&self, entity: Entity) -> bool;
}

/// Encapsulates a filtered component view.
#[derive(Default)]
pub struct Filter<C, F> {
	pub(crate) component_view: C,
	pub(crate) filter: PhantomData<F>,
}

impl<C, F> Filter<C, F> {
	pub(crate) fn new(component_view: C) -> Self {
		Self {
			component_view,
			filter: PhantomData,
		}
	}
}

/// Marker type used to create a passthrough query filter.
#[derive(Clone, Copy, Default, Debug)]
pub struct Passthrough;

/// Creates a passthrough query filter.
pub const fn passthrough() -> Filter<(), Passthrough> {
	Filter {
		component_view: (),
		filter: PhantomData,
	}
}

impl QueryFilter for Filter<(), Passthrough> {
	fn matches(&self, _: Entity) -> bool {
		true
	}
}

impl<'a, C, F> QueryFilter for Filter<C, F>
where
	C: ImmutableUnfilteredComponentView<'a>,
	F: ComponentTicksFilter,
{
	fn matches(&self, entity: Entity) -> bool {
		F::matches(
			self.component_view.get_ticks(entity),
			self.component_view.world_tick(),
			self.component_view.change_tick(),
		)
	}
}

unsafe impl<'a, C, F> ComponentView<'a> for Filter<C, F>
where
	C: UnfilteredComponentView<'a>,
	F: ComponentTicksFilter,
{
	type Item = C::Item;
	type Component = C::Component;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		let matches = F::matches(
			self.component_view.get_ticks(entity),
			self.component_view.world_tick(),
			self.component_view.change_tick(),
		);

		if matches {
			self.component_view.get(entity)
		} else {
			None
		}
	}

	fn get_ticks(&self, entity: Entity) -> Option<&ChangeTicks> {
		self.component_view.get_ticks(entity)
	}

	fn contains(&self, entity: Entity) -> bool {
		F::matches(
			self.component_view.get_ticks(entity),
			self.component_view.world_tick(),
			self.component_view.change_tick(),
		)
	}

	fn group_info(&self) -> GroupInfo<'a> {
		self.component_view.group_info()
	}

	fn world_tick(&self) -> Ticks {
		self.component_view.world_tick()
	}

	fn change_tick(&self) -> Ticks {
		self.component_view.change_tick()
	}

	fn into_parts(self) -> SplitComponentView<'a, Self::Component> {
		self.component_view.into_parts()
	}

	unsafe fn get_from_parts(
		data: *mut Self::Component,
		ticks: *mut ChangeTicks,
		index: usize,
		world_tick: Ticks,
		change_tick: Ticks,
	) -> Option<Self::Item> {
		if F::matches(Some(&*ticks.add(index)), world_tick, change_tick) {
			C::get_from_parts(data, ticks, index, world_tick, change_tick)
		} else {
			None
		}
	}
}
