use crate::entity::{Entity, World};
use crate::query::{Iter, Query, QueryGroupInfo};

#[must_use]
pub struct WorldQuery<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    pub(crate) world: &'a World,
    pub(crate) get: G::View<'a>,
    pub(crate) include: I::View<'a>,
    pub(crate) exclude: E::View<'a>,
}

impl<'a, G, E> WorldQuery<'a, G, (), E>
where
    G: Query,
    E: Query,
{
    pub fn include<I>(self) -> WorldQuery<'a, G, I, E>
    where
        I: Query,
    {
        WorldQuery {
            world: self.world,
            get: self.get,
            include: I::borrow(self.world),
            exclude: self.exclude,
        }
    }
}

impl<'a, G, I> WorldQuery<'a, G, I, ()>
where
    G: Query,
    I: Query,
{
    pub fn exclude<E>(self) -> WorldQuery<'a, G, I, E>
    where
        E: Query,
    {
        WorldQuery {
            world: self.world,
            get: self.get,
            include: self.include,
            exclude: E::borrow(self.world),
        }
    }
}

impl<'a, G, I, E> WorldQuery<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    #[must_use]
    pub fn get(&mut self, entity: Entity) -> Option<G::Item<'_>> {
        if !E::contains_none(&self.exclude, entity) {
            return None;
        }

        if !I::contains_all(&self.include, entity) {
            return None;
        }

        unsafe { G::get(&mut self.get, entity) }
    }

    #[must_use]
    pub fn contains(&mut self, entity: Entity) -> bool {
        if !E::contains_none(&self.exclude, entity) {
            return false;
        }

        if !I::contains_all(&self.include, entity) {
            return false;
        }

        G::contains_all(&self.get, entity)
    }
}

#[must_use]
pub struct WorldQueryAll<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    pub(crate) world: &'a World,
    pub(crate) get: G::View<'a>,
    pub(crate) include: I::View<'a>,
    pub(crate) exclude: E::View<'a>,
    pub(crate) get_info: Option<QueryGroupInfo>,
    pub(crate) include_info: Option<QueryGroupInfo>,
    pub(crate) exclude_info: Option<QueryGroupInfo>,
}

impl<'a, G, I, E> WorldQueryAll<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    pub fn iter(&mut self) -> Iter<'_, 'a, G, I, E> {
        Iter::new(self)
    }

    pub fn for_each<F>(&mut self, f: F)
    where
        F: FnMut(G::Item<'_>),
    {
        Iter::new(self).for_each(f)
    }
}

impl<'a, G, E> WorldQueryAll<'a, G, (), E>
where
    G: Query,
    E: Query,
{
    pub fn include<I>(self) -> WorldQueryAll<'a, G, I, E>
    where
        I: Query,
    {
        let (include, include_info) = I::borrow_with_group_info(self.world);

        WorldQueryAll {
            world: self.world,
            get: self.get,
            include,
            exclude: self.exclude,
            get_info: self.get_info,
            include_info,
            exclude_info: self.exclude_info,
        }
    }
}

impl<'a, G, I> WorldQueryAll<'a, G, I, ()>
where
    G: Query,
    I: Query,
{
    pub fn exclude<E>(self) -> WorldQueryAll<'a, G, I, E>
    where
        E: Query,
    {
        let (exclude, exclude_info) = E::borrow_with_group_info(self.world);

        WorldQueryAll {
            world: self.world,
            get: self.get,
            include: self.include,
            exclude,
            get_info: self.get_info,
            include_info: self.include_info,
            exclude_info,
        }
    }
}

#[allow(clippy::into_iter_without_iter)]
impl<'query, 'view, G, I, E> IntoIterator for &'query mut WorldQueryAll<'view, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    type Item = <Iter<'query, 'view, G, I, E> as Iterator>::Item;
    type IntoIter = Iter<'query, 'view, G, I, E>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}
