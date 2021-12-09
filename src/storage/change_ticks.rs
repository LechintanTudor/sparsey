use std::num::NonZeroU32;

/// Used in change detection to track the tick in which a change occurred.
pub type Ticks = u32;
/// Same as `Ticks`, but non zero.
pub type NonZeroTicks = NonZeroU32;

/// Holds the `Ticks` in which a component was added and last mutated.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub struct ChangeTicks {
    /// The tick when the component was added.
    pub tick_added: Ticks,
    /// The tick when the component was last mutated.
    pub tick_mutated: Ticks,
}

impl ChangeTicks {
    /// Creates a new `ChangeTicks` object with the given ticks.
    pub const fn new(tick_added: Ticks, tick_mutated: Ticks) -> Self {
        Self { tick_added, tick_mutated }
    }

    /// Creates a new `ChangeTicks` object for components which were just
    /// added.
    pub const fn just_added(tick_added: Ticks) -> Self {
        Self { tick_added, tick_mutated: 0 }
    }
}
