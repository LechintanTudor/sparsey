use crate::query::{Iter, QueryBase, QueryFilter, QueryModifier};
use crate::storage::Entity;

/// Trait used for splitting a query into its parts.
pub trait IntoQueryParts<'a> {
    type Base: QueryBase<'a>;
    type Include: QueryModifier<'a>;
    type Exclude: QueryModifier<'a>;
    type Filter: QueryFilter;

    /// Splits a query into its parts.
    fn into_query_parts(self) -> (Self::Base, Self::Include, Self::Exclude, Self::Filter);
}

/// Trait implemented by queries.
pub trait Query<'a>
where
    Self: IntoQueryParts<'a>,
{
    /// Item returned by the query.
    type Item;
    /// Iterator over all items which match the query.
    type Iterator: Iterator<Item = Self::Item>;

    /// Returns the item at `entity` if it matches the query.
    fn get(self, entity: Entity) -> Option<Self::Item>;

    /// Returns `true` if `entity` matches the query.
    fn contains(self, entity: Entity) -> bool;

    /// Returns an iterator over all items which match the query.
    fn iter(self) -> Self::Iterator;
}

impl<'a, Q> Query<'a> for Q
where
    Q: IntoQueryParts<'a>,
{
    type Item = <Q::Base as QueryBase<'a>>::Item;
    type Iterator = Iter<'a, Q::Base, Q::Include, Q::Exclude, Q::Filter>;

    fn get(self, entity: Entity) -> Option<Self::Item> {
        let (base, include, exclude, filter) = self.into_query_parts();

        if QueryFilter::matches(&filter, entity)
            && QueryModifier::excludes(&exclude, entity)
            && QueryModifier::includes(&include, entity)
        {
            QueryBase::get(base, entity)
        } else {
            None
        }
    }

    fn contains(self, entity: Entity) -> bool {
        let (base, include, exclude, filter) = self.into_query_parts();

        QueryFilter::matches(&filter, entity)
            && QueryModifier::excludes(&exclude, entity)
            && QueryModifier::includes(&include, entity)
            && QueryBase::contains(&base, entity)
    }

    fn iter(self) -> Self::Iterator {
        let (base, include, exclude, filter) = self.into_query_parts();
        Iter::new(base, include, exclude, filter)
    }
}
