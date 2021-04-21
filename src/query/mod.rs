mod group_filter;

use self::group_filter::GroupFilter;
use crate::components::{Component, Entity};
use crate::world::{Comp, CompMut, GroupInfo};

pub unsafe trait Query<'a>
where
	Self: Sized,
{
	type Item: 'a;

	fn get(self, entity: Entity) -> Option<Self::Item>;

	fn contains(&self, entity: Entity) -> bool;
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

	fn contains(&self, entity: Entity) -> bool {
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

	fn contains(&self, entity: Entity) -> bool {
		if self.filter.excludes_all(entity) {
			self.query.contains(entity)
		} else {
			false
		}
	}
}

pub unsafe trait QueryComponent<'a> {
	type Item: 'a;

	fn get(self, entity: Entity) -> Option<Self::Item>;

	fn contains(&self, entity: Entity) -> bool;

	fn group_info(&self) -> Option<&GroupInfo>;
}

unsafe impl<'a, T> QueryComponent<'a> for &'a Comp<'a, T>
where
	T: Component,
{
	type Item = &'a T;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		self.storage.get(entity)
	}

	fn contains(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	fn group_info(&self) -> Option<&GroupInfo> {
		self.group_info.as_ref()
	}
}

unsafe impl<'a, T> QueryComponent<'a> for &'a CompMut<'a, T>
where
	T: Component,
{
	type Item = &'a T;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		self.storage.get(entity)
	}

	fn contains(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	fn group_info(&self) -> Option<&GroupInfo> {
		self.group_info.as_ref()
	}
}

macro_rules! impl_query {
	($(($comp:ident, $idx:tt)),*) => {
		unsafe impl<'a, $($comp),*> Query<'a> for ($($comp,)*)
		where
			$($comp: QueryComponent<'a>,)*
		{
			type Item = ($($comp::Item,)*);

			#[allow(unused_variables)]
			fn get(self, entity: Entity) -> Option<Self::Item> {
				Some((
					$(self.$idx.get(entity)?,)*
				))
			}

			#[allow(unused_variables)]
			fn contains(&self, entity: Entity) -> bool {
				true $(&& self.$idx.contains(entity))*
			}
		}

		unsafe impl<'a, $($comp),*> SimpleQuery<'a> for ($($comp,)*)
		where
			$($comp: QueryComponent<'a>,)*
		{}
	};
}

impl_query!();
impl_query!((A, 0));
impl_query!((A, 0), (B, 1));
impl_query!((A, 0), (B, 1), (C, 2));
impl_query!((A, 0), (B, 1), (C, 2), (D, 3));
