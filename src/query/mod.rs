pub use self::element::*;
pub use self::group_filter::*;
pub use self::iter::*;
pub use self::state_filter::*;

mod element;
mod group_filter;
mod iter;
mod state_filter;

use crate::components::Entity;

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
	fn include<I>(self, include: I) -> Include<Self, I>
	where
		I: GroupFilter,
	{
		Include::new(self, include)
	}

	fn exclude<E>(self, exclude: E) -> Exclude<Include<Self, ()>, E>
	where
		E: GroupFilter,
	{
		Exclude::new(Include::new(self, ()), exclude)
	}

	fn filter<F>(self, filter: F) -> Filter<Exclude<Include<Self, ()>, ()>, F>
	where
		F: StateFilter,
	{
		Filter::new(Exclude::new(Include::new(self, ()), ()), filter)
	}
}

pub struct Include<Q, I> {
	query: Q,
	include: I,
}

impl<Q, I> Include<Q, I> {
	fn new(query: Q, include: I) -> Self {
		Self { query, include }
	}

	pub fn exclude<E>(self, exclude: E) -> Exclude<Self, E> {
		Exclude::new(self, exclude)
	}

	pub fn filter<F>(self, filter: F) -> Filter<Exclude<Self, ()>, F> {
		Filter::new(Exclude::new(self, ()), filter)
	}
}

unsafe impl<'a, Q, I> Query<'a> for Include<Q, I>
where
	Q: Query<'a>,
	I: GroupFilter,
{
	type Item = Q::Item;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		if self.include.includes_all(entity) {
			self.query.get(entity)
		} else {
			None
		}
	}

	fn contains(&self, entity: Entity) -> bool {
		self.include.includes_all(entity) && self.query.contains(entity)
	}
}

pub struct Exclude<Q, E> {
	query: Q,
	exclude: E,
}

impl<Q, E> Exclude<Q, E> {
	fn new(query: Q, exclude: E) -> Self {
		Self { query, exclude }
	}

	pub fn filter<F>(self, filter: F) -> Filter<Self, F>
	where
		F: StateFilter,
	{
		Filter::new(self, filter)
	}
}

unsafe impl<'a, Q, E> Query<'a> for Exclude<Q, E>
where
	Q: Query<'a>,
	E: GroupFilter,
{
	type Item = Q::Item;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		if self.exclude.excludes_all(entity) {
			self.query.get(entity)
		} else {
			None
		}
	}

	fn contains(&self, entity: Entity) -> bool {
		self.exclude.excludes_all(entity) && self.query.contains(entity)
	}
}

pub struct Filter<Q, F> {
	query: Q,
	filter: F,
}

impl<Q, F> Filter<Q, F> {
	fn new(query: Q, filter: F) -> Self {
		Self { query, filter }
	}
}

unsafe impl<'a, Q, F> Query<'a> for Filter<Q, F>
where
	Q: Query<'a>,
	F: StateFilter,
{
	type Item = Q::Item;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		if self.filter.matches(entity) {
			self.query.get(entity)
		} else {
			None
		}
	}

	fn contains(&self, entity: Entity) -> bool {
		self.filter.matches(entity) && self.query.contains(entity)
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
			fn contains(&self, entity: Entity) -> bool {
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
