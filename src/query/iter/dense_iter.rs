use crate::entity::Entity;
use crate::query::QueryPart;

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
