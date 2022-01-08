use crate::query::NonEmptyQuery;
use crate::storage::Entity;

pub struct DenseIter<'a, G>
where
    G: NonEmptyQuery<'a>,
{
    index: usize,
    entities: &'a [Entity],
    components: G::ComponentPtrs,
}

impl<'a, G> DenseIter<'a, G>
where
    G: NonEmptyQuery<'a>,
{
    pub(crate) unsafe fn new(entities: &'a [Entity], components: G::ComponentPtrs) -> Self {
        Self { index: 0, entities, components }
    }
}

impl<'a, G> Iterator for DenseIter<'a, G>
where
    G: NonEmptyQuery<'a>,
{
    type Item = G::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.entities.len() {
            return None;
        }

        let index = self.index;
        self.index += 1;

        unsafe {
            return Some(G::get_from_dense_components_unchecked(self.components, index));
        }
    }
}
