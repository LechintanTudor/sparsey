pub type Ticks = u32;

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct ComponentTicks {
	tick_added: Ticks,
	tick_mutated: Ticks,
}

impl ComponentTicks {
	pub(crate) fn added(tick_added: Ticks) -> Self {
		Self {
			tick_added,
			tick_mutated: 0,
		}
	}

	pub(crate) fn set_tick_mutated(&mut self, tick_mutated: Ticks) {
		self.tick_mutated = tick_mutated;
	}

	pub fn tick_added(&self) -> Ticks {
		self.tick_added
	}

	pub fn tick_mutated(&self) -> Ticks {
		self.tick_mutated
	}
}
