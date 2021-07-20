use std::num::NonZeroU32;

pub type Ticks = u32;
pub type NonZeroTicks = NonZeroU32;

/// Holds the ticks in which a component was added and last mutated.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct ComponentTicks {
	tick_added: Ticks,
	tick_mutated: Ticks,
}

impl ComponentTicks {
	/// Creates a new `ComponentTicks` object with the given ticks.
	pub const fn new(tick_added: Ticks, tick_mutated: Ticks) -> Self {
		Self {
			tick_added,
			tick_mutated,
		}
	}

	/// Creates a new `ComponentTicks` object for components which were just
	/// added.
	pub const fn just_added(tick_added: Ticks) -> Self {
		Self {
			tick_added,
			tick_mutated: 0,
		}
	}

	pub(crate) fn set_tick_mutated(&mut self, tick_mutated: Ticks) {
		self.tick_mutated = tick_mutated;
	}

	/// Returns the tick in which the component was added to the `World`.
	pub const fn tick_added(&self) -> Ticks {
		self.tick_added
	}

	/// Returns the tick in which the component was last mutated.
	pub const fn tick_mutated(&self) -> Ticks {
		self.tick_mutated
	}
}
