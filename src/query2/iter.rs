use crate::storage::Entity;
use crate::utils::Ticks;

#[derive(Clone, Copy, Debug)]
pub struct IterData<'a> {
	pub entities: &'a [Entity],
	pub world_tick: Ticks,
	pub change_tick: Ticks,
}

impl<'a> IterData<'a> {
	pub fn new(entities: &'a [Entity], world_tick: Ticks, change_tick: Ticks) -> Self {
		Self {
			entities,
			world_tick,
			change_tick,
		}
	}
}
