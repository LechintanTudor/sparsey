use crate::dispatcher::{Command, System, ThreadLocalSystem};
use crate::registry::{Resources, World};

pub struct ThreadLocalExecutor {
    command_buffer: Vec<Box<dyn Command>>,
    systems: Vec<Box<dyn ThreadLocalSystem>>,
}

impl ThreadLocalExecutor {
    pub fn run(&mut self, world: &mut World, resources: &mut Resources) {
        todo!()
    }
}

pub struct Executor {
    command_buffers: Vec<Vec<Box<dyn Command>>>,
    systems: Vec<Box<dyn System>>,
}

impl Executor {
    pub fn run(&mut self, world: &mut World, resources: &mut Resources) {
        todo!()
    }
}
