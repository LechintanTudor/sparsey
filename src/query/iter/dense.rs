use crate::query::{EntityIterator, Query};
use crate::storage::Entity;

/// Iterator over grouped storages. Extremely fast.
pub struct DenseIter<'a, G>
where
    G: Query<'a>,
{
    index: usize,
    entities: &'a [Entity],
    components: G::ComponentPtrs,
}

impl<'a, G> DenseIter<'a, G>
where
    G: Query<'a>,
{
    pub(crate) unsafe fn new(entities: &'a [Entity], components: G::ComponentPtrs) -> Self {
        Self { index: 0, entities, components }
    }
}

impl<'a, G> Iterator for DenseIter<'a, G>
where
    G: Query<'a>,
{
    type Item = G::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;

        if index >= self.entities.len() {
            return None;
        }

        self.index += 1;

        unsafe { Some(G::get_from_dense_components(self.components, index)) }
    }

    fn fold<B, F>(mut self, mut init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        while self.index < self.entities.len() {
            unsafe {
                init = f(init, G::get_from_dense_components(self.components, self.index));
            }

            self.index += 1;
        }

        init
    }
}

impl<'a, G> EntityIterator for DenseIter<'a, G>
where
    G: Query<'a>,
{
    fn next_with_entity(&mut self) -> Option<(Entity, Self::Item)> {
        let index = self.index;

        if index >= self.entities.len() {
            return None;
        }

        self.index += 1;

        unsafe {
            Some((
                *self.entities.get_unchecked(index),
                G::get_from_dense_components(self.components, index),
            ))
        }
    }

    fn fold_with_entity<B, F>(mut self, mut init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, (Entity, Self::Item)) -> B,
    {
        while self.index < self.entities.len() {
            unsafe {
                init = f(
                    init,
                    (
                        *self.entities.get_unchecked(self.index),
                        G::get_from_dense_components(self.components, self.index),
                    ),
                );
            }

            self.index += 1;
        }

        init
    }
}
