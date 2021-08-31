use crate::query2::{QueryBase, QueryFilter};
use crate::storage::Entity;

pub trait IntoQueryParts<'a> {
	type Base: QueryBase<'a>;
	type Include;
	type Exclude;
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
		let (base, _, _, _) = self.into_query_parts();
		base.get(entity)
	}

	fn contains(self, entity: Entity) -> bool {
		let (base, _, _, _) = self.into_query_parts();
		base.contains(entity)
	}
}
