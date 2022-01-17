use crate::query::{EntityIterator, Query};
use crate::storage::Entity;
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
    pub(crate) unsafe fn new(entities: &'a [Entity], components: G::ComponentPtrs) -> Self {
        Self { entities: entities.iter(), components }
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
            self.components = G::next_component_ptrs(self.components);
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
                self.components = G::next_component_ptrs(self.components);
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
            self.components = G::next_component_ptrs(self.components);
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
                self.components = G::next_component_ptrs(self.components);
            }
        }

        init
    }
}
