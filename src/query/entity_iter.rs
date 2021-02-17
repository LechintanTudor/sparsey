use crate::data::Entity;

pub trait EntityIterator
where
    Self: Sized + Iterator,
{
    fn current_entity(&self) -> Option<Entity>;

    fn entities(self) -> EntityIter<Self> {
        EntityIter { inner: self }
    }
}

pub struct EntityIter<I>
where
    I: EntityIterator,
{
    inner: I,
}

impl<I> EntityIter<I>
where
    I: EntityIterator,
{
    pub fn current_entity(&self) -> Option<Entity> {
        self.inner.current_entity()
    }
}

impl<I> Iterator for EntityIter<I>
where
    I: EntityIterator,
{
    type Item = (Entity, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.inner.current_entity()?, self.inner.next()?))
    }
}
