use crate::components::{ComponentInfo, Ticks};
use crate::query::{ComponentInfoFilter, FilteredComponentView, UnfilteredComponentView};
use std::ops::Not;

#[derive(Clone, Copy, Default, Debug)]
pub struct AddedFilter;

impl ComponentInfoFilter for AddedFilter {
	fn matches(info: Option<&ComponentInfo>, world_tick: Ticks, _last_system_tick: Ticks) -> bool {
		info.filter(|info| info.tick_added() == world_tick)
			.is_some()
	}
}

pub fn added<'a, C>(view: C) -> FilteredComponentView<C, AddedFilter>
where
	C: UnfilteredComponentView<'a>,
{
	FilteredComponentView::new(view)
}

#[derive(Clone, Copy, Default, Debug)]
pub struct NotAddedFilter;

impl ComponentInfoFilter for NotAddedFilter {
	fn matches(info: Option<&ComponentInfo>, world_tick: Ticks, last_system_tick: Ticks) -> bool {
		!AddedFilter::matches(info, world_tick, last_system_tick)
	}
}

impl<E> Not for FilteredComponentView<E, AddedFilter> {
	type Output = FilteredComponentView<E, NotAddedFilter>;

	fn not(self) -> Self::Output {
		FilteredComponentView::new(self.into_view())
	}
}
