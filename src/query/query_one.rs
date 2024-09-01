use crate::entity::Entity;
use crate::query::Query;
use crate::World;

/// Queries one item that matches the query.
#[must_use]
pub struct QueryOne<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    world: &'a World,
    get: G::View<'a>,
    include: I::View<'a>,
    exclude: E::View<'a>,
}

impl<'a, G> QueryOne<'a, G, (), ()>
where
    G: Query,
{
    pub(crate) fn new(world: &'a World) -> Self {
        Self {
            world,
            get: G::borrow(world),
            include: (),
            exclude: (),
        }
    }
}

impl<'a, G, E> QueryOne<'a, G, (), E>
where
    G: Query,
    E: Query,
{
    /// Applies an "include" filter to the query.
    pub fn include<I>(self) -> QueryOne<'a, G, I, E>
    where
        I: Query,
    {
        QueryOne {
            world: self.world,
            get: self.get,
            include: I::borrow(self.world),
            exclude: self.exclude,
        }
    }
}

impl<'a, G, I> QueryOne<'a, G, I, ()>
where
    G: Query,
    I: Query,
{
    /// Applies an "exclude" filter to the query.
    pub fn exclude<E>(self) -> QueryOne<'a, G, I, E>
    where
        E: Query,
    {
        QueryOne {
            world: self.world,
            get: self.get,
            include: self.include,
            exclude: E::borrow(self.world),
        }
    }
}

impl<'a, G, I, E> QueryOne<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    /// Returns whether `entity` matches the query.
    #[must_use]
    pub fn contains(&self, entity: Entity) -> bool {
        if !E::contains_none(&self.exclude, entity) {
            return false;
        }

        if !I::contains_all(&self.include, entity) {
            return false;
        }

        G::contains_all(&self.get, entity)
    }

    /// Returns the item mapped to `entity`, if any.
    #[must_use]
    pub fn get(&mut self, entity: Entity) -> Option<G::Item<'_>> {
        if !E::contains_none(&self.exclude, entity) {
            return None;
        }

        if !I::contains_all(&self.include, entity) {
            return None;
        }

        G::get(&mut self.get, entity)
    }

    /// Calls `f` for the item mapped to `entity`, if any.
    #[must_use]
    pub fn map<T, F>(&mut self, entity: Entity, f: F) -> Option<T>
    where
        F: FnOnce(G::Item<'_>) -> T,
    {
        self.get(entity).map(f)
    }
}
