use crate::entity::Entity;

/// Component iterator able to yield the entities to which the components belong.
pub trait EntityIterator: Iterator {
    /// Returns the next entity and its associated components, if any.
    fn next_with_entity(&mut self) -> Option<(Entity, Self::Item)>;

    /// Folds all entities and their associated components into an accumulator.
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

/// Component iterator that also yields the entities to which the components belong.
#[must_use]
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

/// Helper trait for building an [`EntityIter`].
pub trait IntoEntityIter: Sized + EntityIterator {
    /// Makes the iterator return the entities to which the components belong.
    fn with_entity(self) -> EntityIter<Self>;
}

impl<I> IntoEntityIter for I
where
    I: EntityIterator,
{
    fn with_entity(self) -> EntityIter<Self> {
        EntityIter(self)
    }
}
