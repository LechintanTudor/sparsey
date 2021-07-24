use crate::components::{ComponentTicks, Entity, Ticks};
use crate::query::{
	ComponentTicksFilter, ComponentView, ImmutableUnfilteredComponentView, SplitComponentView,
	UnfilteredComponentView,
};
use crate::world::GroupInfo;
use std::marker::PhantomData;

pub type PassthroughFilter = Filter<(), Passthrough>;

pub trait QueryFilter {
	fn matches(&self, entity: Entity) -> bool;
}

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

#[derive(Clone, Copy, Default, Debug)]
pub struct Passthrough;

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
			self.component_view.last_system_tick(),
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
			self.component_view.last_system_tick(),
		);

		if matches {
			self.component_view.get(entity)
		} else {
			None
		}
	}

	fn get_ticks(&self, entity: Entity) -> Option<&ComponentTicks> {
		self.component_view.get_ticks(entity)
	}

	fn contains(&self, entity: Entity) -> bool {
		F::matches(
			self.component_view.get_ticks(entity),
			self.component_view.world_tick(),
			self.component_view.last_system_tick(),
		)
	}

	fn group_info(&self) -> GroupInfo<'a> {
		self.component_view.group_info()
	}

	fn world_tick(&self) -> Ticks {
		self.component_view.world_tick()
	}

	fn last_system_tick(&self) -> Ticks {
		self.component_view.last_system_tick()
	}

	fn into_parts(self) -> SplitComponentView<'a, Self::Component> {
		self.component_view.into_parts()
	}

	unsafe fn get_from_parts(
		data: *mut Self::Component,
		ticks: *mut ComponentTicks,
		index: usize,
		world_tick: Ticks,
		last_system_tick: Ticks,
	) -> Option<Self::Item> {
		if F::matches(Some(&*ticks.add(index)), world_tick, last_system_tick) {
			C::get_from_parts(data, ticks, index, world_tick, last_system_tick)
		} else {
			None
		}
	}
}
