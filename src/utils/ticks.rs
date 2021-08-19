use std::num::NonZeroU32;

pub type Ticks = u32;
pub type NonZeroTicks = NonZeroU32;

/// Holds the ticks in which a component was added and last mutated.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub struct ChangeTicks {
	pub tick_added: Ticks,
	pub tick_mutated: Ticks,
}

impl ChangeTicks {
	/// Creates a new `ChangeTicks` object with the given ticks.
	pub const fn new(tick_added: Ticks, tick_mutated: Ticks) -> Self {
		Self {
			tick_added,
			tick_mutated,
		}
	}

	/// Creates a new `ChangeTicks` object for components which were just
	/// added.
	pub const fn just_added(tick_added: Ticks) -> Self {
		Self {
			tick_added,
			tick_mutated: 0,
		}
	}
}
