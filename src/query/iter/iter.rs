use crate::query::iter::{DenseIter, SparseIter};
use crate::query::{DenseExcludeIter, DenseIncludeIter, NonEmptyQuery, Query};

pub enum Iter<'a, G, I, E>
where
    G: NonEmptyQuery<'a>,
    I: Query<'a>,
    E: Query<'a>,
{
    Sparse(SparseIter<'a, G, I, E>),
    DenseExclude(DenseExcludeIter<'a, G, E>),
    DenseInclude(DenseIncludeIter<'a, G, I>),
    Dense(DenseIter<'a, G>),
}

impl<'a, G, I, E> Iter<'a, G, I, E>
where
    G: NonEmptyQuery<'a>,
    I: Query<'a>,
    E: Query<'a>,
{
    pub(crate) fn new(get: G, include: I, exclude: E) -> Self {
        /*
            if grouped(G, I)
                if grouped_exclude((G, I), E)
                    Dense
                else
                    DenseExclude
            else
                if get.len <= include.len AND grouped_exclude(G, E)
                    DenseInclude
                else
                    Sparse
        */

        todo!()
    }
}
