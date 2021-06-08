#[macro_use]
mod split;

mod base;
mod component_view;
mod composite;
mod errors;
mod filter;
mod iter;
mod modifier;
mod slice;

pub use self::base::{QueryBaseModifiers, UnfilteredQueryBase};
pub use self::component_view::*;
pub use self::composite::*;
pub use self::errors::*;
pub use self::filter::*;
pub use self::iter::*;
pub use self::modifier::*;
pub use self::slice::*;

use crate::components::Entity;
use crate::query::base::QueryBase;
use crate::world::QueryGroupInfo;

pub unsafe trait Query<'a> {
	type Item;
	type Iterator: Iterator<Item = Self::Item>;

	fn get(self, entity: Entity) -> Option<Self::Item>;

	fn contains(self, entity: Entity) -> bool;

	fn iter(self) -> Self::Iterator;
}

pub trait IntoQueryParts<'a> {
	type Base: QueryBase<'a>;
	type Include: QueryModifier<'a>;
	type Exclude: QueryModifier<'a>;
	type Filter: QueryFilter;

	fn into_parts(self) -> (Self::Base, Self::Include, Self::Exclude, Self::Filter);
}

unsafe impl<'a, Q> Query<'a> for Q
where
	Q: IntoQueryParts<'a>,
{
	type Item = <Q::Base as QueryBase<'a>>::Item;
	type Iterator = Iter<'a, Q::Base, Q::Include, Q::Exclude, Q::Filter>;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		let (base, include, exclude, filter) = self.into_parts();

		if filter.matches(entity) && exclude.excludes(entity) && include.includes(entity) {
			QueryBase::get(base, entity)
		} else {
			None
		}
	}

	fn contains(self, entity: Entity) -> bool {
		let (base, include, exclude, filter) = self.into_parts();

		filter.matches(entity)
			&& exclude.excludes(entity)
			&& include.includes(entity)
			&& base.contains(entity)
	}

	fn iter(self) -> Self::Iterator {
		let (base, include, exclude, filter) = self.into_parts();
		Iter::new(base, include, exclude, filter)
	}
}
