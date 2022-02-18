use crate::query::{group_range, Iter, Query};
use crate::storage::Entity;

#[doc(hidden)]
pub trait IntoCompoundQueryParts<'a> {
    type Get: Query<'a>;
    type Include: Query<'a> + Copy;
    type Exclude: Query<'a> + Copy;

    fn into_compound_query_parts(self) -> (Self::Get, Self::Include, Self::Exclude);
}

/// Trait used for fetching and iterating entities and components from component views.
pub trait CompoundQuery<'a>: IntoCompoundQueryParts<'a> + Sized {
    /// Returns the component set associated to `entity`.
    fn get(self, entity: Entity) -> Option<<Self::Get as Query<'a>>::Item>;

    /// Returns `true` if `entity` matches the query.
    fn contains(self, entity: Entity) -> bool;

    /// Returns an iterator over all items in the query.
    fn iter(self) -> Iter<'a, Self::Get, Self::Include, Self::Exclude>;

    /// Iterates over all items in the query. Equivalent to `.iter().for_each(|item| f(item))`.
    fn for_each<F>(self, f: F)
    where
        F: FnMut(<Self::Get as Query<'a>>::Item),
    {
        self.iter().for_each(f)
    }

    /// Iterates over all items in the query and the entities to which they belong. Equivalent to
    /// `.iter().with_entity().for_each(|(entity, item)| f((entity, item)))`.
    fn for_each_with_entity<F>(self, f: F)
    where
        F: FnMut((Entity, <Self::Get as Query<'a>>::Item)),
    {
        use crate::query::iter::IntoEntityIter;

        self.iter().with_entity().for_each(f)
    }

    /// For grouped storages, returns all entities that match the query as a slice.
    #[allow(clippy::wrong_self_convention)]
    fn as_entity_slice(self) -> Option<&'a [Entity]>;

    /// For grouped storages, returns all components that match the query as slices.
    #[allow(clippy::wrong_self_convention)]
    fn as_component_slices(self) -> Option<<Self::Get as Query<'a>>::ComponentSlices>;

    /// For grouped storages, returns all entities and components that match the query as slices.
    #[allow(clippy::wrong_self_convention)]
    fn as_entity_and_component_slices(
        self,
    ) -> Option<(&'a [Entity], <Self::Get as Query<'a>>::ComponentSlices)>;
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

    fn iter(self) -> Iter<'a, Self::Get, Self::Include, Self::Exclude> {
        let (get, include, exclude) = self.into_compound_query_parts();
        Iter::new(get, include, exclude)
    }

    fn as_entity_slice(self) -> Option<&'a [Entity]> {
        let (get, include, exclude) = self.into_compound_query_parts();
        let range = group_range(get.group_info(), include.group_info(), exclude.group_info())?;
        unsafe { Some(get.get_entities_unchecked(range)) }
    }

    fn as_component_slices(self) -> Option<<Self::Get as Query<'a>>::ComponentSlices> {
        let (get, include, exclude) = self.into_compound_query_parts();
        let range = group_range(get.group_info(), include.group_info(), exclude.group_info())?;
        unsafe { Some(get.get_components_unchecked(range)) }
    }

    fn as_entity_and_component_slices(
        self,
    ) -> Option<(&'a [Entity], <Self::Get as Query<'a>>::ComponentSlices)> {
        let (get, include, exclude) = self.into_compound_query_parts();
        let range = group_range(get.group_info(), include.group_info(), exclude.group_info())?;
        unsafe { Some(get.get_entities_components_unchecked(range)) }
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

/// Wrapper over a query that applies an "include" filter.
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

    /// Applies an "exclude" filter to the query.
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

/// Wrapper over a query that applies "include" and "exclude" filters.
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

/// Helper trait for applying "include" and "exclude" filters to queries.
pub trait QueryFilters<'a>: Query<'a> + Sized {
    /// Applies an "include" filter to the query.
    fn include<I>(self, include: I) -> Include<Self, I>
    where
        I: Query<'a> + Copy;

    /// Applies an "exclude" filter to the query.
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
