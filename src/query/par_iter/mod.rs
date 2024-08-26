mod dense_par_iter;
mod sparse_par_iter;

pub use self::dense_par_iter::*;
pub use self::sparse_par_iter::*;

use crate::query::Query;
use rayon::iter::plumbing::UnindexedConsumer;
use rayon::iter::ParallelIterator;

#[must_use]
pub enum ParIter<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    Sparse(SparseParIter<'a, G, I, E>),
    Dense(DenseParIter<'a, G>),
}

impl<'a, G, I, E> ParIter<'a, G, I, E>
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

impl<'a, G, I, E> ParallelIterator for ParIter<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
    G::Item<'a>: Send,
{
    type Item = G::Item<'a>;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        match self {
            Self::Sparse(iter) => iter.drive_unindexed(consumer),
            Self::Dense(iter) => iter.drive_unindexed(consumer),
        }
    }

    fn opt_len(&self) -> Option<usize> {
        match self {
            Self::Sparse(iter) => iter.opt_len(),
            Self::Dense(iter) => iter.opt_len(),
        }
    }
}

impl<'a, G, I, E> From<SparseParIter<'a, G, I, E>> for ParIter<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    fn from(iter: SparseParIter<'a, G, I, E>) -> Self {
        Self::Sparse(iter)
    }
}

impl<'a, G, I, E> From<DenseParIter<'a, G>> for ParIter<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    fn from(iter: DenseParIter<'a, G>) -> Self {
        Self::Dense(iter)
    }
}
