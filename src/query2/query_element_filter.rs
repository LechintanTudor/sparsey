use crate::components::Component;
use crate::query2::{Filter, UnfilteredQueryElement};
use crate::utils::{ChangeTicks, Ticks};

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

#[derive(Clone, Copy, Default)]
pub struct Contains;

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

#[derive(Clone, Copy, Default)]
pub struct Added;

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

#[derive(Clone, Copy, Default)]
pub struct Mutated;

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

#[derive(Clone, Copy, Default)]
pub struct Changed;

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
