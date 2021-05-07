pub use self::added::*;

mod added;

use crate::components::{ComponentInfo, Entity, Ticks};
use crate::query::{QueryElement, SplitQueryElement, UnfilteredQueryElement};
use crate::world::GroupInfo;
use std::marker::PhantomData;

pub trait InfoFilter {
	fn matches(info: &ComponentInfo, world_tick: Ticks, last_system_tick: Ticks) -> bool;
}

pub struct FilteredQueryElement<E, F> {
	element: E,
	_phantom: PhantomData<F>,
}

impl<E, F> FilteredQueryElement<E, F> {
	fn new(element: E) -> Self {
		Self {
			element,
			_phantom: PhantomData,
		}
	}
}

unsafe impl<'a, E, F> QueryElement<'a> for FilteredQueryElement<E, F>
where
	E: UnfilteredQueryElement<'a>,
	F: InfoFilter,
{
	type Item = E::Item;
	type Component = E::Component;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		let matches = F::matches(
			self.element.get_info(entity)?,
			self.element.world_tick(),
			self.element.last_system_tick(),
		);

		if matches {
			self.element.get(entity)
		} else {
			None
		}
	}

	fn get_info(&self, entity: Entity) -> Option<&ComponentInfo> {
		self.element.get_info(entity)
	}

	fn contains(&self, entity: Entity) -> bool {
		let info = match self.element.get_info(entity) {
			Some(info) => info,
			None => return false,
		};

		F::matches(
			info,
			self.element.world_tick(),
			self.element.last_system_tick(),
		)
	}

	fn group_info(&self) -> Option<GroupInfo> {
		self.element.group_info()
	}

	fn world_tick(&self) -> Ticks {
		self.element.world_tick()
	}

	fn last_system_tick(&self) -> Ticks {
		self.element.last_system_tick()
	}

	fn split(self) -> SplitQueryElement<'a, Self::Component> {
		self.element.split()
	}

	unsafe fn get_from_parts(
		data: *mut Self::Component,
		info: *mut ComponentInfo,
		index: usize,
		world_tick: Ticks,
		last_system_tick: Ticks,
	) -> Option<Self::Item> {
		if F::matches(&*info.add(index), world_tick, last_system_tick) {
			E::get_from_parts(data, info, index, world_tick, last_system_tick)
		} else {
			None
		}
	}
}
