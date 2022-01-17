use crate::storage::Entity;

pub trait EntityIterator: Iterator {
    fn next_with_entity(&mut self) -> Option<(Entity, Self::Item)>;

    fn fold_with_entity<B, F>(mut self, mut init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, (Entity, Self::Item)) -> B,
    {
        while let Some(item) = self.next_with_entity() {
            init = f(init, item);
        }

        init
    }
}

pub struct EntityIter<I>(I);

impl<I> Iterator for EntityIter<I>
where
    I: EntityIterator,
{
    type Item = (Entity, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next_with_entity()
    }

    fn fold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.0.fold_with_entity(init, f)
    }
}

pub trait IntoEntityIter: EntityIterator + Sized {
    fn entities(self) -> EntityIter<Self>;
}

impl<I> IntoEntityIter for I
where
    I: EntityIterator,
{
    fn entities(self) -> EntityIter<Self> {
        EntityIter(self)
    }
}
