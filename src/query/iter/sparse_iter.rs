use crate::entity::Entity;
use crate::query::Query;
use core::slice::Iter as SliceIter;

pub struct SparseIter<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    entities: SliceIter<'a, Entity>,
    get_parts: G::SparseParts<'a>,
    include_sparse: I::Sparse<'a>,
    exclude_sparse: E::Sparse<'a>,
}

impl<'a, G, I, E> SparseIter<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    pub(crate) fn new(
        entities: &'a [Entity],
        get_parts: G::SparseParts<'a>,
        include_sparse: I::Sparse<'a>,
        exclude_sparse: E::Sparse<'a>,
    ) -> Self {
        Self {
            entities: entities.iter(),
            get_parts,
            include_sparse,
            exclude_sparse,
        }
    }
}

impl<'a, G, I, E> Iterator for SparseIter<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    type Item = G::Item<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some(&entity) = self.entities.next() else {
                break None;
            };

            if !E::sparse_contains_none(self.exclude_sparse, entity) {
                continue;
            }

            if !I::sparse_contains_all(self.include_sparse, entity) {
                continue;
            }

            unsafe {
                if let Some(item) = G::get_sparse(self.get_parts, entity) {
                    break Some(item);
                };
            }
        }
    }

    fn fold<B, F>(self, mut init: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        for &entity in self.entities {
            if !E::sparse_contains_none(self.exclude_sparse, entity) {
                continue;
            }

            if !I::sparse_contains_all(self.include_sparse, entity) {
                continue;
            }

            unsafe {
                if let Some(item) = G::get_sparse(self.get_parts, entity) {
                    init = f(init, item);
                };
            }
        }

        init
    }
}
