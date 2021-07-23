use crate::components::{ComponentTicks, Entity, Ticks};
use crate::query::{
	ComponentTicksFilter, Filter, ImmutableUnfilteredComponentView, QueryFilter,
	UnfilteredComponentView,
};

pub type Passthrough = Filter<(), ()>;

impl QueryFilter for Passthrough {
	fn matches(&self, _: Entity) -> bool {
		true
	}
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Added;

pub fn added<'a, C>(view: C) -> Filter<C, Added>
where
	C: UnfilteredComponentView<'a>,
{
	Filter::new(view)
}

impl ComponentTicksFilter for Added {
	fn matches(info: Option<&ComponentTicks>, world_tick: Ticks, _last_system_tick: Ticks) -> bool {
		info.filter(|info| info.tick_added() == world_tick)
			.is_some()
	}
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Mutated;

pub fn mutated<'a, C>(view: C) -> Filter<C, Mutated>
where
	C: UnfilteredComponentView<'a>,
{
	Filter::new(view)
}

impl ComponentTicksFilter for Mutated {
	fn matches(info: Option<&ComponentTicks>, _world_tick: Ticks, last_system_tick: Ticks) -> bool {
		info.filter(|info| info.tick_mutated() > last_system_tick)
			.is_some()
	}
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Updated;

pub fn updated<'a, C>(view: C) -> Filter<C, Updated>
where
	C: UnfilteredComponentView<'a>,
{
	Filter::new(view)
}

impl ComponentTicksFilter for Updated {
	fn matches(info: Option<&ComponentTicks>, world_tick: Ticks, last_system_tick: Ticks) -> bool {
		info.filter(|info| {
			info.tick_mutated() > last_system_tick || info.tick_added() == world_tick
		})
		.is_some()
	}
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Contains;

pub fn contains<'a, C>(view: C) -> Filter<C, Contains>
where
	C: ImmutableUnfilteredComponentView<'a>,
{
	Filter::new(view)
}

impl<'a, C> QueryFilter for Filter<C, Contains>
where
	C: ImmutableUnfilteredComponentView<'a>,
{
	fn matches(&self, entity: Entity) -> bool {
		self.view().contains(entity)
	}
}
