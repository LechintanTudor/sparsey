use crate::query::{DenseIter, Iter, Query, QueryGroupInfo, SparseIter};
use crate::World;
use core::ops::Range;

#[must_use]
pub struct QueryAll<'a, G, I, E>
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

impl<'a, G> QueryAll<'a, G, (), ()>
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

impl<'a, G, I, E> QueryAll<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    pub fn iter(&mut self) -> Iter<'_, G, I, E> {
        if let Some(range) = self.get_group_range() {
            let (get_entities, get_data) = G::split_dense_data(&self.get);
            let (include_entities, _) = I::split_sparse(&self.include);
            let entities = get_entities.or(include_entities).unwrap();
            unsafe { Iter::Dense(DenseIter::new(range, entities, get_data)) }
        } else {
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

    pub fn for_each<F>(&mut self, f: F)
    where
        F: FnMut(G::Item<'_>),
    {
        self.iter().for_each(f);
    }

    #[cfg(feature = "parallel")]
    pub fn par_iter(&mut self) -> ParIter<'_, G, I, E> {
        if let Some(range) = self.get_group_range() {
            let (get_entities, get_data) = G::split_dense_data(&self.get);
            let (include_entities, _) = I::split_sparse(&self.include);
            let entities = get_entities.or(include_entities).unwrap();
            unsafe { ParIter::Dense(DenseParIter::new(range, entities, get_data)) }
        } else {
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

            ParIter::Sparse(SparseParIter::new(
                entities,
                exclude_sparse,
                include_sparse,
                get_sparse,
                get_data,
            ))
        }
    }

    #[cfg(feature = "parallel")]
    pub fn par_for_each<F>(&mut self, f: F)
    where
        for<'b> G::Item<'b>: Send,
        F: Fn(G::Item<'_>) + Send + Sync,
    {
        self.par_iter().for_each(f);
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

impl<'a, G, E> QueryAll<'a, G, (), E>
where
    G: Query,
    E: Query,
{
    pub fn include<I>(self) -> QueryAll<'a, G, I, E>
    where
        I: Query,
    {
        let (include, include_info) = I::borrow_with_group_info(self.world);

        QueryAll {
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

impl<'a, G, I> QueryAll<'a, G, I, ()>
where
    G: Query,
    I: Query,
{
    pub fn exclude<E>(self) -> QueryAll<'a, G, I, E>
    where
        E: Query,
    {
        let (exclude, exclude_info) = E::borrow_with_group_info(self.world);

        QueryAll {
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
impl<'a, G, I, E> IntoIterator for &'a mut QueryAll<'_, G, I, E>
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
