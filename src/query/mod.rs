//! Handles querying and iterating sets of components.

mod component_view;
mod compound_query;
mod into_query_parts;
mod iter;
mod query_group_info;
mod query_part;

pub use self::component_view::*;
pub use self::compound_query::*;
pub use self::into_query_parts::*;
pub use self::iter::*;
pub use self::query_group_info::*;
pub use self::query_part::*;

use crate::entity::Entity;

/// Trait for all queries that can be performed on component views.
pub trait Query: IntoQueryParts {
    /// Return the components mapped to `entity`, if `entity` matches the query.
    #[must_use]
    fn get<'a>(self, entity: Entity) -> Option<<Self::Get as QueryPart>::Refs<'a>>;

    /// Returns whether `entity` matches the query.
    #[must_use]
    fn matches(self, entity: Entity) -> bool;

    /// Returns an iterator over all components that match the query.
    fn iter<'a>(self) -> Iter<'a, Self::Get, Self::Include, Self::Exclude>
    where
        Self: 'a;

    /// Runs a function for each component set that matches the query.
    fn for_each<'a, F>(self, f: F)
    where
        Self: 'a,
        F: FnMut(<Self::Get as QueryPart>::Refs<'a>);

    /// Runs a function for each entity and component set that matches the query.
    fn for_each_with_entity<'a, F>(self, f: F)
    where
        Self: 'a,
        F: FnMut((Entity, <Self::Get as QueryPart>::Refs<'a>));

    /// Returns the entities that match the query, if the query is grouped.
    #[must_use]
    fn group_entities<'a>(self) -> Option<&'a [Entity]>
    where
        Self: 'a;

    /// Returns the components that match the query, if the query is grouped.
    #[must_use]
    fn group_components<'a>(self) -> Option<<Self::Get as QueryPart>::Slices<'a>>;

    /// Returns the entities and components that match the query, if the query is grouped.
    #[must_use]
    fn group_data<'a>(self) -> Option<(&'a [Entity], <Self::Get as QueryPart>::Slices<'a>)>
    where
        Self: 'a;
}

impl<Q> Query for Q
where
    Q: IntoQueryParts,
{
    fn get<'a>(self, entity: Entity) -> Option<<Self::Get as QueryPart>::Refs<'a>> {
        let (get, include, exclude) = self.into_query_parts();

        if !(include.contains_all(entity) && exclude.contains_none(entity)) {
            return None;
        }

        QueryPart::get(get, entity)
    }

    fn matches(self, entity: Entity) -> bool {
        let (get, include, exclude) = self.into_query_parts();
        get.contains_all(entity) && include.contains_all(entity) && exclude.contains_none(entity)
    }

    fn iter<'a>(self) -> Iter<'a, Self::Get, Self::Include, Self::Exclude>
    where
        Self: 'a,
    {
        let (get, include, exclude) = self.into_query_parts();
        Iter::new(get, include, exclude)
    }

    fn for_each<'a, F>(self, f: F)
    where
        Self: 'a,
        F: FnMut(<Self::Get as QueryPart>::Refs<'a>),
    {
        self.iter().for_each(f);
    }

    fn for_each_with_entity<'a, F>(self, f: F)
    where
        Self: 'a,
        F: FnMut((Entity, <Self::Get as QueryPart>::Refs<'a>)),
    {
        self.iter().with_entity().for_each(f);
    }

    fn group_entities<'a>(self) -> Option<&'a [Entity]>
    where
        Self: 'a,
    {
        let (get, include, exclude) = self.into_query_parts();
        let range = group_range(&get, &include, &exclude)?;

        let entities = unsafe {
            if Self::Get::HAS_DATA {
                get.get_entities_unchecked(range)
            } else {
                include.get_entities_unchecked(range)
            }
        };

        Some(entities)
    }

    fn group_components<'a>(self) -> Option<<Self::Get as QueryPart>::Slices<'a>> {
        let (get, include, exclude) = self.into_query_parts();
        let range = group_range(&get, &include, &exclude)?;
        unsafe { Some(get.get_components_unchecked(range)) }
    }

    fn group_data<'a>(self) -> Option<(&'a [Entity], <Self::Get as QueryPart>::Slices<'a>)>
    where
        Self: 'a,
    {
        let (get, include, exclude) = self.into_query_parts();
        let range = group_range(&get, &include, &exclude)?;

        let (entities, components) = unsafe {
            if Self::Get::HAS_DATA {
                get.get_data_unchecked(range)
            } else {
                let entities = include.get_entities_unchecked(range.clone());
                let (_, components) = get.get_data_unchecked(range);
                (entities, components)
            }
        };

        Some((entities, components))
    }
}
