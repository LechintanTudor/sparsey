use crate::query::{QueryPart, IntoQueryParts};
use crate::storage::Entity;

pub trait Query: IntoQueryParts + Sized {
    fn get<'a>(self, entity: Entity) -> Option<<Self::Get<'a> as QueryPart>::Refs<'a>>
    where
        Self: 'a;
    
    fn matches(self, entity: Entity) -> bool;
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
}
