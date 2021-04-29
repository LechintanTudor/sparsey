use crate::components::{Component, ComponentInfo, ComponentRefMut, Entity, SparseArray, Ticks};
use crate::dispatcher::{Comp, CompMut};
use crate::world::GroupInfo;

pub type SplitQueryElement<'a, S, T> =
	(S, &'a SparseArray, &'a [Entity], *mut ComponentInfo, *mut T);

pub unsafe trait QueryElement<'a>
where
	Self: Sized,
{
	type Item: 'a;
	type Component: Component;
	type State: Copy + 'a;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		let world_tick = self.world_tick();
		let last_system_tick = self.last_system_tick();

		let (state, sparse, _, info, data) = self.split();
		let index = sparse.get_index(entity)? as usize;

		unsafe { Self::get_from_split(data, info, index, state, world_tick, last_system_tick) }
	}

	fn get_info(&self, entity: Entity) -> Option<&ComponentInfo>;

	fn contains(&self, entity: Entity) -> bool;

	fn group_info(&self) -> Option<&GroupInfo>;

	fn world_tick(&self) -> Ticks;

	fn last_system_tick(&self) -> Ticks;

	fn split(self) -> SplitQueryElement<'a, Self::State, Self::Component>;

	unsafe fn get_from_split(
		data: *mut Self::Component,
		info: *mut ComponentInfo,
		index: usize,
		state: Self::State,
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
	type State = ();

	fn get(self, entity: Entity) -> Option<Self::Item> {
		self.storage.get(entity)
	}

	fn get_info(&self, entity: Entity) -> Option<&ComponentInfo> {
		self.storage.get_info(entity)
	}

	fn contains(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	fn group_info(&self) -> Option<&GroupInfo> {
		self.group_info.as_ref()
	}

	fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	fn last_system_tick(&self) -> Ticks {
		self.last_system_tick
	}

	fn split(self) -> SplitQueryElement<'a, Self::State, Self::Component> {
		let (sparse, entities, info, data) = self.storage.split();
		((), sparse, entities, info.as_ptr() as _, data.as_ptr() as _)
	}

	unsafe fn get_from_split(
		data: *mut Self::Component,
		_info: *mut ComponentInfo,
		index: usize,
		_state: Self::State,
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
	type State = ();

	fn get(self, entity: Entity) -> Option<Self::Item> {
		self.storage.get(entity)
	}

	fn get_info(&self, entity: Entity) -> Option<&ComponentInfo> {
		self.storage.get_info(entity)
	}

	fn contains(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	fn group_info(&self) -> Option<&GroupInfo> {
		self.group_info.as_ref()
	}

	fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	fn last_system_tick(&self) -> Ticks {
		self.last_system_tick
	}

	fn split(self) -> SplitQueryElement<'a, Self::State, Self::Component> {
		let (sparse, entities, info, data) = self.storage.split();
		((), sparse, entities, info.as_ptr() as _, data.as_ptr() as _)
	}

	unsafe fn get_from_split(
		data: *mut Self::Component,
		_info: *mut ComponentInfo,
		index: usize,
		_state: Self::State,
		_world_tick: Ticks,
		_last_system_tick: Ticks,
	) -> Option<Self::Item> {
		Some(&*data.add(index))
	}
}

unsafe impl<'a, 'b: 'a, T> QueryElement<'a> for &'a mut CompMut<'b, T>
where
	T: Component,
{
	type Item = ComponentRefMut<'a, T>;
	type Component = T;
	type State = ();

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

	fn group_info(&self) -> Option<&GroupInfo> {
		self.group_info.as_ref()
	}

	fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	fn last_system_tick(&self) -> Ticks {
		self.last_system_tick
	}

	fn split(self) -> SplitQueryElement<'a, Self::State, Self::Component> {
		let (sparse, entities, info, data) = self.storage.split();
		((), sparse, entities, info.as_ptr() as _, data.as_ptr() as _)
	}

	unsafe fn get_from_split(
		data: *mut Self::Component,
		info: *mut ComponentInfo,
		index: usize,
		_state: Self::State,
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
