use crate::data::Entity;

/// Trait implemented by iterators over `ComponentViews`
/// to produce iterators which also yield the `Entity`
/// associated to the components.
pub trait EntityIterator
where
    Self: Sized + Iterator,
{
    /// Get the current `Entity` of the iterator, if any.
    fn current_entity(&self) -> Option<Entity>;

    /// Create an iterator which also yields the `Entities`
    /// associated to the components.
    fn entities(self) -> EntityIter<Self> {
        EntityIter(self)
    }
}

/// Wrapper for iterators over `ComponentViews` which
/// also yields the `Entity` associated to the components.
pub struct EntityIter<I>(I)
where
    I: EntityIterator;

impl<I> Iterator for EntityIter<I>
where
    I: EntityIterator,
{
    type Item = (Entity, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.0.current_entity()?, self.0.next()?))
    }
}
