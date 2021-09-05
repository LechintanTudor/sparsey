use crate::storage::Entity;
use crate::utils::Ticks;
use std::slice::SliceIndex;

/// Data used internally by query iterators.
#[derive(Clone, Copy, Debug)]
pub struct IterData<'a> {
	/// The entities over which to iterate.
	pub entities: &'a [Entity],
	/// The world tick at the time of creating the iterator.
	pub world_tick: Ticks,
	/// The change tick at the time of creating the iterator.
	pub change_tick: Ticks,
}

impl<'a> IterData<'a> {
	/// Empty `IterData`. Used for creating iterators over empty queries.
	pub const EMPTY: Self = Self::new(&[], 0, 0);

	/// Creates a new `IterData`.
	pub const fn new(entities: &'a [Entity], world_tick: Ticks, change_tick: Ticks) -> Self {
		Self {
			entities,
			world_tick,
			change_tick,
		}
	}

	/// Creates a new `IterData` by slicing the previous `Entity` slice.
	pub fn with_range<I>(self, range: I) -> Self
	where
		I: SliceIndex<[Entity], Output = [Entity]>,
	{
		Self::new(&self.entities[range], self.world_tick, self.change_tick)
	}
}
