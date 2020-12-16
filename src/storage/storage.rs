use crate::entity::{Entity, IndexEntity};
use std::any::Any;

pub trait AbstractStorage
where
    Self: 'static,
{
    fn as_any(&self) -> &dyn Any;

    fn as_mut_any(&mut self) -> &mut dyn Any;

    fn get_index_entity(&self, entity: Entity) -> Option<&IndexEntity>;

    fn swap(&mut self, i1: usize, i2: usize);
}
