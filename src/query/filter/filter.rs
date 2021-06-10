use crate::components::{ComponentTicks, Entity, Ticks};
use crate::query::{ComponentView, SplitComponentView, UnfilteredComponentView};
use crate::world::GroupInfo;
use std::marker::PhantomData;

pub trait QueryFilter {
	fn matches(&self, entity: Entity) -> bool;
}

pub trait ComponentTicksFilter {
	fn matches(info: Option<&ComponentTicks>, world_tick: Ticks, last_system_tick: Ticks) -> bool;
}

#[derive(Default)]
pub struct Filter<C, F> {
	view: C,
	filter: PhantomData<F>,
}

impl<C, F> Filter<C, F> {
	pub(crate) fn new(view: C) -> Self {
		Self {
			view,
			filter: PhantomData,
		}
	}

	pub(crate) fn view(&self) -> &C {
		&self.view
	}
}

impl<'a, C, F> QueryFilter for Filter<C, F>
where
	C: UnfilteredComponentView<'a>,
	F: ComponentTicksFilter,
{
	fn matches(&self, entity: Entity) -> bool {
		F::matches(
			self.view.get_ticks(entity),
			self.view.world_tick(),
			self.view.last_system_tick(),
		)
	}
}

unsafe impl<'a, C, F> ComponentView<'a> for Filter<C, F>
where
	C: UnfilteredComponentView<'a>,
	F: ComponentTicksFilter,
{
	type Item = C::Item;
	type Component = C::Component;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		let matches = F::matches(
			self.view.get_ticks(entity),
			self.view.world_tick(),
			self.view.last_system_tick(),
		);

		if matches {
			self.view.get(entity)
		} else {
			None
		}
	}

	fn get_ticks(&self, entity: Entity) -> Option<&ComponentTicks> {
		self.view.get_ticks(entity)
	}

	fn contains(&self, entity: Entity) -> bool {
		F::matches(
			self.view.get_ticks(entity),
			self.view.world_tick(),
			self.view.last_system_tick(),
		)
	}

	fn group_info(&self) -> GroupInfo<'a> {
		self.view.group_info()
	}

	fn world_tick(&self) -> Ticks {
		self.view.world_tick()
	}

	fn last_system_tick(&self) -> Ticks {
		self.view.last_system_tick()
	}

	fn split(self) -> SplitComponentView<'a, Self::Component> {
		self.view.split()
	}

	unsafe fn get_from_parts(
		data: *mut Self::Component,
		info: *mut ComponentTicks,
		index: usize,
		world_tick: Ticks,
		last_system_tick: Ticks,
	) -> Option<Self::Item> {
		if F::matches(Some(&*info.add(index)), world_tick, last_system_tick) {
			C::get_from_parts(data, info, index, world_tick, last_system_tick)
		} else {
			None
		}
	}
}
