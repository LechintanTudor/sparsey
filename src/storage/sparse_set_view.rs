use crate::{entity::Entity, storage::SparseArray};

pub trait SparseSetView {
    type Component;

    fn sparse(&self) -> &SparseArray;

    fn dense(&self) -> &[Entity];

    fn data(&self) -> &[Self::Component];

    fn len(&self) -> usize;

    fn contains(&self, entity: Entity) -> bool;

    fn get(&self, entity: Entity) -> Option<&Self::Component>;

    fn split(&self) -> (&SparseArray, &[Entity], &[Self::Component]);

    unsafe fn get_unchecked(&self, entity: Entity) -> &Self::Component;
}

pub trait SparseSetViewMut
where
    Self: SparseSetView,
{
    fn data_mut(&mut self) -> &mut [Self::Component];

    fn get_mut(&mut self, entity: Entity) -> Option<&mut Self::Component>;

    fn split_mut(&mut self) -> (&SparseArray, &[Entity], &mut [Self::Component]);

    unsafe fn get_unchecked_mut(&mut self, entity: Entity) -> &mut Self::Component;
}
