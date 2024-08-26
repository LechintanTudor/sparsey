use crate::component::QueryGroupInfo;
use crate::entity::Entity;
use crate::query::{DenseIter, Iter, Query, SparseIter};
use crate::World;
use core::ops::Range;

#[must_use]
pub struct WorldQuery<'a, G, I, E>
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

impl<'a, G> WorldQuery<'a, G, (), ()>
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
    pub fn contains(&self, entity: Entity) -> bool {
        if !E::contains_none(&self.exclude, entity) {
            return false;
        }

        if !I::contains_all(&self.include, entity) {
            return false;
        }

        G::contains_all(&self.get, entity)
    }

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

    #[must_use]
    pub fn map<T, F>(&mut self, entity: Entity, f: F) -> Option<T>
    where
        F: FnOnce(G::Item<'_>) -> T,
    {
        self.get(entity).map(f)
    }
}

#[must_use]
pub struct WorldQueryAll<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    world: &'a World,
    get: G::View<'a>,
    include: I::View<'a>,
    exclude: E::View<'a>,
    get_info: Option<QueryGroupInfo>,
    include_info: Option<QueryGroupInfo>,
    exclude_info: Option<QueryGroupInfo>,
}

impl<'a, G> WorldQueryAll<'a, G, (), ()>
where
    G: Query,
{
    pub(crate) fn new(world: &'a World) -> Self {
        let (get, get_info) = G::borrow_with_group_info(world);

        Self {
            world,
            get,
            include: (),
            exclude: (),
            get_info,
            include_info: Some(QueryGroupInfo::Empty),
            exclude_info: Some(QueryGroupInfo::Empty),
        }
    }
}

impl<'a, G, I, E> WorldQueryAll<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    pub fn iter(&mut self) -> Iter<'_, G, I, E> {
        match self.get_group_range() {
            Some(range) => {
                let (get_entities, get_data) = G::split_dense_data(&self.get);
                let (include_entities, _) = I::split_sparse(&self.include);
                let entities = get_entities.or(include_entities).unwrap();
                unsafe { Iter::Dense(DenseIter::new(range, entities, get_data)) }
            }
            None => {
                let (get_entities, get_sparse, get_data) = G::split_sparse_data(&self.get);
                let (include_entities, include_sparse) = I::split_sparse(&self.include);
                let (_, exclude_sparse) = E::split_sparse(&self.exclude);

                let entities = match (get_entities, include_entities) {
                    (Some(get_entities), Some(include_entities)) => {
                        if get_entities.len() <= include_entities.len() {
                            get_entities
                        } else {
                            include_entities
                        }
                    }
                    (Some(get_entities), None) => get_entities,
                    (None, Some(include_entities)) => include_entities,
                    (None, None) => &[],
                };

                Iter::Sparse(SparseIter::new(
                    entities,
                    exclude_sparse,
                    include_sparse,
                    get_sparse,
                    get_data,
                ))
            }
        }
    }

    pub fn for_each<F>(&mut self, f: F)
    where
        F: FnMut(G::Item<'_>),
    {
        self.iter().for_each(f);
    }

    #[must_use]
    pub fn slice(&mut self) -> Option<G::Slice<'_>> {
        let range = self.get_group_range()?;
        let (get_entities, get_parts) = G::split_dense_data(&self.get);
        let (include_entities, _) = I::split_sparse(&self.include);
        let entities = get_entities.or(include_entities).unwrap_or(&[]);
        unsafe { Some(G::slice(get_parts, entities, range)) }
    }

    #[must_use]
    fn get_group_range(&self) -> Option<Range<usize>> {
        let get_info = self.get_info?;
        let include_info = self.include_info?;
        let exclude_info = self.exclude_info?;

        unsafe {
            self.world
                .components
                .group_range(&get_info.add_query(&include_info)?, &exclude_info)
        }
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
impl<'a, G, I, E> IntoIterator for &'a mut WorldQueryAll<'_, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    type Item = <Iter<'a, G, I, E> as Iterator>::Item;
    type IntoIter = Iter<'a, G, I, E>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
