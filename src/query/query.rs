use crate::query::{QueryFilter, QueryGet, QueryModifier};
use crate::storage::Entity;

pub trait IntoQueryParts<'a> {
    type Get: QueryGet<'a>;
    type Include: QueryModifier<'a>;
    type Exclude: QueryModifier<'a>;
    type Filter: QueryFilter;

    fn into_query_parts(self) -> (Self::Get, Self::Include, Self::Exclude, Self::Filter);
}

pub trait Query<'a>: IntoQueryParts<'a> {
    type Item: 'a;

    fn contains(self, entity: Entity) -> bool;

    fn get(self, entity: Entity) -> Option<Self::Item>;
}

impl<'a, Q> Query<'a> for Q
where
    Q: IntoQueryParts<'a>,
{
    type Item = <Q::Get as QueryGet<'a>>::Item;

    fn contains(self, entity: Entity) -> bool {
        let (get, include, exclude, filter) = self.into_query_parts();

        QueryFilter::matches(&filter, entity)
            && QueryModifier::excludes(&exclude, entity)
            && QueryModifier::includes(&include, entity)
            && QueryGet::contains(&get, entity)
    }

    fn get(self, entity: Entity) -> Option<Self::Item> {
        let (get, include, exclude, filter) = self.into_query_parts();

        let matches = QueryFilter::matches(&filter, entity)
            && QueryModifier::excludes(&exclude, entity)
            && QueryModifier::includes(&include, entity);

        if matches {
            QueryGet::get(get, entity)
        } else {
            None
        }
    }
}
