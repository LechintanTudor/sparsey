use crate::query;
use crate::query::iter::{DenseIter, SparseIter};
use crate::query::Query;

pub enum Iter<'a, G, I, E>
where
    G: Query<'a>,
    I: Query<'a>,
    E: Query<'a>,
{
    Sparse(SparseIter<'a, G, I, E>),
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
                let entities = &entities.unwrap_or_else(|| {
                    include.into_any_entities().expect("Cannot iterate empty Query")
                })[range];

                unsafe { Self::Dense(DenseIter::new(entities, components)) }
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
}
