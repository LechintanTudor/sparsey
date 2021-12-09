use crate::query::{Added, Changed, Mutated, Not, Passthrough};
use crate::storage::{ChangeTicks, Ticks};

/// Trait that enables filtering a component's `ChangeTicks`. Used internally by queries.
pub trait ChangeTicksFilter: 'static {
    /// Whether or not the filer matches all inputs. Used internally by queries for optimization
    /// purposes.
    const IS_PASSTHROUGH: bool = false;

    /// Returns `true` if the `ticks` match the filter considering the provided `world_tick` and
    /// `change_tick`.
    fn matches(ticks: &ChangeTicks, world_tick: Ticks, change_tick: Ticks) -> bool;
}

impl ChangeTicksFilter for Passthrough {
    const IS_PASSTHROUGH: bool = true;

    #[inline(always)]
    fn matches(_: &ChangeTicks, _: Ticks, _: Ticks) -> bool {
        true
    }
}

impl ChangeTicksFilter for Added {
    #[inline(always)]
    fn matches(ticks: &ChangeTicks, world_tick: Ticks, _change_tick: Ticks) -> bool {
        ticks.tick_added == world_tick
    }
}

impl ChangeTicksFilter for Mutated {
    #[inline(always)]
    fn matches(ticks: &ChangeTicks, _world_tick: Ticks, change_tick: Ticks) -> bool {
        ticks.tick_mutated > change_tick
    }
}

impl ChangeTicksFilter for Changed {
    #[inline(always)]
    fn matches(ticks: &ChangeTicks, world_tick: Ticks, change_tick: Ticks) -> bool {
        ticks.tick_added == world_tick || ticks.tick_mutated > change_tick
    }
}

impl<F> ChangeTicksFilter for Not<F>
where
    F: ChangeTicksFilter,
{
    #[inline(always)]
    fn matches(ticks: &ChangeTicks, world_tick: Ticks, change_tick: Ticks) -> bool {
        !F::matches(ticks, world_tick, change_tick)
    }
}
