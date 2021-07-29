use crate::components::{ComponentTicks, Entity, Ticks};
use crate::group::GroupInfo;
use crate::query::{
	ComponentView, Filter, ImmutableUnfilteredComponentView, QueryFilter, SplitComponentView,
};

#[doc(hidden)]
#[derive(Clone, Copy, Default, Debug)]
pub struct Contains;

/// Creates a query filter thats checks if the component view contains a
/// component.
pub fn contains<'a, C>(component_view: C) -> Filter<C, Contains>
where
	C: ImmutableUnfilteredComponentView<'a>,
{
	Filter::new(component_view)
}

impl<'a, C> QueryFilter for Filter<C, Contains>
where
	C: ImmutableUnfilteredComponentView<'a>,
{
	fn matches(&self, entity: Entity) -> bool {
		self.component_view.contains(entity)
	}
}

#[doc(hidden)]
#[derive(Clone, Copy, Default, Debug)]
pub struct Maybe;

/// Creates a filtered component view which returns `Options`.
pub fn maybe<'a, C>(component_view: C) -> Filter<C, Maybe>
where
	C: ComponentView<'a>,
{
	Filter::new(component_view)
}

unsafe impl<'a, C> ComponentView<'a> for Filter<C, Maybe>
where
	C: ComponentView<'a>,
{
	type Item = Option<C::Item>;
	type Component = C::Component;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		Some(self.component_view.get(entity))
	}

	fn get_ticks(&self, entity: Entity) -> Option<&ComponentTicks> {
		self.component_view.get_ticks(entity)
	}

	fn contains(&self, entity: Entity) -> bool {
		self.component_view.contains(entity)
	}

	fn group_info(&self) -> GroupInfo<'a> {
		self.component_view.group_info()
	}

	fn world_tick(&self) -> Ticks {
		self.component_view.world_tick()
	}

	fn last_system_tick(&self) -> Ticks {
		self.component_view.last_system_tick()
	}

	fn into_parts(self) -> SplitComponentView<'a, Self::Component> {
		self.component_view.into_parts()
	}

	unsafe fn get_from_parts(
		data: *mut Self::Component,
		ticks: *mut ComponentTicks,
		index: usize,
		world_tick: Ticks,
		last_system_tick: Ticks,
	) -> Option<Self::Item> {
		Some(C::get_from_parts(
			data,
			ticks,
			index,
			world_tick,
			last_system_tick,
		))
	}
}
