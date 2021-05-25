use crate::components::{
	Component, ComponentInfo, ComponentRefMut, Entity, SparseArrayView, Ticks,
};
use crate::dispatcher::{Comp, CompMut};
use crate::world::GroupInfo;
use std::marker::PhantomData;

pub unsafe trait ComponentView<'a>
where
	Self: Sized,
{
	type Item;
	type Component: Component;

	fn get(self, entity: Entity) -> Option<Self::Item>;

	fn get_info(&self, entity: Entity) -> Option<&ComponentInfo>;

	fn contains(&self, entity: Entity) -> bool;

	fn group_info(&self) -> GroupInfo<'a>;

	fn world_tick(&self) -> Ticks;

	fn last_system_tick(&self) -> Ticks;

	fn split(self) -> SplitComponentView<'a, Self::Component>;

	fn split_sparse(self) -> (&'a [Entity], SparseSplitComponentView<'a, Self::Component>) {
		let (sparse, entities, data, info) = self.split();
		(entities, SparseSplitComponentView::new(sparse, data, info))
	}

	fn split_dense(self) -> (&'a [Entity], DenseSplitComponentView<'a, Self::Component>) {
		let (_, entities, data, info) = self.split();
		(entities, DenseSplitComponentView::new(data, info))
	}

	unsafe fn get_from_parts(
		data: *mut Self::Component,
		info: *mut ComponentInfo,
		index: usize,
		world_tick: Ticks,
		last_system_tick: Ticks,
	) -> Option<Self::Item>;
}

unsafe impl<'a, T> ComponentView<'a> for &'a Comp<'a, T>
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

	fn group_info(&self) -> GroupInfo<'a> {
		self.group_info
	}

	fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	fn last_system_tick(&self) -> Ticks {
		self.last_system_tick
	}

	fn split(self) -> SplitComponentView<'a, Self::Component> {
		let (sparse, entities, data, info) = self.storage.split();
		(sparse, entities, data.as_ptr() as _, info.as_ptr() as _)
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

unsafe impl<'a, T> ComponentView<'a> for &'a CompMut<'a, T>
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

	fn group_info(&self) -> GroupInfo<'a> {
		self.group_info
	}

	fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	fn last_system_tick(&self) -> Ticks {
		self.last_system_tick
	}

	fn split(self) -> SplitComponentView<'a, Self::Component> {
		let (sparse, entities, data, info) = self.storage.split();
		(sparse, entities, data.as_ptr() as _, info.as_ptr() as _)
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
unsafe impl<'a, 'b, T> ComponentView<'a> for &'a mut CompMut<'b, T>
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

	fn group_info(&self) -> GroupInfo<'a> {
		self.group_info
	}

	fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	fn last_system_tick(&self) -> Ticks {
		self.last_system_tick
	}

	fn split(self) -> SplitComponentView<'a, Self::Component> {
		let (sparse, entities, data, info) = self.storage.split();
		(sparse, entities, data.as_ptr() as _, info.as_ptr() as _)
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

pub type SplitComponentView<'a, T> = (
	SparseArrayView<'a>,
	&'a [Entity],
	*mut T,
	*mut ComponentInfo,
);

#[derive(Copy, Clone)]
pub struct SparseSplitComponentView<'a, T> {
	sparse: SparseArrayView<'a>,
	data: *mut T,
	info: *mut ComponentInfo,
}

impl<'a, T> SparseSplitComponentView<'a, T> {
	fn new(sparse: SparseArrayView<'a>, data: *mut T, info: *mut ComponentInfo) -> Self {
		Self { sparse, data, info }
	}

	pub unsafe fn get<V>(
		&mut self,
		entity: Entity,
		world_tick: Ticks,
		last_system_tick: Ticks,
	) -> Option<V::Item>
	where
		V: ComponentView<'a, Component = T>,
	{
		let index = self.sparse.get_index(entity)? as usize;
		V::get_from_parts(self.data, self.info, index, world_tick, last_system_tick)
	}
}

#[derive(Copy, Clone)]
pub struct DenseSplitComponentView<'a, T> {
	data: *mut T,
	info: *mut ComponentInfo,
	_phantom: PhantomData<&'a ()>,
}

impl<'a, T> DenseSplitComponentView<'a, T> {
	fn new(data: *mut T, info: *mut ComponentInfo) -> Self {
		Self {
			data,
			info,
			_phantom: PhantomData,
		}
	}

	pub unsafe fn get<V>(
		&mut self,
		index: usize,
		world_tick: Ticks,
		last_system_tick: Ticks,
	) -> Option<V::Item>
	where
		V: ComponentView<'a, Component = T>,
	{
		V::get_from_parts(self.data, self.info, index, world_tick, last_system_tick)
	}
}

pub unsafe trait UnfilteredComponentView<'a>
where
	Self: ComponentView<'a>,
{
	// Empty
}

unsafe impl<'a, T> UnfilteredComponentView<'a> for &'a Comp<'a, T>
where
	T: Component,
{
	// Empty
}

unsafe impl<'a, T> UnfilteredComponentView<'a> for &'a CompMut<'a, T>
where
	T: Component,
{
	// Empty
}

unsafe impl<'a, 'b, T> UnfilteredComponentView<'a> for &'a mut CompMut<'b, T>
where
	T: Component,
{
	// Empty
}
