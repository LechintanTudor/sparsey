use crate::entity::Entity;
use crate::query::Query;
use core::slice::Iter as SliceIter;

#[must_use]
pub struct SparseIter<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    entities: SliceIter<'a, Entity>,
    exclude_sparse: E::Sparse<'a>,
    include_sparse: I::Sparse<'a>,
    get_sparse: G::Sparse<'a>,
    get_data: G::Data<'a>,
}

impl<'a, G, I, E> SparseIter<'a, G, I, E>
where
    G: Query,
    I: Query,
    E: Query,
{
    pub(crate) fn new(
        entities: &'a [Entity],
        exclude_sparse: E::Sparse<'a>,
        include_sparse: I::Sparse<'a>,
        get_sparse: G::Sparse<'a>,
        get_data: G::Data<'a>,
    ) -> Self {
        Self {
            entities: entities.iter(),
            exclude_sparse,
            include_sparse,
            get_sparse,
            get_data,
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
                if let Some(item) = G::get_sparse(self.get_sparse, self.get_data, entity) {
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
                if let Some(item) = G::get_sparse(self.get_sparse, self.get_data, entity) {
                    init = f(init, item);
                };
            }
        }

        init
    }
}
