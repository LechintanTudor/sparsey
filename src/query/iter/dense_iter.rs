use crate::entity::Entity;
use crate::query::{EntityIterator, QueryPart};

/// Iterator over packed component arrays.
pub struct DenseIter<'a, G>
where
    G: QueryPart,
{
    index: usize,
    entities: &'a [Entity],
    ptrs: G::Ptrs,
}

impl<'a, G> DenseIter<'a, G>
where
    G: QueryPart,
{
    pub(crate) unsafe fn new(entities: &'a [Entity], ptrs: G::Ptrs) -> Self {
        Self {
            index: 0,
            entities,
            ptrs,
        }
    }
}

impl<'a, G> Iterator for DenseIter<'a, G>
where
    G: QueryPart + 'a,
{
    type Item = G::Refs<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;

        if index >= self.entities.len() {
            return None;
        }

        self.index += 1;

        unsafe { Some(G::get_dense(self.ptrs, index)) }
    }

    fn fold<B, F>(mut self, mut init: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        while self.index < self.entities.len() {
            init = unsafe { f(init, G::get_dense(self.ptrs, self.index)) };
            self.index += 1;
        }

        init
    }
}

impl<'a, G> EntityIterator for DenseIter<'a, G>
where
    G: QueryPart + 'a,
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
                G::get_dense(self.ptrs, index),
            ))
        }
    }

    fn fold_with_entity<B, F>(mut self, mut init: B, mut f: F) -> B
    where
        F: FnMut(B, (Entity, Self::Item)) -> B,
    {
        while self.index < self.entities.len() {
            init = unsafe {
                f(
                    init,
                    (
                        *self.entities.get_unchecked(self.index),
                        G::get_dense(self.ptrs, self.index),
                    ),
                )
            };

            self.index += 1;
        }

        init
    }
}
