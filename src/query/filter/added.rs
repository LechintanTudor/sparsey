use crate::components::{ComponentTicks, Ticks};
use crate::query::{ComponentInfoFilter, FilteredComponentView, UnfilteredComponentView};
use std::ops::Not;

pub fn added<'a, C>(view: C) -> FilteredComponentView<C, Added>
where
	C: UnfilteredComponentView<'a>,
{
	FilteredComponentView::new(view)
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Added;

impl ComponentInfoFilter for Added {
	fn matches(info: Option<&ComponentTicks>, world_tick: Ticks, _last_system_tick: Ticks) -> bool {
		info.filter(|info| info.tick_added() == world_tick)
			.is_some()
	}
}

#[derive(Clone, Copy, Default, Debug)]
pub struct NotAdded;

impl ComponentInfoFilter for NotAdded {
	fn matches(info: Option<&ComponentTicks>, world_tick: Ticks, last_system_tick: Ticks) -> bool {
		!Added::matches(info, world_tick, last_system_tick)
	}
}

impl<E> Not for FilteredComponentView<E, Added> {
	type Output = FilteredComponentView<E, NotAdded>;

	fn not(self) -> Self::Output {
		FilteredComponentView::new(self.into_view())
	}
}
