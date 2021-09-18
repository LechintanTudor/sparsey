use crate::query;
use crate::query::{DenseIter, QueryBase, QueryFilter, QueryModifier, SparseIter};
use crate::storage::Entity;
use crate::utils::EntityIterator;

/// Iterator over grouped or ungrouped component storages.
pub enum Iter<'a, B, I, E, F>
where
    B: QueryBase<'a>,
    I: QueryModifier<'a>,
    E: QueryModifier<'a>,
    F: QueryFilter,
{
    /// Iterator over ungrouped component storages.
    Sparse(SparseIter<'a, B, I, E, F>),
    /// Iterator over grouped component storages. Extremely fast.
    Dense(DenseIter<'a, B, F>),
}

impl<'a, B, I, E, F> Iter<'a, B, I, E, F>
where
    B: QueryBase<'a>,
    I: QueryModifier<'a>,
    E: QueryModifier<'a>,
    F: QueryFilter,
{
    /// Creates a new iterator from the given query parts.
    pub fn new(base: B, include: I, exclude: E, filter: F) -> Self {
        if query::is_trivial_group::<B, I, E>() {
            let (iter_data, base) = base.split_dense();

            unsafe { Self::Dense(DenseIter::new_unchecked(iter_data, base, filter)) }
        } else {
            match query::group_range(&base, &include, &exclude) {
                Ok(range) => {
                    let (base_data, base) = base.split_dense();
                    let iter_data = base_data.with_range(range);

                    unsafe { Self::Dense(DenseIter::new_unchecked(iter_data, base, filter)) }
                }
                Err(_) => {
                    let (base_data, base) = base.split_sparse();
                    let (include_data, include) = include.split_modifier();
                    let (_, exclude) = exclude.split_modifier();

                    let iter_data = if let Some(include_data) = include_data {
                        if base_data.entities.len() <= include_data.entities.len() {
                            base_data
                        } else {
                            include_data
                        }
                    } else {
                        base_data
                    };

                    Self::Sparse(SparseIter::new(iter_data, base, include, exclude, filter))
                }
            }
        }
    }

    /// Returns `true` if the iterator is dense.
    pub fn is_dense(&self) -> bool {
        matches!(self, Self::Dense(_))
    }
}

impl<'a, B, I, E, F> Iterator for Iter<'a, B, I, E, F>
where
    B: QueryBase<'a>,
    I: QueryModifier<'a>,
    E: QueryModifier<'a>,
    F: QueryFilter,
{
    type Item = B::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Sparse(sparse) => sparse.next(),
            Self::Dense(dense) => dense.next(),
        }
    }

    fn for_each<Func>(self, f: Func)
    where
        Self: Sized,
        Func: FnMut(Self::Item),
    {
        match self {
            Self::Sparse(sparse) => sparse.for_each(f),
            Self::Dense(dense) => dense.for_each(f),
        }
    }
}

unsafe impl<'a, B, I, E, F> EntityIterator for Iter<'a, B, I, E, F>
where
    B: QueryBase<'a>,
    I: QueryModifier<'a>,
    E: QueryModifier<'a>,
    F: QueryFilter,
{
    fn next_with_entity(&mut self) -> Option<(Entity, Self::Item)> {
        match self {
            Self::Sparse(sparse) => sparse.next_with_entity(),
            Self::Dense(dense) => dense.next_with_entity(),
        }
    }
}
