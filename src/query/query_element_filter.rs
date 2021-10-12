use crate::components::Component;
use crate::query::{Filter, Not, UnfilteredQueryElement};
use crate::utils::{ChangeTicks, Ticks};

/// Trait used for filtering `QueryElement`s.
pub trait QueryElementFilter<T>
where
    T: Component,
{
    /// Returns `true` if the component and ticks match the filter.
    fn matches(
        &self,
        component: &T,
        ticks: &ChangeTicks,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> bool;
}

/// `QueryElementFilter` that matches all components.
#[derive(Clone, Copy, Default)]
pub struct Contains;

impl<T> QueryElementFilter<T> for Contains
where
    T: Component,
{
    #[inline]
    fn matches(
        &self,
        _component: &T,
        _ticks: &ChangeTicks,
        _world_tick: Ticks,
        _change_tick: Ticks,
    ) -> bool {
        true
    }
}

/// `QueryElementFilter` that ony matches newly added components.
#[derive(Clone, Copy, Default)]
pub struct Added;

impl<T> QueryElementFilter<T> for Added
where
    T: Component,
{
    #[inline]
    fn matches(
        &self,
        _component: &T,
        ticks: &ChangeTicks,
        world_tick: Ticks,
        _change_tick: Ticks,
    ) -> bool {
        ticks.tick_added == world_tick
    }
}

/// `QueryElementFilter` that only matches mutated components.
#[derive(Clone, Copy, Default)]
pub struct Mutated;

impl<T> QueryElementFilter<T> for Mutated
where
    T: Component,
{
    #[inline]
    fn matches(
        &self,
        _component: &T,
        ticks: &ChangeTicks,
        _world_tick: Ticks,
        change_tick: Ticks,
    ) -> bool {
        ticks.tick_mutated > change_tick
    }
}

/// `QueryElementFilter` that only matches newly added or mutated components.
#[derive(Clone, Copy, Default)]
pub struct Changed;

impl<T> QueryElementFilter<T> for Changed
where
    T: Component,
{
    #[inline]
    fn matches(
        &self,
        _component: &T,
        ticks: &ChangeTicks,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> bool {
        ticks.tick_added == world_tick || ticks.tick_mutated > change_tick
    }
}

impl<T, F> QueryElementFilter<T> for Not<F>
where
    T: Component,
    F: QueryElementFilter<T>,
{
    #[inline]
    fn matches(
        &self,
        component: &T,
        ticks: &ChangeTicks,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> bool {
        !self.0.matches(component, ticks, world_tick, change_tick)
    }
}

/// Creates a new `Filter` that matches all components.
pub fn contains<'a, E>(element: E) -> Filter<Contains, E>
where
    E: UnfilteredQueryElement<'a>,
{
    Filter::new(Contains, element)
}

/// Creates a new `Filter` that ony matches newly added components.
pub fn added<'a, E>(element: E) -> Filter<Added, E>
where
    E: UnfilteredQueryElement<'a>,
{
    Filter::new(Added, element)
}

/// Creates a new `Filter` that ony matches mutated components.
pub fn mutated<'a, E>(element: E) -> Filter<Mutated, E>
where
    E: UnfilteredQueryElement<'a>,
{
    Filter::new(Mutated, element)
}

/// Creates a new `Filter` that ony matches newly added or mutated components.
pub fn changed<'a, E>(element: E) -> Filter<Changed, E>
where
    E: UnfilteredQueryElement<'a>,
{
    Filter::new(Changed, element)
}
