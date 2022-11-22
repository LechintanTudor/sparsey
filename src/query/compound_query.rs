use crate::query::QueryPart;

pub trait BuildCompoundQuery: QueryPart + Sized {
    fn include<I>(self, include: I) -> IncludeQuery<Self, I>
    where
        I: QueryPart;

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
        IncludeExcludeQuery { get: self, include: (), exclude }
    }
}

pub struct IncludeQuery<G, I> {
    pub get: G,
    pub include: I,
}

impl<G, I> IncludeQuery<G, I>
where
    G: QueryPart,
    I: QueryPart,
{
    pub fn exclude<E>(self, exclude: E) -> IncludeExcludeQuery<G, I, E>
    where
        E: QueryPart,
    {
        IncludeExcludeQuery { get: self.get, include: self.include, exclude }
    }
}

pub struct IncludeExcludeQuery<G, I, E> {
    pub get: G,
    pub include: I,
    pub exclude: E,
}
