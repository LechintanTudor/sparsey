use crate::query::iter::{DenseIter, SparseIter};
use crate::query::{self, EntityIterator, Query};
use crate::storage::Entity;

/// Iterator over grouped or ungrouped storages.
pub enum Iter<'a, G, I, E>
where
    G: Query<'a>,
    I: Query<'a>,
    E: Query<'a>,
{
    /// Iterator over ungrouped storages.
    Sparse(SparseIter<'a, G, I, E>),
    /// Iterator over grouped storages. Extremely fast.
    Dense(DenseIter<'a, G>),
}

impl<'a, G, I, E> Iter<'a, G, I, E>
where
    G: Query<'a>,
    I: Query<'a>,
    E: Query<'a>,
{
    pub(crate) fn new(get: G, include: I, exclude: E) -> Self {
        let get_info = get.group_info();
        let include_info = include.group_info();
        let exclude_info = exclude.group_info();

        match query::group_range(get_info, include_info, exclude_info) {
            Some(range) => {
                let (entities, components) = get.split_dense();

                unsafe {
                    let components = G::offset_component_ptrs(components, range.start as isize);

                    let entities = &entities
                        .unwrap_or_else(|| {
                            include.into_any_entities().expect("Cannot iterate empty Query")
                        })
                        .get_unchecked(range);

                    Self::Dense(DenseIter::new(entities, components))
                }
            }
            None => {
                let (get_entities, sparse, components) = get.split_sparse();
                let (sparse_entities, include) = include.split_filter();
                let (_, exclude) = exclude.split_filter();

                let entities = match (get_entities, sparse_entities) {
                    (Some(e1), Some(e2)) => {
                        if e1.len() <= e2.len() {
                            e1
                        } else {
                            e2
                        }
                    }
                    (Some(e1), None) => e1,
                    (None, Some(e2)) => e2,
                    (None, None) => panic!("Cannot iterate empty Query"),
                };

                unsafe {
                    Self::Sparse(SparseIter::new(entities, sparse, include, exclude, components))
                }
            }
        }
    }

    /// Returns `true` if the iterator is sparse.
    pub fn is_sparse(&self) -> bool {
        matches!(self, Self::Sparse(_))
    }

    /// Returns `true` if the iterator is dense.
    pub fn is_dense(&self) -> bool {
        matches!(self, Self::Dense(_))
    }
}

impl<'a, G, I, E> Iterator for Iter<'a, G, I, E>
where
    G: Query<'a>,
    I: Query<'a>,
    E: Query<'a>,
{
    type Item = G::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Sparse(sparse) => sparse.next(),
            Self::Dense(dense) => dense.next(),
        }
    }

    fn fold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        match self {
            Self::Sparse(sparse) => sparse.fold(init, f),
            Self::Dense(dense) => dense.fold(init, f),
        }
    }
}

impl<'a, G, I, E> EntityIterator for Iter<'a, G, I, E>
where
    G: Query<'a>,
    I: Query<'a>,
    E: Query<'a>,
{
    fn next_with_entity(&mut self) -> Option<(Entity, Self::Item)> {
        match self {
            Self::Sparse(sparse) => sparse.next_with_entity(),
            Self::Dense(dense) => dense.next_with_entity(),
        }
    }

    fn fold_with_entity<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, (Entity, Self::Item)) -> B,
    {
        match self {
            Self::Sparse(sparse) => sparse.fold_with_entity(init, f),
            Self::Dense(dense) => dense.fold_with_entity(init, f),
        }
    }
}
