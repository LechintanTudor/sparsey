use crate::entity::Entity;
use crate::query::Query;
use rayon::iter::plumbing::UnindexedConsumer;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

/// Sparse parallel iterator over all items that match a query.
#[must_use]
pub struct SparseParIter<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    entities: &'a [Entity],
    exclude_sparse: E::Sparse<'a>,
    include_sparse: I::Sparse<'a>,
    get_sparse: G::Sparse<'a>,
    get_data: G::Data<'a>,
}

impl<'a, G, I, E> SparseParIter<'a, G, I, E>
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
            entities,
            exclude_sparse,
            include_sparse,
            get_sparse,
            get_data,
        }
    }

    #[must_use]
    unsafe fn get(&self, entity: Entity) -> Option<G::Item<'a>> {
        let sparse = entity.sparse();

        if !E::contains_none_raw(self.exclude_sparse, sparse) {
            return None;
        }

        if !I::contains_all_raw(self.include_sparse, sparse) {
            return None;
        }

        unsafe { G::get_sparse_raw(self.get_sparse, self.get_data, entity) }
    }
}

unsafe impl<G, I, E> Send for SparseParIter<'_, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    // Empty
}

unsafe impl<G, I, E> Sync for SparseParIter<'_, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    // Empty
}

impl<'a, G, I, E> ParallelIterator for SparseParIter<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    type Item = G::Item<'a>;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.entities
            .par_iter()
            .flat_map(|&entity| unsafe { self.get(entity) })
            .drive_unindexed(consumer)
    }
}
