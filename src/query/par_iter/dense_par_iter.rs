use crate::entity::Entity;
use crate::query::Query;
use core::ops::Range;
use core::ptr::NonNull;
use rayon::iter::plumbing::{Consumer, ProducerCallback, UnindexedConsumer};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

/// Dense parallel iterator over all items that match a query. Extremely fast.
#[must_use]
pub struct DenseParIter<'a, G>
where
    G: Query,
{
    range: Range<usize>,
    entities: NonNull<Entity>,
    get_data: G::Data<'a>,
}

impl<'a, G> DenseParIter<'a, G>
where
    G: Query,
{
    pub(crate) unsafe fn new(
        range: Range<usize>,
        entities: &'a [Entity],
        get_data: G::Data<'a>,
    ) -> Self {
        let entities = NonNull::new_unchecked(entities.as_ptr().cast_mut());

        Self {
            range,
            entities,
            get_data,
        }
    }

    #[must_use]
    unsafe fn get(&self, index: usize) -> G::Item<'a> {
        let entity = *self.entities.add(index).as_ref();
        G::get_dense(self.get_data, index, entity)
    }
}

unsafe impl<G> Send for DenseParIter<'_, G>
where
    G: Query,
{
    // Empty
}

unsafe impl<G> Sync for DenseParIter<'_, G>
where
    G: Query,
{
    // Empty
}

impl<'a, G> ParallelIterator for DenseParIter<'a, G>
where
    G: Query,
{
    type Item = G::Item<'a>;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.range
            .clone()
            .into_par_iter()
            .map(|i| unsafe { self.get(i) })
            .drive_unindexed(consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.range.end - self.range.start)
    }
}

impl<'a, G> IndexedParallelIterator for DenseParIter<'a, G>
where
    G: Query,
{
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        self.range
            .clone()
            .into_par_iter()
            .map(|i| unsafe { self.get(i) })
            .drive(consumer)
    }

    fn len(&self) -> usize {
        self.range.end - self.range.start
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        self.range
            .clone()
            .into_par_iter()
            .map(|i| unsafe { self.get(i) })
            .with_producer(callback)
    }
}
