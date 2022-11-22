use crate::query::{IncludeExcludeQuery, IncludeQuery, QueryPart};

/// Helper trait for building a [`Query`](crate::query::Query).
pub trait IntoQueryParts {
    /// Which components to fetch.
    type Get<'a>: QueryPart
    where
        Self: 'a;

    /// Which components to include.
    type Include<'a>: QueryPart
    where
        Self: 'a;

    /// Which components to exclude.
    type Exclude<'a>: QueryPart
    where
        Self: 'a;

    /// Splits `self` into its underlying [`QueryParts`](QueryPart).
    fn into_query_parts<'a>(self) -> (Self::Get<'a>, Self::Include<'a>, Self::Exclude<'a>)
    where
        Self: 'a;
}

impl<Q> IntoQueryParts for Q
where
    Q: QueryPart,
{
    type Get<'a> = Q where Self: 'a;
    type Include<'a> = () where Self: 'a;
    type Exclude<'a> = () where Self: 'a;

    fn into_query_parts<'a>(self) -> (Self::Get<'a>, Self::Include<'a>, Self::Exclude<'a>)
    where
        Self: 'a,
    {
        (self, (), ())
    }
}

impl<G, I> IntoQueryParts for IncludeQuery<G, I>
where
    G: QueryPart,
    I: QueryPart,
{
    type Get<'a> = G where Self: 'a;
    type Include<'a> = I where Self: 'a;
    type Exclude<'a> = () where Self: 'a;

    fn into_query_parts<'a>(self) -> (Self::Get<'a>, Self::Include<'a>, Self::Exclude<'a>)
    where
        Self: 'a,
    {
        (self.get, self.include, ())
    }
}

impl<G, I, E> IntoQueryParts for IncludeExcludeQuery<G, I, E>
where
    G: QueryPart,
    I: QueryPart,
    E: QueryPart,
{
    type Get<'a> = G where Self: 'a;
    type Include<'a> = I where Self: 'a;
    type Exclude<'a> = E where Self: 'a;

    fn into_query_parts<'a>(self) -> (Self::Get<'a>, Self::Include<'a>, Self::Exclude<'a>)
    where
        Self: 'a,
    {
        (self.get, self.include, self.exclude)
    }
}
