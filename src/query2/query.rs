use crate::query2::{QueryBase, QueryFilter, QueryModifier};
use crate::storage::Entity;

pub trait IntoQueryParts<'a> {
	type Base: QueryBase<'a>;
	type Include: QueryModifier<'a>;
	type Exclude: QueryModifier<'a>;
	type Filter: QueryFilter;

	fn into_query_parts(self) -> (Self::Base, Self::Include, Self::Exclude, Self::Filter);
}

pub trait Query<'a>
where
	Self: IntoQueryParts<'a>,
{
	type Item;

	fn get(self, entity: Entity) -> Option<Self::Item>;

	fn contains(self, entity: Entity) -> bool;
}

impl<'a, Q> Query<'a> for Q
where
	Q: IntoQueryParts<'a>,
{
	type Item = <Q::Base as QueryBase<'a>>::Item;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		let (base, include, exclude, filter) = self.into_query_parts();

		if filter.matches(entity) && exclude.excludes(entity) && include.includes(entity) {
			base.get(entity)
		} else {
			None
		}
	}

	fn contains(self, entity: Entity) -> bool {
		let (base, include, exclude, filter) = self.into_query_parts();

		filter.matches(entity)
			&& exclude.excludes(entity)
			&& include.includes(entity)
			&& base.contains(entity)
	}
}
