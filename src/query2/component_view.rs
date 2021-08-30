use crate::components::Component;
use crate::group::GroupInfo;
use crate::query2::{ComponentRefMut, Passthrough, QueryElement, SplitQueryElement};
use crate::storage::{ComponentStorage, Entity, TypedComponentStorage};
use crate::utils::{ChangeTicks, Ticks};
use std::ops::{Deref, DerefMut};

pub struct ComponentView<'a, T, S> {
	storage: TypedComponentStorage<T, S>,
	group_info: GroupInfo<'a>,
	world_tick: Ticks,
	change_tick: Ticks,
}

impl<'a, T, S> ComponentView<'a, T, S>
where
	T: Component,
	S: Deref<Target = ComponentStorage>,
{
	pub(crate) unsafe fn new(
		storage: S,
		group_info: GroupInfo<'a>,
		world_tick: Ticks,
		change_tick: Ticks,
	) -> Self {
		Self {
			storage: TypedComponentStorage::new(storage),
			group_info,
			world_tick,
			change_tick,
		}
	}
}

unsafe impl<'a, T, S> QueryElement<'a> for &'a ComponentView<'a, T, S>
where
	T: Component,
	S: Deref<Target = ComponentStorage>,
{
	type Item = &'a T;
	type Component = T;
	type Filter = Passthrough;

	#[inline]
	fn get(self, entity: Entity) -> Option<Self::Item> {
		self.storage.get(entity)
	}

	#[inline]
	fn get_with_ticks(&self, entity: Entity) -> Option<(&Self::Component, &ChangeTicks)> {
		self.storage.get_with_ticks(entity)
	}

	#[inline]
	fn contains(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	#[inline]
	fn group_info(&self) -> GroupInfo<'a> {
		self.group_info
	}

	#[inline]
	fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	#[inline]
	fn change_tick(&self) -> Ticks {
		self.change_tick
	}

	fn split(self) -> SplitQueryElement<'a, Self::Component, Self::Filter> {
		let (sparse, entities, components, ticks) = self.storage.split();

		SplitQueryElement::new(
			sparse,
			entities,
			components.as_ptr() as _,
			ticks.as_ptr() as _,
			Passthrough,
		)
	}

	#[inline]
	unsafe fn get_from_parts(
		component: *mut Self::Component,
		_ticks: *mut ChangeTicks,
		_world_tick: Ticks,
		_change_tick: Ticks,
	) -> Self::Item {
		&*component
	}
}

unsafe impl<'a, 'b, T, S> QueryElement<'a> for &'a mut ComponentView<'b, T, S>
where
	T: Component,
	S: Deref<Target = ComponentStorage> + DerefMut,
{
	type Item = ComponentRefMut<'a, T>;
	type Component = T;
	type Filter = Passthrough;

	#[inline]
	fn get(self, entity: Entity) -> Option<Self::Item> {
		let (component, ticks) = self.storage.get_with_ticks_mut(entity)?;
		Some(ComponentRefMut::new(component, ticks, self.world_tick))
	}

	#[inline]
	fn get_with_ticks(&self, entity: Entity) -> Option<(&Self::Component, &ChangeTicks)> {
		self.storage.get_with_ticks(entity)
	}

	#[inline]
	fn contains(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	#[inline]
	fn group_info(&self) -> GroupInfo<'a> {
		self.group_info
	}

	#[inline]
	fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	#[inline]
	fn change_tick(&self) -> Ticks {
		self.change_tick
	}

	fn split(self) -> SplitQueryElement<'a, Self::Component, Self::Filter> {
		let (sparse, entities, components, ticks) = self.storage.split_mut();

		SplitQueryElement::new(
			sparse,
			entities,
			components.as_mut_ptr(),
			ticks.as_mut_ptr(),
			Passthrough,
		)
	}

	#[inline]
	unsafe fn get_from_parts(
		component: *mut Self::Component,
		ticks: *mut ChangeTicks,
		world_tick: Ticks,
		_change_tick: Ticks,
	) -> Self::Item {
		ComponentRefMut::new(&mut *component, &mut *ticks, world_tick)
	}
}
