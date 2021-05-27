use crate::components::{ComponentTicks, Entity, Ticks};
use crate::query::{
	AndFilter, ComponentInfoFilter, ComponentView, OrFilter, QueryFilter, SplitComponentView,
	UnfilteredComponentView,
};
use crate::world::GroupInfo;
use std::marker::PhantomData;
use std::ops::{BitAnd, BitOr};

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

impl<'a, C, F> QueryFilter for FilteredComponentView<C, F>
where
	C: UnfilteredComponentView<'a>,
	F: ComponentInfoFilter,
{
	fn matches(&self, entity: Entity) -> bool {
		F::matches(
			self.view.get_ticks(entity),
			self.view.world_tick(),
			self.view.last_system_tick(),
		)
	}
}

impl<'a, C, F, Q> BitAnd<Q> for FilteredComponentView<C, F>
where
	C: UnfilteredComponentView<'a>,
	F: ComponentInfoFilter,
	Q: QueryFilter,
{
	type Output = AndFilter<Self, Q>;

	fn bitand(self, other: Q) -> Self::Output {
		AndFilter::new(self, other)
	}
}

impl<'a, C, F, Q> BitOr<Q> for FilteredComponentView<C, F>
where
	C: UnfilteredComponentView<'a>,
	F: ComponentInfoFilter,
	Q: QueryFilter,
{
	type Output = OrFilter<Self, Q>;

	fn bitor(self, other: Q) -> Self::Output {
		OrFilter::new(self, other)
	}
}
