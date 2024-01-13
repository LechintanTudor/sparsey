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

pub trait Query: IntoQueryParts {
    #[must_use]
    fn get<'a>(self, entity: Entity) -> Option<<Self::Get as QueryPart>::Refs<'a>>;

    #[must_use]
    fn matches(self, entity: Entity) -> bool;

    fn iter<'a>(self) -> Iter<'a, Self::Get, Self::Include, Self::Exclude>
    where
        Self: 'a;

    fn for_each<'a, F>(self, f: F)
    where
        Self: 'a,
        F: FnMut(<Self::Get as QueryPart>::Refs<'a>);

    fn for_each_with_entity<'a, F>(self, f: F)
    where
        Self: 'a,
        F: FnMut((Entity, <Self::Get as QueryPart>::Refs<'a>));
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
}
