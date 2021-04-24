mod group_filter;
mod info_filter;
mod iter;

use crate::components::{Component, ComponentInfo, Entity, SparseArray, Ticks};
use crate::dispatcher::{Comp, CompMut};
use crate::query::group_filter::GroupFilter;
use crate::world::GroupInfo;

pub unsafe trait Query<'a>
where
	Self: Sized,
{
	type Item: 'a;

	fn get(self, entity: Entity) -> Option<Self::Item>;

	fn contains(self, entity: Entity) -> bool;
}

pub unsafe trait SimpleQuery<'a>
where
	Self: Query<'a>,
{
	fn include<F>(self, filter: F) -> Include<Self, F>
	where
		F: GroupFilter,
	{
		Include::new(self, filter)
	}

	fn exclude<F>(self, filter: F) -> Exclude<Include<Self, ()>, F>
	where
		F: GroupFilter,
	{
		Exclude::new(Include::new(self, ()), filter)
	}
}

pub struct Include<Q, F> {
	query: Q,
	filter: F,
}

impl<Q, F> Include<Q, F> {
	fn new<'a>(query: Q, filter: F) -> Self
	where
		Q: Query<'a>,
		F: GroupFilter,
	{
		Self { query, filter }
	}

	pub fn exclude<'a, E>(self, filter: E) -> Exclude<Self, E>
	where
		Q: Query<'a>,
		F: GroupFilter,
		E: GroupFilter,
	{
		Exclude::new(self, filter)
	}
}

unsafe impl<'a, Q, F> Query<'a> for Include<Q, F>
where
	Q: Query<'a>,
	F: GroupFilter,
{
	type Item = Q::Item;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		if self.filter.includes_all(entity) {
			self.query.get(entity)
		} else {
			None
		}
	}

	fn contains(self, entity: Entity) -> bool {
		if self.filter.includes_all(entity) {
			self.query.contains(entity)
		} else {
			false
		}
	}
}

pub struct Exclude<Q, F> {
	query: Q,
	filter: F,
}

impl<Q, F> Exclude<Q, F> {
	fn new<'a>(query: Q, filter: F) -> Self
	where
		Q: Query<'a>,
		F: GroupFilter,
	{
		Self { query, filter }
	}
}

unsafe impl<'a, Q, F> Query<'a> for Exclude<Q, F>
where
	Q: Query<'a>,
	F: GroupFilter,
{
	type Item = Q::Item;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		if self.filter.excludes_all(entity) {
			self.query.get(entity)
		} else {
			None
		}
	}

	fn contains(self, entity: Entity) -> bool {
		if self.filter.excludes_all(entity) {
			self.query.contains(entity)
		} else {
			false
		}
	}
}

pub type SplitQueryElement<'a, S, T> =
	(S, &'a SparseArray, &'a [Entity], *mut ComponentInfo, *mut T);

pub unsafe trait QueryElement<'a>
where
	Self: Sized,
{
	type Item: 'a;
	type Component: Component;
	type SplitState: 'a;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		let (state, sparse, _, info, data) = self.split();
		let index = sparse.get_index(entity)? as usize;

		unsafe { Self::get_from_split(state, info, data, index) }
	}

	fn contains(self, entity: Entity) -> bool;

	fn group_info(&self) -> Option<&GroupInfo>;

	fn split(self) -> SplitQueryElement<'a, Self::SplitState, Self::Component>;

	unsafe fn get_from_split(
		state: Self::SplitState,
		info: *mut ComponentInfo,
		data: *mut Self::Component,
		index: usize,
	) -> Option<Self::Item>;
}

unsafe impl<'a, T> QueryElement<'a> for &'a Comp<'a, T>
where
	T: Component,
{
	type Item = &'a T;
	type Component = T;
	type SplitState = ();

	fn get(self, entity: Entity) -> Option<Self::Item> {
		self.storage.get(entity)
	}

	fn contains(self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	fn group_info(&self) -> Option<&GroupInfo> {
		Comp::group_info(self)
	}

	fn split(self) -> SplitQueryElement<'a, Self::SplitState, Self::Component> {
		let (sparse, entities, info, data) = self.storage.split();
		((), sparse, entities, info.as_ptr() as _, data.as_ptr() as _)
	}

	unsafe fn get_from_split(
		_state: Self::SplitState,
		_info: *mut ComponentInfo,
		data: *mut Self::Component,
		index: usize,
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
	type SplitState = ();

	fn get(self, entity: Entity) -> Option<Self::Item> {
		self.storage.get(entity)
	}

	fn contains(self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	fn group_info(&self) -> Option<&GroupInfo> {
		CompMut::group_info(self)
	}

	fn split(self) -> SplitQueryElement<'a, Self::SplitState, Self::Component> {
		let (sparse, entities, info, data) = self.storage.split();
		((), sparse, entities, info.as_ptr() as _, data.as_ptr() as _)
	}

	unsafe fn get_from_split(
		_state: Self::SplitState,
		_info: *mut ComponentInfo,
		data: *mut Self::Component,
		index: usize,
	) -> Option<Self::Item> {
		Some(&*data.add(index))
	}
}

pub unsafe trait UnfilteredQueryElement<'a>
where
	Self: QueryElement<'a, SplitState = ()>,
{
	fn world_tick(&self) -> Ticks;

	fn last_system_tick(&self) -> Ticks;
}

unsafe impl<'a, T> UnfilteredQueryElement<'a> for &'a Comp<'a, T>
where
	T: Component,
{
	fn world_tick(&self) -> Ticks {
		Comp::world_tick(self)
	}

	fn last_system_tick(&self) -> Ticks {
		Comp::last_system_tick(self)
	}
}

unsafe impl<'a, T> UnfilteredQueryElement<'a> for &'a CompMut<'a, T>
where
	T: Component,
{
	fn world_tick(&self) -> Ticks {
		CompMut::world_tick(self)
	}

	fn last_system_tick(&self) -> Ticks {
		CompMut::last_system_tick(self)
	}
}

macro_rules! impl_query {
	($(($elem:ident, $idx:tt)),*) => {
		unsafe impl<'a, $($elem),*> Query<'a> for ($($elem,)*)
		where
			$($elem: QueryElement<'a>,)*
		{
			type Item = ($($elem::Item,)*);

			#[allow(unused_variables)]
			fn get(self, entity: Entity) -> Option<Self::Item> {
				Some((
					$(self.$idx.get(entity)?,)*
				))
			}

			#[allow(unused_variables)]
			fn contains(self, entity: Entity) -> bool {
				true $(&& self.$idx.contains(entity))*
			}
		}

		unsafe impl<'a, $($elem),*> SimpleQuery<'a> for ($($elem,)*)
		where
			$($elem: QueryElement<'a>,)*
		{}
	};
}

impl_query!();
impl_query!((A, 0));
impl_query!((A, 0), (B, 1));
impl_query!((A, 0), (B, 1), (C, 2));
impl_query!((A, 0), (B, 1), (C, 2), (D, 3));
