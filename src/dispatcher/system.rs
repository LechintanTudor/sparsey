use crate::registry::{Resources, SyncResources, World};
use std::any::TypeId;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum RegistryAccess {
    Commands,
    Comp(TypeId),
    CompMut(TypeId),
    Res(TypeId),
    ResMut(TypeId),
}

pub trait ThreadLocalSystem
where
    Self: 'static,
{
    fn run_thread_local(&mut self, world: &mut World, resources: &mut Resources);
}

impl<F> ThreadLocalSystem for F
where
    F: FnMut(&mut World, &mut Resources) + 'static,
{
    fn run_thread_local(&mut self, world: &mut World, resources: &mut Resources) {
        self(world, resources);
    }
}

pub trait System
where
    Self: ThreadLocalSystem,
{
    unsafe fn registry_access(&self) -> Vec<RegistryAccess>;

    fn run(&mut self, world: &World, resources: SyncResources);
}
