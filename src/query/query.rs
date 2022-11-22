use crate::query::{group_range, IntoQueryParts, Iter, QueryPart};
use crate::storage::Entity;

pub trait Query: IntoQueryParts + Sized {
    fn get<'a>(self, entity: Entity) -> Option<<Self::Get<'a> as QueryPart>::Refs<'a>>
    where
        Self: 'a;

    fn matches(self, entity: Entity) -> bool;

    /// Returns an iterator over all items in the query.
    fn iter<'a>(self) -> Iter<'a, Self::Get<'a>, Self::Include<'a>, Self::Exclude<'a>>
    where
        Self: 'a;

    /// Iterates over all items in the query. Equivalent to `.iter().for_each(|item| f(item))`.
    fn for_each<'a, F>(self, f: F)
    where
        Self: 'a,
        F: FnMut(<Self::Get<'a> as QueryPart>::Refs<'a>),
    {
        self.iter().for_each(f)
    }

    /// Iterates over all items in the query and the entities to which they belong. Equivalent to
    /// `.iter().with_entity().for_each(|(entity, item)| f((entity, item)))`.
    fn for_each_with_entity<'a, F>(self, f: F)
    where
        Self: 'a,
        F: FnMut((Entity, <Self::Get<'a> as QueryPart>::Refs<'a>)),
    {
        use crate::query::IntoEntityIter;
        self.iter().with_entity().for_each(f)
    }

    /// For grouped storages, returns all entities that match the query as a slice.
    fn into_entities<'a>(self) -> Option<&'a [Entity]>
    where
        Self: 'a;

    /// For grouped storages, returns all components that match the query as slices.
    fn into_components<'a>(self) -> Option<<Self::Get<'a> as QueryPart>::Slices<'a>>
    where
        Self: 'a;

    /// For grouped storages, returns all entities and components that match the query as slices.
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
