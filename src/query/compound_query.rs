use crate::query::{Iter, NonEmptyQuery, Query};
use crate::storage::Entity;

pub trait IntoCompoundQueryParts<'a> {
    type Get: Query<'a>;
    type Include: Query<'a> + Copy;
    type Exclude: Query<'a> + Copy;

    fn into_compound_query_parts(self) -> (Self::Get, Self::Include, Self::Exclude);
}

pub trait CompoundQuery<'a>: IntoCompoundQueryParts<'a> {
    fn get(self, entity: Entity) -> Option<<Self::Get as Query<'a>>::Item>;

    fn contains(self, entity: Entity) -> bool;
}

impl<'a, Q> CompoundQuery<'a> for Q
where
    Q: IntoCompoundQueryParts<'a>,
{
    fn get(self, entity: Entity) -> Option<<Self::Get as Query<'a>>::Item> {
        let (get, include, exclude) = self.into_compound_query_parts();

        if exclude.excludes(entity) && include.includes(entity) {
            Query::get(get, entity)
        } else {
            None
        }
    }

    fn contains(self, entity: Entity) -> bool {
        let (get, include, exclude) = self.into_compound_query_parts();
        exclude.excludes(entity) && include.includes(entity) && get.includes(entity)
    }
}

pub trait IterableCompoundQuery<'a>: CompoundQuery<'a>
where
    Self::Get: NonEmptyQuery<'a>,
{
    fn iter(self) -> Iter<'a, Self::Get, Self::Include, Self::Exclude>;
}

impl<'a, Q> IterableCompoundQuery<'a> for Q
where
    Q: CompoundQuery<'a>,
    Q::Get: NonEmptyQuery<'a>,
{
    fn iter(self) -> Iter<'a, Self::Get, Self::Include, Self::Exclude> {
        let (get, include, exclude) = self.into_compound_query_parts();
        Iter::new(get, include, exclude)
    }
}

impl<'a, Q> IntoCompoundQueryParts<'a> for Q
where
    Q: Query<'a>,
{
    type Get = Q;
    type Include = ();
    type Exclude = ();

    fn into_compound_query_parts(self) -> (Self::Get, Self::Include, Self::Exclude) {
        (self, (), ())
    }
}

pub struct Include<G, I> {
    get: G,
    include: I,
}

impl<'a, G, I> Include<G, I>
where
    G: Query<'a>,
    I: Query<'a> + Copy,
{
    pub(crate) fn new(get: G, include: I) -> Self {
        Self { get, include }
    }

    pub fn exclude<E>(self, exclude: E) -> IncludeExclude<G, I, E>
    where
        E: Query<'a> + Copy,
    {
        IncludeExclude::new(self.get, self.include, exclude)
    }
}

impl<'a, G, I> IntoCompoundQueryParts<'a> for Include<G, I>
where
    G: Query<'a>,
    I: Query<'a> + Copy,
{
    type Get = G;
    type Include = I;
    type Exclude = ();

    fn into_compound_query_parts(self) -> (Self::Get, Self::Include, Self::Exclude) {
        (self.get, self.include, ())
    }
}

pub struct IncludeExclude<G, I, E> {
    get: G,
    include: I,
    exclude: E,
}

impl<'a, G, I, E> IncludeExclude<G, I, E>
where
    G: Query<'a>,
    I: Query<'a>,
    E: Query<'a>,
{
    pub(crate) fn new(get: G, include: I, exclude: E) -> Self {
        Self { get, include, exclude }
    }
}

impl<'a, G, I, E> IntoCompoundQueryParts<'a> for IncludeExclude<G, I, E>
where
    G: Query<'a>,
    I: Query<'a> + Copy,
    E: Query<'a> + Copy,
{
    type Get = G;
    type Include = I;
    type Exclude = E;

    fn into_compound_query_parts(self) -> (Self::Get, Self::Include, Self::Exclude) {
        (self.get, self.include, self.exclude)
    }
}

pub trait QueryFilters<'a>: Query<'a> + Sized {
    fn include<I>(self, include: I) -> Include<Self, I>
    where
        I: Query<'a> + Copy;

    fn exclude<E>(self, exclude: E) -> IncludeExclude<Self, (), E>
    where
        E: Query<'a> + Copy;
}

impl<'a, Q> QueryFilters<'a> for Q
where
    Q: Query<'a>,
{
    fn include<I>(self, include: I) -> Include<Self, I>
    where
        I: Query<'a> + Copy,
    {
        Include::new(self, include)
    }

    fn exclude<E>(self, exclude: E) -> IncludeExclude<Self, (), E>
    where
        E: Query<'a> + Copy,
    {
        IncludeExclude::new(self, (), exclude)
    }
}
