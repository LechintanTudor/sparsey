use crate::components::{ComponentInfo, Ticks};
use crate::query::{ComponentInfoFilter, FilteredComponentView, UnfilteredComponentView};
use std::ops::Not;

pub fn updated<'a, C>(view: C) -> FilteredComponentView<C, Updated>
where
	C: UnfilteredComponentView<'a>,
{
	FilteredComponentView::new(view)
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Updated;

impl ComponentInfoFilter for Updated {
	fn matches(info: Option<&ComponentInfo>, world_tick: Ticks, last_system_tick: Ticks) -> bool {
		info.filter(|info| {
			info.tick_mutated() > last_system_tick || info.tick_added() == world_tick
		})
		.is_some()
	}
}

#[derive(Clone, Copy, Default, Debug)]
pub struct NotUpdated;

impl ComponentInfoFilter for NotUpdated {
	fn matches(info: Option<&ComponentInfo>, world_tick: Ticks, last_system_tick: Ticks) -> bool {
		!Updated::matches(info, world_tick, last_system_tick)
	}
}

impl<E> Not for FilteredComponentView<E, Updated> {
	type Output = FilteredComponentView<E, NotUpdated>;

	fn not(self) -> Self::Output {
		FilteredComponentView::new(self.into_view())
	}
}
