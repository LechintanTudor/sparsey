use crate::{entity::Entity, storage::SparseArray};

pub trait SparseSetLike<T> {
    fn split(&self) -> (&SparseArray, &[Entity], &[T]);

    fn split_mut(&mut self) -> (&SparseArray, &[Entity], &mut [T]);

    fn len(&self) -> usize;

    fn get(&self, entity: Entity) -> Option<&T>;

    fn get_mut(&mut self, entity: Entity) -> Option<&mut T>;

    fn contains(&self, entity: Entity) -> bool;
}
