use crate::components;
use crate::query::iter::{DenseIter, SparseIter};
use crate::query::{NonEmptyQuery, Query, QueryType};
use std::ops::Range;

pub enum Iter<'a, G, I, E>
where
    G: NonEmptyQuery<'a>,
    I: Query<'a>,
    E: Query<'a>,
{
    Sparse(SparseIter<'a, G, I, E>),
    Dense(DenseIter<'a, G>),
}

impl<'a, G, I, E> Iter<'a, G, I, E>
where
    G: NonEmptyQuery<'a>,
    I: Query<'a>,
    E: Query<'a>,
{
    pub(crate) fn new(get: G, include: I, exclude: E) -> Self {
        let group_range = (|| -> Option<Range<usize>> {
            let get_info = get.non_empty_group_info()?;
            let include_info = include.group_info()?;
            let exclude_info = exclude.group_info()?;

            // Dense or

            if G::TYPE == QueryType::Single && I::TYPE == QueryType::Empty {
                // Single
            } else {
                // Sparse
            }

            todo!()
        })();

        todo!()
    }
}
