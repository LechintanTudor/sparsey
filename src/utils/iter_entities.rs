use crate::storage::Entity;

/// Trait implemented by iterators over entities. Used internally by
/// `EntityIter`.
pub unsafe trait EntityIterator
where
    Self: Iterator,
{
    fn next_with_entity(&mut self) -> Option<(Entity, Self::Item)>;
}

/// Trait used for creating an `EntityIter`.
pub trait IntoEntityIterator
where
    Self: Iterator + Sized,
{
    /// Wrapps the iterator in an `EntityIter`.
    fn entities(self) -> EntityIter<Self>;
}

impl<I> IntoEntityIterator for I
where
    I: Iterator + Sized,
{
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
        self.0.next_with_entity()
    }
}
