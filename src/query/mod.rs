#[macro_use]
mod split;

mod base;
mod component_view;
mod composite;
mod filter;
mod iter;
mod modifier;

pub use self::base::QueryBaseModifiers;
pub use self::component_view::*;
pub use self::composite::*;
pub use self::filter::*;
pub use self::iter::*;
pub use self::modifier::*;

use crate::components::Entity;
use crate::query::base::QueryBase;
use crate::world::QueryGroupInfo;

pub unsafe trait Query<'a> {
	type Item;
	type Iterator: Iterator<Item = Self::Item>;

	fn get(self, entity: Entity) -> Option<Self::Item>;

	fn contains(&self, entity: Entity) -> bool;

	fn iter(self) -> Self::Iterator;
}

unsafe impl<'a, Q> Query<'a> for Q
where
	Q: QueryBase<'a>,
{
	type Item = <Q as QueryBase<'a>>::Item;
	type Iterator = Iter<'a, Q, (), (), PassthroughFilter>;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		<Q as QueryBase<'a>>::get(self, entity)
	}

	fn contains(&self, entity: Entity) -> bool {
		<Q as QueryBase<'a>>::contains(self, entity)
	}

	fn iter(self) -> Self::Iterator {
		Iter::new(self, (), (), PassthroughFilter)
	}
}

unsafe impl<'a, Q, I> Query<'a> for Include<Q, I>
where
	Q: QueryBase<'a>,
	I: QueryModifier<'a>,
{
	type Item = <Q as QueryBase<'a>>::Item;
	type Iterator = Iter<'a, Q, I, (), PassthroughFilter>;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		if self.include.includes(entity) {
			self.query.get(entity)
		} else {
			None
		}
	}

	fn contains(&self, entity: Entity) -> bool {
		self.include.includes(entity) && self.query.contains(entity)
	}

	fn iter(self) -> Self::Iterator {
		Iter::new(self.query, self.include, (), PassthroughFilter)
	}
}

unsafe impl<'a, Q, I, E> Query<'a> for IncludeExclude<Q, I, E>
where
	Q: QueryBase<'a>,
	I: QueryModifier<'a>,
	E: QueryModifier<'a>,
{
	type Item = <Q as QueryBase<'a>>::Item;
	type Iterator = Iter<'a, Q, I, E, PassthroughFilter>;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		if self.exclude.excludes(entity) && self.include.includes(entity) {
			self.query.get(entity)
		} else {
			None
		}
	}

	fn contains(&self, entity: Entity) -> bool {
		self.exclude.excludes(entity)
			&& self.include.includes(entity)
			&& self.query.contains(entity)
	}

	fn iter(self) -> Self::Iterator {
		Iter::new(self.query, self.include, self.exclude, PassthroughFilter)
	}
}

unsafe impl<'a, Q, I, E, F> Query<'a> for IncludeExcludeFilter<Q, I, E, F>
where
	Q: QueryBase<'a>,
	I: QueryModifier<'a>,
	E: QueryModifier<'a>,
	F: QueryFilter,
{
	type Item = <Q as QueryBase<'a>>::Item;
	type Iterator = Iter<'a, Q, I, E, F>;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		if self.filter.matches(entity)
			&& self.exclude.excludes(entity)
			&& self.include.includes(entity)
		{
			self.query.get(entity)
		} else {
			None
		}
	}

	fn contains(&self, entity: Entity) -> bool {
		self.filter.matches(entity)
			&& self.exclude.excludes(entity)
			&& self.include.includes(entity)
			&& self.query.contains(entity)
	}

	fn iter(self) -> Self::Iterator {
		Iter::new(self.query, self.include, self.exclude, self.filter)
	}
}
