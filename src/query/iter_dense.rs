use crate::query::{IterData, QueryBase, QueryFilter};
use crate::storage::Entity;
use crate::utils::EntityIterator;

/// Iterator over grouped queries. Extremely fast.
pub struct DenseIter<'a, B, F>
where
    B: QueryBase<'a>,
    F: QueryFilter,
{
    iter_data: IterData<'a>,
    data: B::Data,
    filter: F,
    index: usize,
}

impl<'a, B, F> DenseIter<'a, B, F>
where
    B: QueryBase<'a>,
    F: QueryFilter,
{
    pub(crate) unsafe fn new_unchecked(iter_data: IterData<'a>, data: B::Data, filter: F) -> Self {
        Self {
            iter_data,
            data,
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
            let entity = self.iter_data.entities.get(self.index)?;

            let index = self.index;
            self.index += 1;

            if self.filter.matches(*entity) {
                let item = unsafe {
                    B::get_from_dense_parts_unchecked(
                        &self.data,
                        index,
                        self.iter_data.world_tick,
                        self.iter_data.change_tick,
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
            let entity = *self.iter_data.entities.get(self.index)?;

            let index = self.index;
            self.index += 1;

            if self.filter.matches(entity) {
                let item = unsafe {
                    B::get_from_dense_parts_unchecked(
                        &self.data,
                        index,
                        self.iter_data.world_tick,
                        self.iter_data.change_tick,
                    )
                };

                if item.is_some() {
                    return item.map(|item| (entity, item));
                }
            }
        }
    }
}
