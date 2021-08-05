use crate::query::{Filter, UnfilteredComponentView};
use crate::utils::{ChangeTicks, Ticks};

/// Trait used for easily implementing filtered component views.
pub trait ComponentTicksFilter {
	fn matches(ticks: Option<&ChangeTicks>, world_tick: Ticks, last_system_tick: Ticks) -> bool;
}

#[doc(hidden)]
#[derive(Clone, Copy, Default, Debug)]
pub struct Added;

/// Filters the component view to only contain components which were just added.
pub fn added<'a, C>(component_view: C) -> Filter<C, Added>
where
	C: UnfilteredComponentView<'a>,
{
	Filter::new(component_view)
}

impl ComponentTicksFilter for Added {
	fn matches(ticks: Option<&ChangeTicks>, world_tick: Ticks, _last_system_tick: Ticks) -> bool {
		ticks
			.filter(|ticks| ticks.tick_added() == world_tick)
			.is_some()
	}
}

#[doc(hidden)]
#[derive(Clone, Copy, Default, Debug)]
pub struct Mutated;

/// Filters the component view to only contain components which were mutated.
pub fn mutated<'a, C>(component_view: C) -> Filter<C, Mutated>
where
	C: UnfilteredComponentView<'a>,
{
	Filter::new(component_view)
}

impl ComponentTicksFilter for Mutated {
	fn matches(ticks: Option<&ChangeTicks>, _world_tick: Ticks, last_system_tick: Ticks) -> bool {
		ticks
			.filter(|ticks| ticks.tick_mutated() > last_system_tick)
			.is_some()
	}
}

#[doc(hidden)]
#[derive(Clone, Copy, Default, Debug)]
pub struct Updated;

/// Filters the component view to only contain components which were just added
/// or mutated.
pub fn updated<'a, C>(component_view: C) -> Filter<C, Updated>
where
	C: UnfilteredComponentView<'a>,
{
	Filter::new(component_view)
}

impl ComponentTicksFilter for Updated {
	fn matches(ticks: Option<&ChangeTicks>, world_tick: Ticks, last_system_tick: Ticks) -> bool {
		ticks
			.filter(|ticks| {
				ticks.tick_mutated() > last_system_tick || ticks.tick_added() == world_tick
			})
			.is_some()
	}
}
