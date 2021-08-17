use crate::storage::Entity;
use crate::utils::Ticks;
use std::ops::Range;

#[derive(Clone, Copy, Debug)]
pub struct IterData<'a> {
	entities: &'a [Entity],
	world_tick: Ticks,
	last_system_tick: Ticks,
}

impl<'a> IterData<'a> {
	pub const EMPTY: Self = Self::new(&[], 0, 0);

	pub const fn new(entities: &'a [Entity], world_tick: Ticks, last_system_tick: Ticks) -> Self {
		Self {
			entities,
			world_tick,
			last_system_tick,
		}
	}

	pub fn with_range(self, range: Range<usize>) -> Self {
		Self {
			entities: &self.entities[range],
			world_tick: self.world_tick,
			last_system_tick: self.last_system_tick,
		}
	}

	pub const fn entities(&self) -> &'a [Entity] {
		self.entities
	}

	pub const fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	pub const fn last_system_tick(&self) -> Ticks {
		self.last_system_tick
	}
}
