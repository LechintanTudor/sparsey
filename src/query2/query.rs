use crate::query2::QueryFilter;
use crate::storage::Entity;

pub trait IntoQueryParts {
	type Fetch;
	type Include;
	type Exclude;
	type Filter: QueryFilter;

	fn into_query_parts(self) -> (Self::Fetch, Self::Include, Self::Exclude, Self::Filter);
}

pub trait Query
where
	Self: IntoQueryParts,
{
	type Item;

	fn get(self, entity: Entity) -> Option<Self::Item>;

	fn contains(&self, entity: Entity) -> bool;
}
