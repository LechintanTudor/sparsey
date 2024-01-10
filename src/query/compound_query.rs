use crate::query::{IntoQueryParts, QueryPart};

pub trait BuildCompoundQuery: Sized {
    fn include<I>(self, include: I) -> IncludeQuery<Self, I>
    where
        I: QueryPart;

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

#[must_use]
pub struct IncludeQuery<G, I> {
    pub get: G,
    pub include: I,
}

impl<G, I> IncludeQuery<G, I> {
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

#[must_use]
pub struct IncludeExcludeQuery<G, I, E> {
    pub get: G,
    pub include: I,
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
