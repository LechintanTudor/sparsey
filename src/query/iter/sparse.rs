use crate::query::{EntityIterator, Query};
use crate::storage::Entity;
use std::slice::Iter as SliceIter;

/// Iterator over ungrouped storages.
pub struct SparseIter<'a, G, I, E>
where
    G: Query<'a>,
    I: Query<'a>,
    E: Query<'a>,
{
    entities: SliceIter<'a, Entity>,
    sparse: G::SparseArrays,
    include: I::SparseArrays,
    exclude: E::SparseArrays,
    components: G::ComponentPtrs,
}

impl<'a, G, I, E> SparseIter<'a, G, I, E>
where
    G: Query<'a>,
    I: Query<'a>,
    E: Query<'a>,
{
    pub(crate) unsafe fn new(
        entities: &'a [Entity],
        sparse: G::SparseArrays,
        include: I::SparseArrays,
        exclude: E::SparseArrays,
        components: G::ComponentPtrs,
    ) -> Self {
        Self { entities: entities.iter(), sparse, include, exclude, components }
    }
}

impl<'a, G, I, E> Iterator for SparseIter<'a, G, I, E>
where
    G: Query<'a>,
    I: Query<'a>,
    E: Query<'a>,
{
    type Item = G::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entity = *self.entities.next()?;

            if E::excludes_split(self.exclude, entity) && I::includes_split(self.include, entity) {
                if let Some(index) = G::get_index_from_split(self.sparse, entity) {
                    unsafe {
                        return Some(G::get_from_sparse_components(self.components, index));
                    }
                }
            }
        }
    }

    fn fold<B, F>(self, mut init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        for &entity in self.entities {
            if E::excludes_split(self.exclude, entity) && I::includes_split(self.include, entity) {
                if let Some(index) = G::get_index_from_split(self.sparse, entity) {
                    unsafe {
                        init = f(init, G::get_from_sparse_components(self.components, index));
                    }
                }
            }
        }

        init
    }
}

impl<'a, G, I, E> EntityIterator for SparseIter<'a, G, I, E>
where
    G: Query<'a>,
    I: Query<'a>,
    E: Query<'a>,
{
    fn next_with_entity(&mut self) -> Option<(Entity, Self::Item)> {
        loop {
            let entity = *self.entities.next()?;

            if E::excludes_split(self.exclude, entity) && I::includes_split(self.include, entity) {
                if let Some(index) = G::get_index_from_split(self.sparse, entity) {
                    unsafe {
                        return Some((
                            entity,
                            G::get_from_sparse_components(self.components, index),
                        ));
                    }
                }
            }
        }
    }

    fn fold_with_entity<B, F>(self, mut init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, (Entity, Self::Item)) -> B,
    {
        for &entity in self.entities {
            if E::excludes_split(self.exclude, entity) && I::includes_split(self.include, entity) {
                if let Some(index) = G::get_index_from_split(self.sparse, entity) {
                    unsafe {
                        init = f(
                            init,
                            (entity, G::get_from_sparse_components(self.components, index)),
                        );
                    }
                }
            }
        }

        init
    }
}
