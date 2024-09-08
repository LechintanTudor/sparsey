use crate::entity::Entity;
use crate::query::Query;
use core::iter::FusedIterator;
use core::ops::Range;
use core::ptr::NonNull;

/// Dense iterator over all items that match a query. Extremely fast.
#[must_use]
pub struct DenseIter<'a, G>
where
    G: Query,
{
    range: Range<usize>,
    entities: NonNull<Entity>,
    get_data: G::Data<'a>,
}

impl<'a, G> DenseIter<'a, G>
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
}

impl<'a, G> Iterator for DenseIter<'a, G>
where
    G: Query,
{
    type Item = G::Item<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.range.next()?;

        unsafe {
            let entity = *self.entities.add(i).as_ref();
            Some(G::get_dense(self.get_data, i, entity))
        }
    }

    fn fold<B, F>(self, mut init: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        for i in self.range {
            unsafe {
                let entity = *self.entities.add(i).as_ref();
                init = f(init, G::get_dense(self.get_data, i, entity));
            }
        }

        init
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        Iterator::size_hint(&self.range)
    }
}

impl<G> ExactSizeIterator for DenseIter<'_, G>
where
    G: Query,
{
    fn len(&self) -> usize {
        ExactSizeIterator::len(&self.range)
    }
}

impl<G> FusedIterator for DenseIter<'_, G>
where
    G: Query,
{
    // Empty
}
