#[macro_use]
mod split;

mod base;
mod component_filter;
mod component_info_filter;
mod component_view;
mod iter;

pub use self::component_filter::*;
pub use self::component_info_filter::*;
pub use self::component_view::*;
pub use self::iter::*;

use crate::components::{Entity, Ticks};
use crate::query::base::BaseQuery;
use crate::world::{CombinedGroupInfo, QueryGroupInfo};

pub struct Include<Q, I> {
	query: Q,
	include: I,
}

pub struct IncludeExclude<Q, I, E> {
	query: Q,
	include: I,
	exclude: E,
}

pub struct IncludeExcludeFilter<Q, I, E, F> {
	query: Q,
	include: I,
	exclude: E,
	filter: F,
}

pub unsafe trait Query<'a> {
	type Item;
	type Iterator: Iterator<Item = Self::Item>;

	fn get(self, entity: Entity) -> Option<Self::Item>;

	fn contains(&self, entity: Entity) -> bool;

	fn iter(self) -> Self::Iterator;
}

unsafe impl<'a, Q> Query<'a> for Q
where
	Q: BaseQuery<'a>,
{
	type Item = <Q as BaseQuery<'a>>::Item;
	type Iterator = Iter<'a, Q, (), (), ()>;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		<Q as BaseQuery<'a>>::get(self, entity)
	}

	fn contains(&self, entity: Entity) -> bool {
		<Q as BaseQuery<'a>>::contains(self, entity)
	}

	fn iter(self) -> Self::Iterator {
		Iter::new(self, (), (), ())
	}
}
