mod dense_iter;
mod entity_iter;
mod sparse_iter;

pub use self::dense_iter::*;
pub use self::entity_iter::*;
pub use self::sparse_iter::*;

use crate::entity::Entity;
use crate::query::{group_range, QueryPart};

/// Iterator over all components that match a query.
pub enum Iter<'a, G, I, E>
where
    G: QueryPart,
    I: QueryPart,
    E: QueryPart,
{
    /// Iterator over ungrouped queries.
    Sparse(SparseIter<'a, G, I, E>),
    /// Iterator over grouped queries. Very fast.
    Dense(DenseIter<'a, G>),
}

impl<'a, G, I, E> Iter<'a, G, I, E>
where
    G: QueryPart + 'a,
    I: QueryPart + 'a,
    E: QueryPart + 'a,
{
    pub(crate) fn new(get: G, include: I, exclude: E) -> Self {
        if let Some(range) = group_range(&get, &include, &exclude) {
            let (entities, ptrs) = get.split_dense();

            let entities = if G::HAS_DATA {
                entities
            } else {
                debug_assert!(I::HAS_DATA);
                let (entities, _, _) = include.split_sparse();
                entities
            };

            unsafe {
                let ptrs = G::add_to_ptrs(ptrs, range.start);
                let entities = entities.get_unchecked(range);
                Self::Dense(DenseIter::new(entities, ptrs))
            }
        } else {
            let (get_entities, sparse, ptrs) = get.split_sparse();
            let (sparse_entities, include) = include.split_filter();
            let (_, exclude) = exclude.split_filter();

            let entities = match (G::HAS_DATA, I::HAS_DATA) {
                (true, false) => get_entities,
                (false, true) => sparse_entities,
                (true, true) => {
                    if get_entities.len() >= sparse_entities.len() {
                        get_entities
                    } else {
                        sparse_entities
                    }
                }
                (false, false) => panic!("Cannot iterate over an empty Query"),
            };

            unsafe { Self::Sparse(SparseIter::new(entities, sparse, include, exclude, ptrs)) }
        }
    }

    /// Returns whether the iterator is sparse.
    #[must_use]
    pub const fn is_sparse(&self) -> bool {
        matches!(self, Self::Sparse(_))
    }

    /// Returns whether the iterator is dense.
    #[must_use]
    pub const fn is_dense(&self) -> bool {
        matches!(self, Self::Dense(_))
    }
}

impl<'a, G, I, E> Iterator for Iter<'a, G, I, E>
where
    G: QueryPart + 'a,
    I: QueryPart,
    E: QueryPart,
{
    type Item = G::Refs<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Sparse(it) => it.next(),
            Self::Dense(it) => it.next(),
        }
    }

    fn fold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        match self {
            Self::Sparse(it) => it.fold(init, f),
            Self::Dense(it) => it.fold(init, f),
        }
    }
}

impl<'a, G, I, E> EntityIterator for Iter<'a, G, I, E>
where
    G: QueryPart + 'a,
    I: QueryPart,
    E: QueryPart,
{
    fn next_with_entity(&mut self) -> Option<(Entity, Self::Item)> {
        match self {
            Self::Sparse(it) => it.next_with_entity(),
            Self::Dense(it) => it.next_with_entity(),
        }
    }

    fn fold_with_entity<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, (Entity, Self::Item)) -> B,
    {
        match self {
            Self::Sparse(it) => it.fold_with_entity(init, f),
            Self::Dense(it) => it.fold_with_entity(init, f),
        }
    }
}
