use crate::query::{EntityIterator, QueryPart};
use crate::storage::Entity;

/// Iterator over grouped storages. Extremely fast.
pub struct DenseIter<'a, G>
where
    G: QueryPart + 'a,
{
    index: usize,
    entities: &'a [Entity],
    components: G::Ptrs,
}

impl<'a, G> DenseIter<'a, G>
where
    G: QueryPart,
{
    pub(crate) unsafe fn new(entities: &'a [Entity], components: G::Ptrs) -> Self {
        Self {
            index: 0,
            entities,
            components,
        }
    }
}

impl<'a, G> Iterator for DenseIter<'a, G>
where
    G: QueryPart,
{
    type Item = G::Refs<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;

        if index >= self.entities.len() {
            return None;
        }

        self.index += 1;

        unsafe { Some(G::dense_get(self.components, index)) }
    }

    fn fold<B, F>(mut self, mut init: B, mut f: F) -> B
    where
        Self: Sized + 'a,
        F: FnMut(B, Self::Item) -> B,
    {
        while self.index < self.entities.len() {
            init = unsafe { f(init, G::dense_get(self.components, self.index)) };

            self.index += 1;
        }

        init
    }
}

impl<'a, G> EntityIterator for DenseIter<'a, G>
where
    G: QueryPart,
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
                G::dense_get(self.components, index),
            ))
        }
    }

    fn fold_with_entity<B, F>(mut self, mut init: B, mut f: F) -> B
    where
        Self: Sized + 'a,
        F: FnMut(B, (Entity, Self::Item)) -> B,
    {
        while self.index < self.entities.len() {
            init = unsafe {
                f(
                    init,
                    (
                        *self.entities.get_unchecked(self.index),
                        G::dense_get(self.components, self.index),
                    ),
                )
            };

            self.index += 1;
        }

        init
    }
}
