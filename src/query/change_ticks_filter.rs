use crate::query::{Not, Passthrough};
use crate::utils::{ChangeTicks, Ticks};

pub trait ChangeTicksFilter
where
    Self: 'static,
{
    const IS_PASSTHROUGH: bool = false;

    fn matches(ticks: &ChangeTicks, world_tick: Ticks, change_tick: Ticks) -> bool;
}

impl ChangeTicksFilter for Passthrough {
    const IS_PASSTHROUGH: bool = true;

    #[inline(always)]
    fn matches(_: &ChangeTicks, _: Ticks, _: Ticks) -> bool {
        true
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Added;

impl ChangeTicksFilter for Added {
    #[inline(always)]
    fn matches(ticks: &ChangeTicks, world_tick: Ticks, _change_tick: Ticks) -> bool {
        ticks.tick_added == world_tick
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Mutated;

impl ChangeTicksFilter for Mutated {
    #[inline(always)]
    fn matches(ticks: &ChangeTicks, _world_tick: Ticks, change_tick: Ticks) -> bool {
        ticks.tick_mutated > change_tick
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Changed;

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
