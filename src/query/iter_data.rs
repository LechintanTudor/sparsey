use crate::storage::Entity;
use crate::utils::Ticks;
use std::slice::SliceIndex;

#[derive(Clone, Copy, Debug)]
pub struct IterData<'a> {
	pub entities: &'a [Entity],
	pub world_tick: Ticks,
	pub change_tick: Ticks,
}

impl<'a> IterData<'a> {
	pub const EMPTY: Self = Self::new(&[], 0, 0);

	pub const fn new(entities: &'a [Entity], world_tick: Ticks, change_tick: Ticks) -> Self {
		Self {
			entities,
			world_tick,
			change_tick,
		}
	}

	pub fn with_range<I>(self, range: I) -> Self
	where
		I: SliceIndex<[Entity], Output = [Entity]>,
	{
		Self::new(&self.entities[range], self.world_tick, self.change_tick)
	}
}
