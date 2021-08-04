use crate::components::Ticks;

pub trait FetchFrom<'a, S> {
	type Item;

	fn fetch(source: &'a S, world_tick: Ticks, change_tick: Ticks) -> Self::Item;
}
