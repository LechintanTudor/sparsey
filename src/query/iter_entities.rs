use crate::storage::Entity;

/// Trait implemented by iterators over entities. Used internally by
/// `EntityIter`.
pub unsafe trait EntityIterator
where
    Self: Iterator,
{
    fn next_with_entity(&mut self) -> Option<(Entity, Self::Item)>;

    fn fold_with_entity<B, F>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, (Entity, Self::Item)) -> B,
    {
        let mut accumulator = init;
        while let Some(item) = self.next_with_entity() {
            accumulator = f(accumulator, item);
        }
        accumulator
    }
}

/// Trait used for creating an `EntityIter`.
pub trait IntoEntityIterator
where
    Self: EntityIterator + Sized,
{
    /// Wrapps the iterator in an `EntityIter`.
    fn entities(self) -> EntityIter<Self>;
}

impl<I> IntoEntityIterator for I
where
    I: EntityIterator + Sized,
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

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next_with_entity()
    }

    #[inline]
    fn fold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.0.fold_with_entity(init, f)
    }
}
