use crate::data::Entity;

pub trait EntityIterator
where
    Self: Sized + Iterator,
{
    fn current_entity(&self) -> Option<Entity>;

    fn entities(self) -> EntityIter<Self> {
        EntityIter(self)
    }
}

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
