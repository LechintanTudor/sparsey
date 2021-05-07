use crate::components::{ComponentInfo, Ticks};
use crate::query::{FilteredQueryElement, InfoFilter, UnfilteredQueryElement};
use std::ops::Not;

#[derive(Clone, Copy, Default, Debug)]
pub struct AddedFilter;

impl InfoFilter for AddedFilter {
	fn matches(info: &ComponentInfo, world_tick: Ticks, _last_system_tick: Ticks) -> bool {
		info.tick_added() == world_tick
	}
}

pub fn added<'a, E>(element: E) -> FilteredQueryElement<E, AddedFilter>
where
	E: UnfilteredQueryElement<'a>,
{
	FilteredQueryElement::new(element)
}

#[derive(Clone, Copy, Default, Debug)]
pub struct NotAddedFilter;

impl InfoFilter for NotAddedFilter {
	fn matches(info: &ComponentInfo, world_tick: Ticks, _last_system_tick: Ticks) -> bool {
		info.tick_added() != world_tick
	}
}

impl<E> Not for FilteredQueryElement<E, AddedFilter> {
	type Output = FilteredQueryElement<E, NotAddedFilter>;

	fn not(self) -> Self::Output {
		FilteredQueryElement::new(self.element)
	}
}
