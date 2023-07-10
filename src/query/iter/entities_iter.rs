use crate::storage::Entity;

/// Trait representing component iterators that are able to return the entity to which the
/// components belong.
pub trait EntityIterator: Iterator {
    /// Returns the next set of components and the entity to which they belong.
    fn next_with_entity(&mut self) -> Option<(Entity, Self::Item)>;

    /// Folds all entity and component set pairs into an accumulator and returns the result.
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

/// Wrapper over a component iterator that makes it also return the [`Entity`] to which the
/// components belong.
#[derive(Clone)]
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

/// Helper trait for creating an [`EntityIter`].
pub trait IntoEntityIter: EntityIterator + Sized {
    /// Makes the iterator also return the [`Entity`] to which the components belong.
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
