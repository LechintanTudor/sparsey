mod dense_iter;
mod sparse_iter;

pub use self::dense_iter::*;
pub use self::sparse_iter::*;

use crate::query::Query;

#[must_use]
pub enum Iter<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    Sparse(SparseIter<'a, G, I, E>),
    Dense(DenseIter<'a, G>),
}

impl<G, I, E> Iter<'_, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    #[must_use]
    pub const fn is_sparse(&self) -> bool {
        matches!(self, Self::Sparse(_))
    }

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
