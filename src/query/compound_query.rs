use crate::query::QueryPart;

/// Helper trait for applying filters to a simple [`Query`](crate::query::Query).
pub trait BuildCompoundQuery: QueryPart + Sized {
    /// Applies an include filter to the query.
    fn include<I>(self, include: I) -> IncludeQuery<Self, I>
    where
        I: QueryPart;

    /// Applies an exclude filter to the query.
    fn exclude<E>(self, exclude: E) -> IncludeExcludeQuery<Self, (), E>
    where
        E: QueryPart;
}

impl<Q> BuildCompoundQuery for Q
where
    Q: QueryPart + Sized,
{
    fn include<I>(self, include: I) -> IncludeQuery<Self, I>
    where
        I: QueryPart,
    {
        IncludeQuery { get: self, include }
    }

    fn exclude<E>(self, exclude: E) -> IncludeExcludeQuery<Self, (), E>
    where
        E: QueryPart,
    {
        IncludeExcludeQuery {
            get: self,
            include: (),
            exclude,
        }
    }
}

/// [`Query`](crate::query::Query) with an include filter.
pub struct IncludeQuery<G, I> {
    /// Which components to fetch.
    pub get: G,
    /// Which components to include.
    pub include: I,
}

impl<G, I> IncludeQuery<G, I>
where
    G: QueryPart,
    I: QueryPart,
{
    /// Applies an exclude filter to the query.
    pub fn exclude<E>(self, exclude: E) -> IncludeExcludeQuery<G, I, E>
    where
        E: QueryPart,
    {
        IncludeExcludeQuery {
            get: self.get,
            include: self.include,
            exclude,
        }
    }
}

/// [`Query`](crate::query::Query) with an include filter and an exclude filter.
pub struct IncludeExcludeQuery<G, I, E> {
    /// Which components to fetch.
    pub get: G,
    /// Which components to include.
    pub include: I,
    /// Which components to exclude.
    pub exclude: E,
}
