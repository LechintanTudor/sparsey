use crate::entity::Entity;
use crate::query::Query;
use core::ops::Range;
use core::ptr::NonNull;

pub struct DenseIter<'query, 'view, G>
where
    G: Query,
{
    range: Range<usize>,
    entities: NonNull<Entity>,
    get: &'query mut G::View<'view>,
}

impl<'query, 'view, G> DenseIter<'query, 'view, G>
where
    G: Query,
{
    pub(crate) unsafe fn new(
        range: Range<usize>,
        entities: NonNull<Entity>,
        get: &'query mut G::View<'view>,
    ) -> Self {
        Self {
            range,
            entities,
            get,
        }
    }
}

impl<'query, G> Iterator for DenseIter<'query, '_, G>
where
    G: Query,
{
    type Item = G::Item<'query>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.range.next()?;

        unsafe {
            let entity = *self.entities.add(index).as_ref();
            G::get(self.get, entity)
        }
    }
}
