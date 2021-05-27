use crate::components::{ComponentTicks, Ticks};
use crate::query::{ComponentInfoFilter, FilteredComponentView, UnfilteredComponentView};
use std::ops::Not;

pub fn mutated<'a, C>(view: C) -> FilteredComponentView<C, Mutated>
where
	C: UnfilteredComponentView<'a>,
{
	FilteredComponentView::new(view)
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Mutated;

impl ComponentInfoFilter for Mutated {
	fn matches(info: Option<&ComponentTicks>, _world_tick: Ticks, last_system_tick: Ticks) -> bool {
		info.filter(|info| info.tick_mutated() > last_system_tick)
			.is_some()
	}
}

#[derive(Clone, Copy, Default, Debug)]
pub struct NotMutated;

impl ComponentInfoFilter for NotMutated {
	fn matches(info: Option<&ComponentTicks>, world_tick: Ticks, last_system_tick: Ticks) -> bool {
		!Mutated::matches(info, world_tick, last_system_tick)
	}
}

impl<E> Not for FilteredComponentView<E, Mutated> {
	type Output = FilteredComponentView<E, NotMutated>;

	fn not(self) -> Self::Output {
		FilteredComponentView::new(self.into_view())
	}
}
