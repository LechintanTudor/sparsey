use crate::query;
use crate::query::iter::{DenseIter, SparseIter};
use crate::query::{NonEmptyQuery, Query};

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
        let get_info = get.group_info();
        let include_info = include.group_info();
        let exclude_info = exclude.group_info();

        match query::group_range(get_info, include_info, exclude_info) {
            Some(range) => {
                let (entities, components) = get.split_dense();
                let entities = unsafe { &entities.unwrap_unchecked().get_unchecked(range) };

                unsafe { Self::Dense(DenseIter::new(entities, components)) }
            }
            None => {
                let (get_entities, sparse, components) = get.split_sparse();
                let (sparse_entities, include) = include.split_filter();
                let (_, exclude) = exclude.split_filter();

                let get_entities = unsafe { get_entities.unwrap_unchecked() };
                let entities = match sparse_entities {
                    Some(sparse_entities) if sparse_entities.len() < get_entities.len() => {
                        sparse_entities
                    }
                    _ => get_entities,
                };

                unsafe {
                    Self::Sparse(SparseIter::new(entities, sparse, include, exclude, components))
                }
            }
        }
    }
}
