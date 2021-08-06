use crate::utils::Ticks;

pub trait FetchFrom<'a, S> {
	type Item;

	fn fetch(source: &'a S, change_tick: Ticks) -> Self::Item;
}
