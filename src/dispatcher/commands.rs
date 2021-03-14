use crate::data::Entity;
use crate::resources::Resources;
use crate::world::{ComponentSet, Entities, World};
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Command buffer used for queueing commands which require 
/// exclusive access to `World` and `Resources`. 
pub struct Commands<'a> {
    buffer: &'a mut Vec<Command>,
    entities: &'a Entities,
}

impl<'a> Commands<'a> {
    pub(crate) fn new(buffer: &'a mut Vec<Command>, entities: &'a Entities) -> Self {
        Self { buffer, entities }
    }

    /// Queue a function to run with exclusive access to the `World` and `Resources`.
    pub fn run<F>(&mut self, command: F)
    where
        F: FnOnce(&mut World, &mut Resources) + Send + 'static,
    {
        self.buffer.push(Box::new(command));
    }

    /// Queue the creation of an entity with given components and
    /// return the `Entity` to be created.
    pub fn create<C>(&mut self, components: C) -> Entity
    where
        C: ComponentSet,
    {
        let entity = self.entities.create_atomic();

        self.run(move |world, _| {
            let _ = world.append(entity, components);
        });

        entity
    }

    /// Queue the creation of a set of entities with 
    /// components produced by the given iterator.
    pub fn extend<C, I>(&mut self, components_iter: I)
    where
        C: ComponentSet,
        I: IntoIterator<Item = C> + Send + 'static,
    {
        self.run(move |world, _| {
            world.extend(components_iter);
        });
    }

    /// Queue the destruction of the given `Entity`.
    pub fn destroy(&mut self, entity: Entity) {
        self.run(move |world, _| {
            world.destroy(entity);
        });
    }

    /// Queue appending a set of components to the given `Entity`.
    pub fn append<C>(&mut self, entity: Entity, components: C)
    where
        C: ComponentSet,
    {
        self.run(move |world, _| {
            let _ = world.append(entity, components);
        });
    }

    /// Queue the deletion of a set of components from the given `Entity`.
    pub fn delete<C>(&mut self, entity: Entity)
    where
        C: ComponentSet,
    {
        self.run(move |world, _| {
            world.delete::<C>(entity);
        });
    }
}

pub(crate) type Command = Box<dyn FnOnce(&mut World, &mut Resources) + Send + 'static>;

pub(crate) struct CommandBuffers {
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
