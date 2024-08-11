mod dense_iter;
mod sparse_iter;

pub use self::dense_iter::*;
pub use self::sparse_iter::*;

use crate::entity::Entity;
use crate::query::{Query, WorldQueryAll};
use std::ops::Range;
use std::ptr::NonNull;

pub enum Iter<'query, 'view, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    Sparse(SparseIter<'query, 'view, G, I, E>),
    Dense(DenseIter<'query, 'view, G>),
}

impl<'query, 'view, G, I, E> Iter<'query, 'view, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    pub fn new(query: &'query mut WorldQueryAll<'view, G, I, E>) -> Self {
        let dense_data = (|| -> Option<(Range<usize>, NonNull<Entity>)> {
            let get_info = query.get_info?;
            let include_info = query.include_info?;
            let exclude_info = query.exclude_info?;

            let range = unsafe {
                query
                    .world
                    .components
                    .group_range(&get_info.add_query(&include_info)?, &exclude_info)?
            };

            let entities = G::entities(&query.get)
                .or_else(|| I::entities(&query.include))
                .unwrap_or(query.world.entities())
                .as_ptr()
                .cast_mut();

            unsafe { Some((range, NonNull::new_unchecked(entities))) }
        })();

        match dense_data {
            Some((range, entities)) => unsafe {
                Self::Dense(DenseIter::new(range, entities, &mut query.get))
            },
            None => Self::Sparse(SparseIter::new(query)),
        }
    }
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
