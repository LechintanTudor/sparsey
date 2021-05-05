use crate::components::{
	Component, ComponentInfo, ComponentRefMut, Entity, SparseArrayView, Ticks,
};
use crate::dispatcher::{Comp, CompMut};
use crate::world::GroupInfo;

pub type SplitQueryElement<'a, T> = (
	SparseArrayView<'a>,
	&'a [Entity],
	*mut T,
	*mut ComponentInfo,
);

pub unsafe trait QueryElement<'a> {
	type Item;
	type Component: Component;

	fn get(self, entity: Entity) -> Option<Self::Item>;

	fn get_info(&self, entity: Entity) -> Option<&ComponentInfo>;

	fn contains(&self, entity: Entity) -> bool;

	fn group_info(&self) -> Option<GroupInfo>;

	fn world_tick(&self) -> Ticks;

	fn last_system_tick(&self) -> Ticks;

	fn split(self) -> SplitQueryElement<'a, Self::Component>;

	unsafe fn get_from_parts(
		data: *mut Self::Component,
		info: *mut ComponentInfo,
		index: usize,
		world_tick: Ticks,
		last_system_tick: Ticks,
	) -> Option<Self::Item>;
}

unsafe impl<'a, T> QueryElement<'a> for &'a Comp<'a, T>
where
	T: Component,
{
	type Item = &'a T;
	type Component = T;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		self.storage.get(entity)
	}

	fn get_info(&self, entity: Entity) -> Option<&ComponentInfo> {
		self.storage.get_info(entity)
	}

	fn contains(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	fn group_info(&self) -> Option<GroupInfo> {
		self.group_info
	}

	fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	fn last_system_tick(&self) -> Ticks {
		self.last_system_tick
	}

	fn split(self) -> SplitQueryElement<'a, Self::Component> {
		let (sparse, entities, data, info) = self.storage.split();
		(
			sparse,
			entities,
			data.as_ptr() as *mut _,
			info.as_ptr() as *mut _,
		)
	}

	unsafe fn get_from_parts(
		data: *mut Self::Component,
		_info: *mut ComponentInfo,
		index: usize,
		_world_tick: Ticks,
		_last_system_tick: Ticks,
	) -> Option<Self::Item> {
		Some(&*data.add(index))
	}
}

unsafe impl<'a, T> QueryElement<'a> for &'a CompMut<'a, T>
where
	T: Component,
{
	type Item = &'a T;
	type Component = T;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		self.storage.get(entity)
	}

	fn get_info(&self, entity: Entity) -> Option<&ComponentInfo> {
		self.storage.get_info(entity)
	}

	fn contains(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	fn group_info(&self) -> Option<GroupInfo> {
		self.group_info
	}

	fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	fn last_system_tick(&self) -> Ticks {
		self.last_system_tick
	}

	fn split(self) -> SplitQueryElement<'a, Self::Component> {
		let (sparse, entities, data, info) = self.storage.split();
		(
			sparse,
			entities,
			data.as_ptr() as *mut _,
			info.as_ptr() as *mut _,
		)
	}

	unsafe fn get_from_parts(
		data: *mut Self::Component,
		_info: *mut ComponentInfo,
		index: usize,
		_world_tick: Ticks,
		_last_system_tick: Ticks,
	) -> Option<Self::Item> {
		Some(&*data.add(index))
	}
}

unsafe impl<'a: 'b, 'b, T> QueryElement<'a> for &'a mut CompMut<'b, T>
where
	T: Component,
{
	type Item = ComponentRefMut<'a, T>;
	type Component = T;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		let (data, info) = self.storage.get_with_info_mut(entity)?;
		Some(ComponentRefMut::new(data, info, self.world_tick))
	}

	fn get_info(&self, entity: Entity) -> Option<&ComponentInfo> {
		self.storage.get_info(entity)
	}

	fn contains(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	fn group_info(&self) -> Option<GroupInfo> {
		self.group_info
	}

	fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	fn last_system_tick(&self) -> Ticks {
		self.last_system_tick
	}

	fn split(self) -> SplitQueryElement<'a, Self::Component> {
		let (sparse, entities, data, info) = self.storage.split();
		(
			sparse,
			entities,
			data.as_ptr() as *mut _,
			info.as_ptr() as *mut _,
		)
	}

	unsafe fn get_from_parts(
		data: *mut Self::Component,
		info: *mut ComponentInfo,
		index: usize,
		world_tick: Ticks,
		_last_system_tick: Ticks,
	) -> Option<Self::Item> {
		Some(ComponentRefMut::new(
			&mut *data.add(index),
			&mut *info.add(index),
			world_tick,
		))
	}
}

pub unsafe trait UnfilteredQueryElement<'a>
where
	Self: QueryElement<'a>,
{
	// Marker
}

unsafe impl<'a, T> UnfilteredQueryElement<'a> for &'a Comp<'a, T>
where
	T: Component,
{
	// Marker
}

unsafe impl<'a, T> UnfilteredQueryElement<'a> for &'a CompMut<'a, T>
where
	T: Component,
{
	// Marker
}

unsafe impl<'a: 'b, 'b, T> UnfilteredQueryElement<'a> for &'a mut CompMut<'b, T>
where
	T: Component,
{
	// Marker
}
