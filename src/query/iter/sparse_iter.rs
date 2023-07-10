use crate::query::{EntityIterator, QueryPart};
use crate::storage::Entity;
use std::slice::Iter as SliceIter;

/// Iterator over ungrouped storages.
pub struct SparseIter<'a, G, I, E>
where
    G: QueryPart + 'a,
    I: QueryPart + 'a,
    E: QueryPart + 'a,
{
    entities: SliceIter<'a, Entity>,
    sparse: G::Sparse<'a>,
    include: I::Sparse<'a>,
    exclude: E::Sparse<'a>,
    components: G::Ptrs,
}

impl<'a, G, I, E> SparseIter<'a, G, I, E>
where
    G: QueryPart,
    I: QueryPart,
    E: QueryPart,
{
    pub(crate) unsafe fn new(
        entities: &'a [Entity],
        sparse: G::Sparse<'a>,
        include: I::Sparse<'a>,
        exclude: E::Sparse<'a>,
        components: G::Ptrs,
    ) -> Self {
        Self {
            entities: entities.iter(),
            sparse,
            include,
            exclude,
            components,
        }
    }
}

impl<'a, G, I, E> Iterator for SparseIter<'a, G, I, E>
where
    G: QueryPart + 'a,
    I: QueryPart + 'a,
    E: QueryPart + 'a,
{
    type Item = G::Refs<'a> where Self: 'a;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entity = *self.entities.next()?;

            if !E::sparse_contains_none(self.exclude, entity) {
                continue;
            }

            if !I::sparse_contains_all(self.include, entity) {
                continue;
            }

            if let Some(components) =
                unsafe { G::sparse_get(self.sparse, self.components, entity.sparse()) }
            {
                return Some(components);
            }
        }
    }

    fn fold<B, F>(self, mut init: B, mut f: F) -> B
    where
        Self: Sized + 'a,
        F: FnMut(B, Self::Item) -> B,
    {
        for &entity in self.entities {
            if !E::sparse_contains_none(self.exclude, entity) {
                continue;
            }

            if !I::sparse_contains_all(self.include, entity) {
                continue;
            }

            if let Some(components) =
                unsafe { G::sparse_get(self.sparse, self.components, entity.sparse()) }
            {
                init = f(init, components);
            }
        }

        init
    }
}

impl<'a, G, I, E> EntityIterator for SparseIter<'a, G, I, E>
where
    G: QueryPart,
    I: QueryPart,
    E: QueryPart,
{
    fn next_with_entity(&mut self) -> Option<(Entity, Self::Item)> {
        loop {
            let entity = *self.entities.next()?;

            if !E::sparse_contains_none(self.exclude, entity) {
                continue;
            }

            if !I::sparse_contains_all(self.include, entity) {
                continue;
            }

            if let Some(components) =
                unsafe { G::sparse_get(self.sparse, self.components, entity.sparse()) }
            {
                return Some((entity, components));
            }
        }
    }

    fn fold_with_entity<B, F>(self, mut init: B, mut f: F) -> B
    where
        Self: Sized + 'a,
        F: FnMut(B, (Entity, Self::Item)) -> B,
    {
        for &entity in self.entities {
            if !E::sparse_contains_none(self.exclude, entity) {
                continue;
            }

            if !I::sparse_contains_all(self.include, entity) {
                continue;
            }

            if let Some(components) =
                unsafe { G::sparse_get(self.sparse, self.components, entity.sparse()) }
            {
                init = f(init, (entity, components));
            }
        }

        init
    }
}
