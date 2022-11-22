use crate::query::{group_range, IntoQueryParts, Iter, QueryPart};
use crate::storage::Entity;

/// Allows fetching and iterating entities and components in
/// [`ComponentViews`](crate::query::ComponentView).
pub trait Query: IntoQueryParts + Sized {
    /// Returns the set of components mapped to `entity` if `entity` matches the query.
    fn get<'a>(self, entity: Entity) -> Option<<Self::Get<'a> as QueryPart>::Refs<'a>>
    where
        Self: 'a;

    /// Returns whether `entity` matches the query.
    fn matches(self, entity: Entity) -> bool;

    /// Returns an iterator over all components mapped to entities that match in the query.
    fn iter<'a>(self) -> Iter<'a, Self::Get<'a>, Self::Include<'a>, Self::Exclude<'a>>
    where
        Self: 'a;

    /// Iterates over all components mapped to entities that match the query. Equivalent to
    /// `.iter().for_each(|item| f(item))`.
    fn for_each<'a, F>(self, f: F)
    where
        Self: 'a,
        F: FnMut(<Self::Get<'a> as QueryPart>::Refs<'a>),
    {
        self.iter().for_each(f)
    }

    /// Iterates over all entities that match the query and their associated components. Equivalent
    /// to `.iter().with_entity().for_each(|(entity, item)| f((entity, item)))`.
    fn for_each_with_entity<'a, F>(self, f: F)
    where
        Self: 'a,
        F: FnMut((Entity, <Self::Get<'a> as QueryPart>::Refs<'a>)),
    {
        use crate::query::IntoEntityIter;
        self.iter().with_entity().for_each(f)
    }

    /// For grouped storages, returns a slice of all entities that match the query.
    fn into_entities<'a>(self) -> Option<&'a [Entity]>
    where
        Self: 'a;

    /// For grouped storages, returns ordered slices of all components mapped to entities that match
    /// the query.
    fn into_components<'a>(self) -> Option<<Self::Get<'a> as QueryPart>::Slices<'a>>
    where
        Self: 'a;

    /// For grouped storages, returns all entities that match query and their associated components
    /// as ordered slices.
    fn into_entities_and_components<'a>(
        self,
    ) -> Option<(&'a [Entity], <Self::Get<'a> as QueryPart>::Slices<'a>)>
    where
        Self: 'a;
}

impl<Q> Query for Q
where
    Q: IntoQueryParts,
{
    fn get<'a>(self, entity: Entity) -> Option<<Self::Get<'a> as QueryPart>::Refs<'a>>
    where
        Self: 'a,
    {
        let (get, include, exclude) = IntoQueryParts::into_query_parts(self);

        if QueryPart::contains_none(exclude, entity) && QueryPart::contains_all(include, entity) {
            QueryPart::get(get, entity)
        } else {
            None
        }
    }

    fn matches<'a>(self, entity: Entity) -> bool {
        let (get, include, exclude) = IntoQueryParts::into_query_parts(self);

        QueryPart::contains_none(exclude, entity)
            && QueryPart::contains_all(include, entity)
            && QueryPart::contains_all(get, entity)
    }

    fn iter<'a>(self) -> Iter<'a, Self::Get<'a>, Self::Include<'a>, Self::Exclude<'a>>
    where
        Self: 'a,
    {
        let (get, include, exclude) = self.into_query_parts();
        Iter::new(get, include, exclude)
    }

    fn into_entities<'a>(self) -> Option<&'a [Entity]>
    where
        Self: 'a,
    {
        let (get, include, exclude) = self.into_query_parts();
        let range = group_range(get.group_info(), include.group_info(), exclude.group_info())?;
        unsafe { Some(get.get_entities_unchecked(range)) }
    }

    fn into_components<'a>(self) -> Option<<Self::Get<'a> as QueryPart>::Slices<'a>>
    where
        Self: 'a,
    {
        let (get, include, exclude) = self.into_query_parts();
        let range = group_range(get.group_info(), include.group_info(), exclude.group_info())?;
        unsafe { Some(get.get_components_unchecked(range)) }
    }

    fn into_entities_and_components<'a>(
        self,
    ) -> Option<(&'a [Entity], <Self::Get<'a> as QueryPart>::Slices<'a>)>
    where
        Self: 'a,
    {
        let (get, include, exclude) = self.into_query_parts();
        let range = group_range(get.group_info(), include.group_info(), exclude.group_info())?;
        unsafe { Some(get.get_entities_and_components_unchecked(range)) }
    }
}
