use crate::query::{EntityIterator, Query};
use crate::storage::Entity;
use std::ops::Range;
use std::slice::Iter as SliceIter;

pub struct DenseIter<'a, G>
where
    G: Query<'a>,
{
    entities: SliceIter<'a, Entity>,
    components: G::ComponentPtrs,
}

impl<'a, G> DenseIter<'a, G>
where
    G: Query<'a>,
{
    pub(crate) unsafe fn new(
        entities: &'a [Entity],
        components: G::ComponentPtrs,
        range: Range<usize>,
    ) -> Self {
        let offset = range.start as isize;
        let entities = entities.get_unchecked(range).iter();
        let components = G::offset_component_ptrs(components, offset);

        Self { entities, components }
    }
}

impl<'a, G> Iterator for DenseIter<'a, G>
where
    G: Query<'a>,
{
    type Item = G::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.entities.next()?;

        unsafe {
            let item = G::get_from_component_ptrs(self.components);
            self.components = G::offset_component_ptrs(self.components, 1);
            Some(item)
        }
    }

    fn fold<B, F>(mut self, mut init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        for _ in self.entities {
            unsafe {
                init = f(init, G::get_from_component_ptrs(self.components));
                self.components = G::offset_component_ptrs(self.components, 1);
            }
        }

        init
    }
}

impl<'a, G> EntityIterator for DenseIter<'a, G>
where
    G: Query<'a>,
{
    fn next_with_entity(&mut self) -> Option<(Entity, Self::Item)> {
        let entity = *self.entities.next()?;

        unsafe {
            let item = G::get_from_component_ptrs(self.components);
            self.components = G::offset_component_ptrs(self.components, 1);
            Some((entity, item))
        }
    }

    fn fold_with_entity<B, F>(mut self, mut init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, (Entity, Self::Item)) -> B,
    {
        for &entity in self.entities {
            unsafe {
                init = f(init, (entity, G::get_from_component_ptrs(self.components)));
                self.components = G::offset_component_ptrs(self.components, 1);
            }
        }

        init
    }
}
