use crate::components::Component;
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

#[derive(Clone, Copy)]
pub struct Passthrough;

impl<T> QueryElementFilter<T> for Passthrough
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

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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
