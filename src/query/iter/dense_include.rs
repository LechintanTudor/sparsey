use crate::prelude::Entity;
use crate::query::{NonEmptyQuery, Query};

pub struct DenseIncludeIter<'a, G, I>
where
    G: NonEmptyQuery<'a>,
    I: Query<'a>,
{
    index: usize,
    entities: &'a [Entity],
    include: I::SparseArrays,
    components: G::ComponentPtrs,
}

impl<'a, G, I> DenseIncludeIter<'a, G, I>
where
    G: NonEmptyQuery<'a>,
    I: Query<'a>,
{
    pub(crate) unsafe fn new(
        entities: &'a [Entity],
        include: I::SparseArrays,
        components: G::ComponentPtrs,
    ) -> Self {
        Self { index: 0, entities, include, components }
    }
}

impl<'a, G, I> Iterator for DenseIncludeIter<'a, G, I>
where
    G: NonEmptyQuery<'a>,
    I: Query<'a>,
{
    type Item = G::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entity = *self.entities.get(self.index)?;

            let index = self.index;
            self.index += 1;

            if I::includes_split(self.include, entity) {
                unsafe {
                    return Some(G::get_from_dense_components_unchecked(self.components, index));
                }
            }
        }
    }
}
