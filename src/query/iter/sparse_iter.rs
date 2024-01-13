use crate::entity::Entity;
use crate::query::{EntityIterator, QueryPart};
use std::slice::Iter as SliceIter;

pub struct SparseIter<'a, G, I, E>
where
    G: QueryPart,
    I: QueryPart,
    E: QueryPart,
{
    entities: SliceIter<'a, Entity>,
    sparse: G::Sparse<'a>,
    include: I::Sparse<'a>,
    exclude: E::Sparse<'a>,
    ptrs: G::Ptrs,
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
        ptrs: G::Ptrs,
    ) -> Self {
        Self {
            entities: entities.iter(),
            sparse,
            include,
            exclude,
            ptrs,
        }
    }
}

impl<'a, G, I, E> Iterator for SparseIter<'a, G, I, E>
where
    G: QueryPart + 'a,
    I: QueryPart,
    E: QueryPart,
{
    type Item = G::Refs<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entity = *self.entities.next()?;

            if !I::sparse_contains_all(self.include, entity) {
                continue;
            }

            if !E::sparse_contains_none(self.exclude, entity) {
                continue;
            }

            if let Some(components) =
                unsafe { G::get_sparse(self.sparse, self.ptrs, entity.sparse()) }
            {
                return Some(components);
            }
        }
    }

    fn fold<B, F>(self, mut init: B, mut f: F) -> B
    where
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
                unsafe { G::get_sparse(self.sparse, self.ptrs, entity.sparse()) }
            {
                init = f(init, components);
            }
        }

        init
    }
}

impl<'a, G, I, E> EntityIterator for SparseIter<'a, G, I, E>
where
    G: QueryPart + 'a,
    I: QueryPart,
    E: QueryPart,
{
    fn next_with_entity(&mut self) -> Option<(Entity, Self::Item)> {
        loop {
            let entity = *self.entities.next()?;

            if !I::sparse_contains_all(self.include, entity) {
                continue;
            }

            if !E::sparse_contains_none(self.exclude, entity) {
                continue;
            }

            if let Some(components) =
                unsafe { G::get_sparse(self.sparse, self.ptrs, entity.sparse()) }
            {
                return Some((entity, components));
            }
        }
    }

    fn fold_with_entity<B, F>(self, mut init: B, mut f: F) -> B
    where
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
                unsafe { G::get_sparse(self.sparse, self.ptrs, entity.sparse()) }
            {
                init = f(init, (entity, components));
            }
        }

        init
    }
}
