mod dense_iter;
mod sparse_iter;

pub use self::dense_iter::*;
pub use self::sparse_iter::*;

use crate::query::Query;

/// Sparse or dense iterator over all items that match the query.
#[must_use]
pub enum Iter<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    /// Sparse iterator.
    Sparse(SparseIter<'a, G, I, E>),
    /// Dense iterator. Extremely fast.
    Dense(DenseIter<'a, G>),
}

impl<G, I, E> Iter<'_, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
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
    G: Query,
    I: Query,
    E: Query,
{
    type Item = G::Item<'a>;

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

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::Sparse(iter) => iter.size_hint(),
            Self::Dense(iter) => iter.size_hint(),
        }
    }
}

impl<'a, G, I, E> From<SparseIter<'a, G, I, E>> for Iter<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    fn from(iter: SparseIter<'a, G, I, E>) -> Self {
        Self::Sparse(iter)
    }
}

impl<'a, G, I, E> From<DenseIter<'a, G>> for Iter<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    fn from(iter: DenseIter<'a, G>) -> Self {
        Self::Dense(iter)
    }
}
