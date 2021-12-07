use crate::query::{QueryFilter, QueryGet};
use crate::storage::Entity;
use crate::utils::{EntityIterator, Ticks};

/// Iterator over grouped queries. Extremely fast.
pub struct DenseIter<'a, G, F>
where
    G: QueryGet<'a>,
    F: QueryFilter,
{
    entities: &'a [Entity],
    data: G::Data,
    filter: F,
    world_tick: Ticks,
    change_tick: Ticks,
    index: usize,
}

impl<'a, G, F> DenseIter<'a, G, F>
where
    G: QueryGet<'a>,
    F: QueryFilter,
{
    pub(crate) unsafe fn new_unchecked(
        entities: &'a [Entity],
        data: G::Data,
        filter: F,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Self {
        Self { entities, data, filter, world_tick, change_tick, index: 0 }
    }
}

impl<'a, G, F> Iterator for DenseIter<'a, G, F>
where
    G: QueryGet<'a>,
    F: QueryFilter,
{
    type Item = G::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entity = self.entities.get(self.index)?;

            let index = self.index;
            self.index += 1;

            if self.filter.matches(*entity) {
                let item = unsafe {
                    G::get_from_dense_unchecked(
                        &self.data,
                        index,
                        self.world_tick,
                        self.change_tick,
                    )
                };

                if item.is_some() {
                    return item;
                }
            }
        }
    }
}

unsafe impl<'a, G, F> EntityIterator for DenseIter<'a, G, F>
where
    G: QueryGet<'a>,
    F: QueryFilter,
{
    fn next_with_entity(&mut self) -> Option<(Entity, Self::Item)> {
        loop {
            let entity = *self.entities.get(self.index)?;

            let index = self.index;
            self.index += 1;

            if self.filter.matches(entity) {
                let item = unsafe {
                    G::get_from_dense_unchecked(
                        &self.data,
                        index,
                        self.world_tick,
                        self.change_tick,
                    )
                };

                if item.is_some() {
                    return item.map(|item| (entity, item));
                }
            }
        }
    }
}
