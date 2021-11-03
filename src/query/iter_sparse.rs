use crate::query::{IterData, QueryBase, QueryFilter, QueryModifier};
use crate::storage::Entity;
use crate::utils::EntityIterator;

/// Iterator over ungrouped queries.
pub struct SparseIter<'a, B, I, E, F>
where
    B: QueryBase<'a>,
    I: QueryModifier<'a>,
    E: QueryModifier<'a>,
    F: QueryFilter,
{
    iter_data: IterData<'a>,
    sparse: B::Sparse,
    data: B::Data,
    include: I::Split,
    exclude: E::Split,
    filter: F,
    index: usize,
}

impl<'a, B, I, E, F> SparseIter<'a, B, I, E, F>
where
    B: QueryBase<'a>,
    I: QueryModifier<'a>,
    E: QueryModifier<'a>,
    F: QueryFilter,
{
    pub(crate) fn new(
        iter_data: IterData<'a>,
        sparse: B::Sparse,
        data: B::Data,
        include: I::Split,
        exclude: E::Split,
        filter: F,
    ) -> Self {
        Self {
            iter_data,
            sparse,
            data,
            include,
            exclude,
            filter,
            index: 0,
        }
    }
}

impl<'a, B, I, E, F> Iterator for SparseIter<'a, B, I, E, F>
where
    B: QueryBase<'a>,
    I: QueryModifier<'a>,
    E: QueryModifier<'a>,
    F: QueryFilter,
{
    type Item = B::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entity = *self.iter_data.entities.get(self.index)?;
            self.index += 1;

            if self.filter.matches(entity)
                && E::excludes_split(&self.exclude, entity)
                && I::includes_split(&self.include, entity)
            {
                let item = unsafe {
                    B::get_from_sparse_parts(
                        &self.sparse,
                        entity,
                        &self.data,
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

unsafe impl<'a, B, I, E, F> EntityIterator for SparseIter<'a, B, I, E, F>
where
    B: QueryBase<'a>,
    I: QueryModifier<'a>,
    E: QueryModifier<'a>,
    F: QueryFilter,
{
    fn next_with_entity(&mut self) -> Option<(Entity, Self::Item)> {
        loop {
            let entity = *self.iter_data.entities.get(self.index)?;
            self.index += 1;

            if self.filter.matches(entity)
                && E::excludes_split(&self.exclude, entity)
                && I::includes_split(&self.include, entity)
            {
                let item = unsafe {
                    B::get_from_sparse_parts(
                        &self.sparse,
                        entity,
                        &self.data,
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
