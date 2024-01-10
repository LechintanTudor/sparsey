mod dense_iter;
mod sparse_iter;

pub use self::dense_iter::*;
pub use self::sparse_iter::*;

use crate::query::QueryPart;

pub enum Iter<'a, G, I, E>
where
    G: QueryPart,
    I: QueryPart,
    E: QueryPart,
{
    Sparse(SparseIter<'a, G, I, E>),
    Dense(DenseIter<'a, G>),
}

impl<'a, G, I, E> Iter<'a, G, I, E>
where
    G: QueryPart,
    I: QueryPart,
    E: QueryPart,
{
    pub(crate) fn new(get: G, include: I, exclude: E) -> Self {
        todo!()
    }

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
