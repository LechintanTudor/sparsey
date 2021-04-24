use crate::components::{ComponentInfo, Entity, Ticks};
use crate::query::{QueryElement, SplitQueryElement, StateFilter};
use crate::world::GroupInfo;
use std::ops::Not;

pub struct Added<Q> {
	query_elem: Q,
}

pub fn added<Q>(query_elem: Q) -> Added<Q> {
	Added { query_elem }
}

impl<Q> Not for Added<Q> {
	type Output = NotAdded<Q>;

	fn not(self) -> Self::Output {
		NotAdded {
			query_elem: self.query_elem,
		}
	}
}

unsafe impl<'a, Q> QueryElement<'a> for Added<Q>
where
	Q: QueryElement<'a>,
{
	type Item = Q::Item;
	type Component = Q::Component;
	type SplitState = Q::SplitState;

	fn get_info(&self, entity: Entity) -> Option<&ComponentInfo> {
		self.query_elem.get_info(entity)
	}

	fn contains(&self, entity: Entity) -> bool {
		self.query_elem
			.get_info(entity)
			.filter(|info| info.tick_added() == self.world_tick())
			.is_some()
	}

	fn split(self) -> SplitQueryElement<'a, Self::SplitState, Self::Component> {
		self.query_elem.split()
	}

	fn group_info(&self) -> Option<&GroupInfo> {
		self.query_elem.group_info()
	}

	fn world_tick(&self) -> Ticks {
		self.query_elem.world_tick()
	}

	fn last_system_tick(&self) -> Ticks {
		self.query_elem.last_system_tick()
	}

	unsafe fn get_from_split(
		data: *mut Self::Component,
		info: *mut ComponentInfo,
		index: usize,
		state: Self::SplitState,
		world_tick: Ticks,
		last_system_tick: Ticks,
	) -> Option<Self::Item> {
		if (*info.add(index)).tick_added() == world_tick {
			Q::get_from_split(data, info, index, state, world_tick, last_system_tick)
		} else {
			None
		}
	}
}

impl<'a, Q> StateFilter for Added<Q>
where
	Q: QueryElement<'a>,
{
	fn matches(&self, entity: Entity) -> bool {
		self.get_info(entity)
			.filter(|info| info.tick_added() == self.world_tick())
			.is_some()
	}
}

pub struct NotAdded<Q> {
	query_elem: Q,
}

unsafe impl<'a, Q> QueryElement<'a> for NotAdded<Q>
where
	Q: QueryElement<'a>,
{
	type Item = Q::Item;
	type Component = Q::Component;
	type SplitState = Q::SplitState;

	fn get_info(&self, entity: Entity) -> Option<&ComponentInfo> {
		self.query_elem.get_info(entity)
	}

	fn contains(&self, entity: Entity) -> bool {
		self.query_elem
			.get_info(entity)
			.filter(|info| info.tick_added() != self.world_tick())
			.is_some()
	}

	fn split(self) -> SplitQueryElement<'a, Self::SplitState, Self::Component> {
		self.query_elem.split()
	}

	fn group_info(&self) -> Option<&GroupInfo> {
		self.query_elem.group_info()
	}

	fn world_tick(&self) -> Ticks {
		self.query_elem.world_tick()
	}

	fn last_system_tick(&self) -> Ticks {
		self.query_elem.last_system_tick()
	}

	unsafe fn get_from_split(
		data: *mut Self::Component,
		info: *mut ComponentInfo,
		index: usize,
		state: Self::SplitState,
		world_tick: Ticks,
		last_system_tick: Ticks,
	) -> Option<Self::Item> {
		if (*info.add(index)).tick_added() != world_tick {
			Q::get_from_split(data, info, index, state, world_tick, last_system_tick)
		} else {
			None
		}
	}
}

impl<'a, Q> StateFilter for NotAdded<Q>
where
	Q: QueryElement<'a>,
{
	fn matches(&self, entity: Entity) -> bool {
		self.get_info(entity)
			.filter(|info| info.tick_added() != self.world_tick())
			.is_some()
	}
}
