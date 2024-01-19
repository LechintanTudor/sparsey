use crate::query::{IntoQueryParts, QueryPart};

/// Helper trait for building compound queries.
pub trait BuildCompoundQuery: Sized {
    /// Applies an "include filter" to the initial query.
    fn include<I>(self, include: I) -> IncludeQuery<Self, I>
    where
        I: QueryPart;

    /// Applies an "exclude filter" to the initial query.
    fn exclude<E>(self, exclude: E) -> IncludeExcludeQuery<Self, (), E>
    where
        E: QueryPart;
}

impl<Q> BuildCompoundQuery for Q
where
    Q: QueryPart,
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

/// Compound query that applies an "include filter" to the results.
#[must_use]
pub struct IncludeQuery<G, I> {
    /// The component views from which to return components.
    pub get: G,
    /// The component view that act as an "include filter".
    pub include: I,
}

impl<G, I> IncludeQuery<G, I> {
    /// Applies an "exclude filter" to the query.
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

impl<G, I> IntoQueryParts for IncludeQuery<G, I>
where
    G: QueryPart,
    I: QueryPart,
{
    type Get = G;
    type Include = I;
    type Exclude = ();

    fn into_query_parts(self) -> (Self::Get, Self::Include, Self::Exclude) {
        (self.get, self.include, ())
    }
}

/// Compound query that applies an "include filter" and an "exclude filter" to the results.
#[must_use]
pub struct IncludeExcludeQuery<G, I, E> {
    /// The component views from which to return components.
    pub get: G,
    /// The component view that act as an "include filter".
    pub include: I,
    /// The component view that act as an "exclude filter".
    pub exclude: E,
}

impl<G, I, E> IntoQueryParts for IncludeExcludeQuery<G, I, E>
where
    G: QueryPart,
    I: QueryPart,
    E: QueryPart,
{
    type Get = G;
    type Include = I;
    type Exclude = E;

    fn into_query_parts(self) -> (Self::Get, Self::Include, Self::Exclude) {
        (self.get, self.include, self.exclude)
    }
}
