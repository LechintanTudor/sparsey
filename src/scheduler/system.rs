use crate::registry::{World, Resources};
use std::any::TypeId;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum RegistryAccess {
    Comp(TypeId),
    CompMut(TypeId),
    Res(TypeId),
    ResMut(TypeId),
}

pub trait ThreadLocalSystem 
where 
    Self: Send + 'static,
{
    unsafe fn registry_accesses(&self, accesses: &mut Vec<RegistryAccess>);

    fn run_thread_local(&mut self, world: &World, resources: &Resources);
}

pub trait System
where 
    Self: Sync + ThreadLocalSystem,
{
    fn run(&mut self, world: &World, resources: &Resources);
}
