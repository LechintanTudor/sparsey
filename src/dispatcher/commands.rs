use crate::resources::Resources;
use crate::storage::Entities;
use crate::world::World;
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Command(Box<dyn FnOnce(&mut World, &mut Resources) + Send + 'static>);

impl Command {
    pub fn run(self, world: &mut World, resources: &mut Resources) {
        self.0(world, resources)
    }
}

impl<F> From<F> for Command
where
    F: FnOnce(&mut World, &mut Resources) + Send + 'static,
{
    fn from(function: F) -> Self {
        Self(Box::new(function))
    }
}

pub struct CommandBuffers {
    buffers: Vec<UnsafeCell<Vec<Command>>>,
    index: AtomicUsize,
}

unsafe impl Sync for CommandBuffers {}

impl CommandBuffers {
    pub fn new(buffer_count: usize) -> Self {
        let mut buffers = Vec::new();
        buffers.resize_with(buffer_count, || UnsafeCell::new(Vec::new()));

        Self {
            buffers,
            index: AtomicUsize::new(0),
        }
    }

    pub fn next(&self) -> Option<&mut Vec<Command>> {
        let mut prev = self.index.load(Ordering::Relaxed);

        while prev < self.buffers.len() {
            match self.index.compare_exchange_weak(
                prev,
                prev + 1,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(result) => unsafe { return Some(&mut *self.buffers[result].get()) },
                Err(next_prev) => prev = next_prev,
            }
        }

        None
    }

    pub fn drain(&mut self) -> impl Iterator<Item = Command> + '_ {
        let used_buffers = *self.index.get_mut();
        *self.index.get_mut() = 0;

        self.buffers
            .iter_mut()
            .take(used_buffers)
            .flat_map(|buffer| unsafe { (&mut *buffer.get()).drain(..) })
    }
}

pub struct Commands<'a> {
    buffer: &'a mut Vec<Command>,
    _entities: &'a Entities,
}

impl<'a> Commands<'a> {
    pub(crate) fn new(buffer: &'a mut Vec<Command>, entities: &'a Entities) -> Self {
        Self {
            buffer,
            _entities: entities,
        }
    }

    pub fn queue<F>(&mut self, command: F)
    where
        F: FnOnce(&mut World, &mut Resources) + Send + 'static,
    {
        self.buffer.push(command.into());
    }
}
