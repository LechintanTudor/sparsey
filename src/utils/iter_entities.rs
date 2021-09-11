use crate::storage::Entity;

/// Trait used for creating an `EntityIter`.
pub trait EntityIterator
where
    Self: Iterator + Sized,
{
    /// Returns the current `Entity` the iterator is pointing at, if any.
    fn current_entity(&self) -> Option<Entity>;

    /// Wrapps the iterator in an `EntityIter`.
    fn entities(self) -> EntityIter<Self> {
        EntityIter(self)
    }
}

/// Wrapper over an iterator which makes it return the `Entity` to which an item
/// belongs.
pub struct EntityIter<I>(I);

impl<I> Iterator for EntityIter<I>
where
    I: EntityIterator,
{
    type Item = (Entity, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.0.current_entity()?, self.0.next()?))
    }
}
