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
        let i = self.range.next()?;

        unsafe {
            let entity = *self.entities.add(i).as_ref();
            Some(G::get_by_index(self.get, entity, i))
        }
    }

    fn fold<B, F>(self, mut init: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        for i in self.range {
            unsafe {
                let entity = *self.entities.add(i).as_ref();
                init = f(init, G::get_by_index(self.get, entity, i));
            }
        }

        init
    }
}
