mod filter;

use self::filter::GroupFilter;
use crate::components::{Component, Entity};
use crate::world::{Comp, CompMut, GroupInfo};

pub unsafe trait Query
where
	Self: Sized,
{
	type Item;

	fn get(self, entity: Entity) -> Option<Self::Item>;
}

pub unsafe trait SimpleQuery
where
	Self: Query,
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

pub struct Include<Q, F>
where
	Q: Query,
	F: GroupFilter,
{
	query: Q,
	filter: F,
}

impl<Q, F> Include<Q, F>
where
	Q: Query,
	F: GroupFilter,
{
	fn new(query: Q, filter: F) -> Self {
		Self { query, filter }
	}

	pub fn exclude<E>(self, filter: E) -> Exclude<Self, E>
	where
		E: GroupFilter,
	{
		Exclude::new(self, filter)
	}
}

unsafe impl<Q, F> Query for Include<Q, F>
where
	Q: Query,
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
}

pub struct Exclude<Q, F>
where
	Q: Query,
	F: GroupFilter,
{
	query: Q,
	filter: F,
}

impl<Q, F> Exclude<Q, F>
where
	Q: Query,
	F: GroupFilter,
{
	fn new(query: Q, filter: F) -> Self {
		Self { query, filter }
	}
}

unsafe impl<Q, F> Query for Exclude<Q, F>
where
	Q: Query,
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
}

pub unsafe trait QueryComponent {
	type Item;

	fn get(self, entity: Entity) -> Option<Self::Item>;

	fn group_info(&self) -> Option<&GroupInfo>;
}

unsafe impl<'a, T> QueryComponent for &'a Comp<'a, T>
where
	T: Component,
{
	type Item = &'a T;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		self.storage.get(entity)
	}

	fn group_info(&self) -> Option<&GroupInfo> {
		self.group_info.as_ref()
	}
}

macro_rules! impl_query {
	($(($comp:ident, $idx:tt)),*) => {
		unsafe impl<$($comp),*> Query for ($($comp,)*)
		where
			$($comp: QueryComponent,)*
		{
			type Item = ($($comp::Item,)*);

			#[allow(unused_variables)]
			fn get(self, entity: Entity) -> Option<Self::Item> {
				Some((
					$(self.$idx.get(entity)?,)*
				))
			}
		}

		unsafe impl<$($comp),*> SimpleQuery for ($($comp,)*)
		where
			$($comp: QueryComponent,)*
		{}
	};
}

impl_query!();
impl_query!((A, 0));
impl_query!((A, 0), (B, 1));
impl_query!((A, 0), (B, 1), (C, 2));
impl_query!((A, 0), (B, 1), (C, 2), (D, 3));
