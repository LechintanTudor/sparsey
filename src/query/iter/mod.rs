mod dense_iter;
mod sparse_iter;

pub use self::dense_iter::*;
pub use self::sparse_iter::*;

use crate::query::Query;

pub enum Iter<'query, 'view, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    Sparse(SparseIter<'query, 'view, G, I, E>),
    Dense(DenseIter<'query, 'view, G>),
}

impl<'query, G, I, E> Iterator for Iter<'query, '_, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    type Item = G::Item<'query>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Sparse(iter) => iter.next(),
            Self::Dense(iter) => iter.next(),
        }
    }

    fn fold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        match self {
            Self::Sparse(iter) => iter.fold(init, f),
            Self::Dense(iter) => iter.fold(init, f),
        }
    }
}
