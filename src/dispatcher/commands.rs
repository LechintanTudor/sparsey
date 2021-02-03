use crate::registry::{Resources, World};
use crate::storage::Entities;

pub struct Commands<'a> {
    buffer: &'a mut Vec<Box<dyn Command>>,
    entities: &'a Entities,
}

impl<'a> Commands<'a> {
    pub(crate) fn new(buffer: &'a mut Vec<Box<dyn Command>>, entities: &'a Entities) -> Self {
        Self { buffer, entities }
    }

    pub fn queue<C>(&mut self, command: C)
    where
        C: Command,
    {
        self.buffer.push(Box::new(command));
    }
}

pub trait Command
where
    Self: Send + 'static,
{
    fn run(self, world: &mut World, resources: &mut Resources);
}

impl<F> Command for F
where
    F: FnOnce(&mut World, &mut Resources) + Send + 'static,
{
    fn run(self, world: &mut World, resources: &mut Resources) {
        self(world, resources)
    }
}
