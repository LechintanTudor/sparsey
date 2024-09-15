use crate::entity::Entity;
use crate::query::Query;
use core::iter::FusedIterator;
use core::slice::Iter as SliceIter;

/// Sparse iterator over all items that match a query.
#[must_use]
pub struct SparseIter<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    entities: SliceIter<'a, Entity>,
    exclude_sparse: E::Sparse<'a>,
    include_sparse: I::Sparse<'a>,
    get_sparse: G::Sparse<'a>,
    get_data: G::Data<'a>,
}

impl<'a, G, I, E> SparseIter<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    pub(crate) fn new(
        entities: &'a [Entity],
        exclude_sparse: E::Sparse<'a>,
        include_sparse: I::Sparse<'a>,
        get_sparse: G::Sparse<'a>,
        get_data: G::Data<'a>,
    ) -> Self {
        Self {
            entities: entities.iter(),
            exclude_sparse,
            include_sparse,
            get_sparse,
            get_data,
        }
    }
}

impl<'a, G, I, E> Iterator for SparseIter<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    type Item = G::Item<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some(&entity) = self.entities.next() else {
                break None;
            };

            let sparse = entity.sparse();

            if !E::contains_none_raw(self.exclude_sparse, sparse) {
                continue;
            }

            if !I::contains_all_raw(self.include_sparse, sparse) {
                continue;
            }

            unsafe {
                if let Some(item) = G::get_sparse_raw(self.get_sparse, self.get_data, entity) {
                    break Some(item);
                };
            }
        }
    }

    fn fold<B, F>(self, mut init: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        for &entity in self.entities {
            let sparse = entity.sparse();

            if !E::contains_none_raw(self.exclude_sparse, sparse) {
                continue;
            }

            if !I::contains_all_raw(self.include_sparse, sparse) {
                continue;
            }

            unsafe {
                if let Some(item) = G::get_sparse_raw(self.get_sparse, self.get_data, entity) {
                    init = f(init, item);
                };
            }
        }

        init
    }
}

impl<G, I, E> FusedIterator for SparseIter<'_, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    // Empty
}
