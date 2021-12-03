use crate::query::{QueryFilter, QueryGet, QueryModifier};
use crate::storage::Entity;
use crate::utils::{EntityIterator, Ticks};

/// Iterator over ungrouped queries.
pub struct SparseIter<'a, G, I, E, F>
where
    G: QueryGet<'a>,
    I: QueryModifier<'a>,
    E: QueryModifier<'a>,
    F: QueryFilter,
{
    entities: &'a [Entity],
    sparse: G::Sparse,
    data: G::Data,
    include: I::Sparse,
    exclude: E::Sparse,
    filter: F,
    world_tick: Ticks,
    change_tick: Ticks,
    index: usize,
}

impl<'a, G, I, E, F> SparseIter<'a, G, I, E, F>
where
    G: QueryGet<'a>,
    I: QueryModifier<'a>,
    E: QueryModifier<'a>,
    F: QueryFilter,
{
    pub(crate) fn new(
        entities: &'a [Entity],
        sparse: G::Sparse,
        data: G::Data,
        include: I::Sparse,
        exclude: E::Sparse,
        filter: F,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Self {
        Self {
            entities,
            sparse,
            data,
            include,
            exclude,
            filter,
            world_tick,
            change_tick,
            index: 0,
        }
    }
}

impl<'a, G, I, E, F> Iterator for SparseIter<'a, G, I, E, F>
where
    G: QueryGet<'a>,
    I: QueryModifier<'a>,
    E: QueryModifier<'a>,
    F: QueryFilter,
{
    type Item = G::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entity = *self.entities.get(self.index)?;
            self.index += 1;

            if self.filter.matches(entity)
                && E::excludes_sparse(&self.exclude, entity)
                && I::includes_sparse(&self.include, entity)
            {
                let item = unsafe {
                    G::get_from_sparse_unchecked(
                        &self.sparse,
                        entity,
                        &self.data,
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

unsafe impl<'a, G, I, E, F> EntityIterator for SparseIter<'a, G, I, E, F>
where
    G: QueryGet<'a>,
    I: QueryModifier<'a>,
    E: QueryModifier<'a>,
    F: QueryFilter,
{
    fn next_with_entity(&mut self) -> Option<(Entity, Self::Item)> {
        loop {
            let entity = *self.entities.get(self.index)?;
            self.index += 1;

            if self.filter.matches(entity)
                && E::excludes_sparse(&self.exclude, entity)
                && I::includes_sparse(&self.include, entity)
            {
                let item = unsafe {
                    G::get_from_sparse_unchecked(
                        &self.sparse,
                        entity,
                        &self.data,
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
