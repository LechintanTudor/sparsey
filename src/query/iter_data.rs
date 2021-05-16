use crate::components::{Entity, Ticks};

#[derive(Debug)]
pub struct IterData<'a> {
	entities: &'a [Entity],
	world_tick: Ticks,
	last_system_tick: Ticks,
}

impl<'a> IterData<'a> {
	pub fn new(entities: &'a [Entity], world_tick: Ticks, last_system_tick: Ticks) -> Self {
		Self {
			entities,
			world_tick,
			last_system_tick,
		}
	}

	pub fn entities(&self) -> &'a [Entity] {
		&self.entities
	}

	pub fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	pub fn last_system_tick(&self) -> Ticks {
		self.last_system_tick
	}
}
