use crate::components::Component;
use crate::query::{Filter, UnfilteredQueryElement};
use crate::utils::{ChangeTicks, Ticks};

/// Trait used for filtering `QueryElement`s.
pub trait QueryElementFilter<T>
where
    T: Component,
{
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

/// Creates a new `Filter` that matches all components.
pub fn contains<'a, E>(element: E) -> Filter<Contains, E>
where
    E: UnfilteredQueryElement<'a>,
{
    Filter::new(Contains, element)
}

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

/// Creates a new `Filter` that ony matches newly added components.
pub fn added<'a, E>(element: E) -> Filter<Added, E>
where
    E: UnfilteredQueryElement<'a>,
{
    Filter::new(Added, element)
}

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

/// Creates a new `Filter` that ony matches mutated components.
pub fn mutated<'a, E>(element: E) -> Filter<Mutated, E>
where
    E: UnfilteredQueryElement<'a>,
{
    Filter::new(Mutated, element)
}

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

/// Creates a new `Filter` that ony matches newly added or mutated components.
pub fn changed<'a, E>(element: E) -> Filter<Changed, E>
where
    E: UnfilteredQueryElement<'a>,
{
    Filter::new(Changed, element)
}

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
