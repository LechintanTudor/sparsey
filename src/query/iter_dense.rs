use crate::query::{IterData, QueryBase, QueryFilter};
use crate::storage::Entity;
use crate::utils::EntityIterator;

/// Iterator over grouped queries. Extremely fast.
pub struct DenseIter<'a, B, F>
where
    B: QueryBase<'a>,
    F: QueryFilter,
{
    data: IterData<'a>,
    base: B::DenseSplit,
    filter: F,
    index: usize,
}

impl<'a, B, F> DenseIter<'a, B, F>
where
    B: QueryBase<'a>,
    F: QueryFilter,
{
    pub(crate) unsafe fn new_unchecked(data: IterData<'a>, base: B::DenseSplit, filter: F) -> Self {
        Self {
            data,
            base,
            filter,
            index: 0,
        }
    }
}

impl<'a, B, F> Iterator for DenseIter<'a, B, F>
where
    B: QueryBase<'a>,
    F: QueryFilter,
{
    type Item = B::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entity = self.data.entities.get(self.index)?;

            let index = self.index;
            self.index += 1;

            if self.filter.matches(*entity) {
                let item = unsafe {
                    B::get_from_dense_split(
                        &mut self.base,
                        index,
                        self.data.world_tick,
                        self.data.change_tick,
                    )
                };

                if item.is_some() {
                    return item;
                }
            }
        }
    }
}

unsafe impl<'a, B, F> EntityIterator for DenseIter<'a, B, F>
where
    B: QueryBase<'a>,
    F: QueryFilter,
{
    fn next_with_entity(&mut self) -> Option<(Entity, Self::Item)> {
        loop {
            let entity = *self.data.entities.get(self.index)?;

            let index = self.index;
            self.index += 1;

            if self.filter.matches(entity) {
                let item = unsafe {
                    B::get_from_dense_split(
                        &mut self.base,
                        index,
                        self.data.world_tick,
                        self.data.change_tick,
                    )
                };

                if item.is_some() {
                    return item.map(|item| (entity, item));
                }
            }
        }
    }
}
