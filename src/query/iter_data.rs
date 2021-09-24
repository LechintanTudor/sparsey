use crate::storage::Entity;
use crate::utils::Ticks;

/// Data used internally by query iterators.
#[derive(Clone, Copy, Debug)]
pub struct IterData<'a> {
    /// The entities over which to iterate.
    pub entities: &'a [Entity],
    /// The world tick at the time of creating the iterator.
    pub world_tick: Ticks,
    /// The change tick at the time of creating the iterator.
    pub change_tick: Ticks,
}

impl<'a> IterData<'a> {
    /// Creates a new `IterData`.
    pub const fn new(entities: &'a [Entity], world_tick: Ticks, change_tick: Ticks) -> Self {
        Self {
            entities,
            world_tick,
            change_tick,
        }
    }
}
