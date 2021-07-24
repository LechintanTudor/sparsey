use crate::components::{ComponentTicks, Ticks};
use crate::query::{Filter, UnfilteredComponentView};

pub trait ComponentTicksFilter {
	fn matches(ticks: Option<&ComponentTicks>, world_tick: Ticks, last_system_tick: Ticks) -> bool;
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Added;

pub fn added<'a, C>(component_view: C) -> Filter<C, Added>
where
	C: UnfilteredComponentView<'a>,
{
	Filter::new(component_view)
}

impl ComponentTicksFilter for Added {
	fn matches(
		ticks: Option<&ComponentTicks>,
		world_tick: Ticks,
		_last_system_tick: Ticks,
	) -> bool {
		ticks
			.filter(|ticks| ticks.tick_added() == world_tick)
			.is_some()
	}
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Mutated;

pub fn mutated<'a, C>(component_view: C) -> Filter<C, Mutated>
where
	C: UnfilteredComponentView<'a>,
{
	Filter::new(component_view)
}

impl ComponentTicksFilter for Mutated {
	fn matches(
		ticks: Option<&ComponentTicks>,
		_world_tick: Ticks,
		last_system_tick: Ticks,
	) -> bool {
		ticks
			.filter(|ticks| ticks.tick_mutated() > last_system_tick)
			.is_some()
	}
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Updated;

pub fn updated<'a, C>(component_view: C) -> Filter<C, Updated>
where
	C: UnfilteredComponentView<'a>,
{
	Filter::new(component_view)
}

impl ComponentTicksFilter for Updated {
	fn matches(ticks: Option<&ComponentTicks>, world_tick: Ticks, last_system_tick: Ticks) -> bool {
		ticks
			.filter(|ticks| {
				ticks.tick_mutated() > last_system_tick || ticks.tick_added() == world_tick
			})
			.is_some()
	}
}
