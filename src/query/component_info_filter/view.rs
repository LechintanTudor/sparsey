use crate::components::{ComponentInfo, Entity, Ticks};
use crate::query::{
	ComponentInfoFilter, ComponentView, SplitComponentView, UnfilteredComponentView,
};
use crate::world::GroupInfo;
use std::marker::PhantomData;

pub struct FilteredComponentView<C, F> {
	view: C,
	_phantom: PhantomData<F>,
}

impl<C, F> FilteredComponentView<C, F> {
	pub(crate) fn new(view: C) -> Self {
		Self {
			view,
			_phantom: PhantomData,
		}
	}

	pub(crate) fn into_view(self) -> C {
		self.view
	}
}

unsafe impl<'a, C, F> ComponentView<'a> for FilteredComponentView<C, F>
where
	C: UnfilteredComponentView<'a>,
	F: ComponentInfoFilter,
{
	type Item = C::Item;
	type Component = C::Component;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		let matches = F::matches(
			self.view.get_info(entity)?,
			self.view.world_tick(),
			self.view.last_system_tick(),
		);

		if matches {
			self.view.get(entity)
		} else {
			None
		}
	}

	fn get_info(&self, entity: Entity) -> Option<&ComponentInfo> {
		self.view.get_info(entity)
	}

	fn contains(&self, entity: Entity) -> bool {
		let info = match self.view.get_info(entity) {
			Some(info) => info,
			None => return false,
		};

		F::matches(info, self.view.world_tick(), self.view.last_system_tick())
	}

	fn group_info(&self) -> GroupInfo {
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
		info: *mut ComponentInfo,
		index: usize,
		world_tick: Ticks,
		last_system_tick: Ticks,
	) -> Option<Self::Item> {
		if F::matches(&*info.add(index), world_tick, last_system_tick) {
			C::get_from_parts(data, info, index, world_tick, last_system_tick)
		} else {
			None
		}
	}
}
