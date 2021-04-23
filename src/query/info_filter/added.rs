use crate::components::{ComponentInfo, Entity, Ticks};
use crate::query::{QueryElement, SplitQueryElement, UnfilteredQueryElement};
use crate::world::GroupInfo;
use std::ops::Not;

pub struct Added<Q> {
	query_elem: Q,
}

pub fn added<Q>(query_elem: Q) -> Added<Q> {
	Added { query_elem }
}

unsafe impl<'a, Q> QueryElement<'a> for Added<Q>
where
	Q: UnfilteredQueryElement<'a>,
{
	type Item = Q::Item;
	type Component = Q::Component;
	type SplitState = Ticks;

	fn contains(self, entity: Entity) -> bool {
		let world_tick = self.query_elem.world_tick();
		let (_, sparse, _, info, _) = self.query_elem.split();

		match sparse.get_index(entity) {
			Some(index) => unsafe { (*info.add(index as usize)).tick_added() == world_tick },
			None => false,
		}
	}

	fn split(self) -> SplitQueryElement<'a, Self::SplitState, Self::Component> {
		let world_tick = self.query_elem.world_tick();
		let (_, sparse, entities, info, data) = self.query_elem.split();
		(world_tick, sparse, entities, info, data)
	}

	fn group_info(&self) -> Option<&GroupInfo> {
		self.query_elem.group_info()
	}

	unsafe fn get_from_split(
		world_tick: Self::SplitState,
		info: *mut ComponentInfo,
		data: *mut Self::Component,
		index: usize,
	) -> Option<Self::Item> {
		if (*info.add(index)).tick_added() == world_tick {
			Q::get_from_split((), info, data, index)
		} else {
			None
		}
	}
}

impl<Q> Not for Added<Q> {
	type Output = NotAdded<Q>;

	fn not(self) -> Self::Output {
		NotAdded {
			query_elem: self.query_elem,
		}
	}
}

pub struct NotAdded<Q> {
	query_elem: Q,
}

unsafe impl<'a, Q> QueryElement<'a> for NotAdded<Q>
where
	Q: UnfilteredQueryElement<'a>,
{
	type Item = Q::Item;
	type Component = Q::Component;
	type SplitState = Ticks;

	fn contains(self, entity: Entity) -> bool {
		let world_tick = self.query_elem.world_tick();
		let (_, sparse, _, info, _) = self.query_elem.split();

		match sparse.get_index(entity) {
			Some(index) => unsafe { (*info.add(index as usize)).tick_added() != world_tick },
			None => false,
		}
	}

	fn split(self) -> SplitQueryElement<'a, Self::SplitState, Self::Component> {
		let world_tick = self.query_elem.world_tick();
		let (_, sparse, entities, info, data) = self.query_elem.split();
		(world_tick, sparse, entities, info, data)
	}

	fn group_info(&self) -> Option<&GroupInfo> {
		self.query_elem.group_info()
	}

	unsafe fn get_from_split(
		world_tick: Self::SplitState,
		info: *mut ComponentInfo,
		data: *mut Self::Component,
		index: usize,
	) -> Option<Self::Item> {
		if (*info.add(index)).tick_added() != world_tick {
			Q::get_from_split((), info, data, index)
		} else {
			None
		}
	}
}
