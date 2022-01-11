use crate::prelude::Entity;
use crate::query::{NonEmptyQuery, Query};

pub struct DenseExcludeIter<'a, G, E>
where
    G: NonEmptyQuery<'a>,
    E: Query<'a>,
{
    index: usize,
    entities: &'a [Entity],
    exclude: E::SparseArrays,
    components: G::ComponentPtrs,
}

impl<'a, G, E> DenseExcludeIter<'a, G, E>
where
    G: NonEmptyQuery<'a>,
    E: Query<'a>,
{
    pub(crate) unsafe fn new(
        entities: &'a [Entity],
        exclude: E::SparseArrays,
        components: G::ComponentPtrs,
    ) -> Self {
        Self { index: 0, entities, exclude, components }
    }
}

impl<'a, G, E> Iterator for DenseExcludeIter<'a, G, E>
where
    G: NonEmptyQuery<'a>,
    E: Query<'a>,
{
    type Item = G::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entity = *self.entities.get(self.index)?;

            let index = self.index;
            self.index += 1;

            if E::excludes_split(self.exclude, entity) {
                unsafe {
                    return Some(G::get_from_dense_components_unchecked(self.components, index));
                }
            }
        }
    }
}
