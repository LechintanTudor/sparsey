use std::num::NonZeroU32;

pub type Ticks = u32;
pub type NonZeroTicks = NonZeroU32;

/// Contains the ticks in which a component was added and last mutated.
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

	/// Returns the tick in which the component was added to the `World`.
	pub fn tick_added(&self) -> Ticks {
		self.tick_added
	}

	/// Returns the tick in which the component was last mutated.
	pub fn tick_mutated(&self) -> Ticks {
		self.tick_mutated
	}
}
