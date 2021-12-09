use crate::query::{Iter, QueryFilter, QueryGet, QueryModifier};
use crate::storage::Entity;

/// Trait used for implementing a `Query`.
pub trait IntoQueryParts<'a> {
    /// Part of the `Query` that fetches components.
    type Get: QueryGet<'a>;
    /// Part of the `Query` that includes components.
    type Include: QueryModifier<'a>;
    /// Part of the `Query` that excludes components.
    type Exclude: QueryModifier<'a>;
    /// Part of the `Query` that filters entities.
    type Filter: QueryFilter;

    /// Splits the `Query` into its parts.
    fn into_query_parts(self) -> (Self::Get, Self::Include, Self::Exclude, Self::Filter);
}

/// Trait that enables fetching and iterating component views.
pub trait Query<'a>: IntoQueryParts<'a> {
    /// Item fetched by the `Query`.
    type Item: 'a;
    /// Iterator returned by the `Query`.
    type Iterator: Iterator<Item = Self::Item>;

    /// Returns `true` if `entity` matches the `Query`.
    fn contains(self, entity: Entity) -> bool;

    /// Returns the item mapped to `entity`, if any.
    fn get(self, entity: Entity) -> Option<Self::Item>;

    /// Returns an iterator over all items that match the `Query`.
    fn iter(self) -> Self::Iterator;
}

impl<'a, Q> Query<'a> for Q
where
    Q: IntoQueryParts<'a>,
{
    type Item = <Q::Get as QueryGet<'a>>::Item;
    type Iterator = Iter<'a, Q::Get, Q::Include, Q::Exclude, Q::Filter>;

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

    fn iter(self) -> Self::Iterator {
        let (get, include, exclude, filter) = self.into_query_parts();
        Iter::new(get, include, exclude, filter)
    }
}
