pub type Ticks = u32;

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct ComponentTicks {
	pub(in crate::components) tick_added: Ticks,
	pub(in crate::components) tick_mutated: Ticks,
}

impl ComponentTicks {
	pub fn new(tick_added: Ticks) -> Self {
		Self {
			tick_added,
			tick_mutated: 0,
		}
	}

	pub fn tick_added(&self) -> Ticks {
		self.tick_added
	}

	pub fn tick_mutated(&self) -> Ticks {
		self.tick_mutated
	}
}
