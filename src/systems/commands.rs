use crate::components::Entity;
use crate::utils::ChangeTicks;
use crate::world::{ComponentSet, EntityStorage, World};
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Command buffer used for queueing commands which require
/// exclusive access to `World` and `Resources`.
pub struct Commands<'a> {
	buffer: &'a mut Vec<Command>,
	entities: &'a EntityStorage,
}

impl<'a> Commands<'a> {
	pub(crate) fn new(buffer: &'a mut Vec<Command>, entities: &'a EntityStorage) -> Self {
		Self { buffer, entities }
	}

	/// Queue a function to run with exclusive access to the `World` and
	/// `Resources`.
	pub fn run<F>(&mut self, command: F)
	where
		F: FnOnce(&mut World) + Send + 'static,
	{
		self.buffer.push(Box::new(command));
	}

	/// Queue the creation of an entity with given `components` and
	/// return the `Entity` to be created.
	pub fn create<C>(&mut self, components: C) -> Entity
	where
		C: ComponentSet,
	{
		let entity = self.entities.create_atomic();

		self.run(move |world| {
			let _ = world.append(entity, components);
		});

		entity
	}

	/// Same as `create`, but the `ChangeTicks` are provided byt the caller.
	pub fn create_with_ticks<C>(&mut self, components: C, ticks: ChangeTicks) -> Entity
	where
		C: ComponentSet,
	{
		let entity = self.entities.create_atomic();

		self.run(move |world| {
			let _ = world.append_with_ticks(entity, components, ticks);
		});

		entity
	}

	/// Queue the creation of entities with components produced by the given
	/// iterator.
	pub fn extend<C, I>(&mut self, components_iter: I)
	where
		C: ComponentSet,
		I: IntoIterator<Item = C> + Send + 'static,
	{
		self.run(move |world| {
			world.extend(components_iter);
		});
	}

	/// Same as `extend`, but the `ChangeTicks` are provided by the caller.
	pub fn extend_with_ticks<C, I>(&mut self, components_iter: I, ticks: ChangeTicks)
	where
		C: ComponentSet,
		I: IntoIterator<Item = C> + Send + 'static,
	{
		self.run(move |world| {
			world.extend_with_ticks(components_iter, ticks);
		});
	}

	/// Queue the destruction of `entity`.
	pub fn destroy(&mut self, entity: Entity) {
		self.run(move |world| {
			world.destroy(entity);
		});
	}

	/// Queue the appending of `components` to `entity`.
	pub fn append<C>(&mut self, entity: Entity, components: C)
	where
		C: ComponentSet,
	{
		self.run(move |world| {
			let _ = world.append(entity, components);
		});
	}

	/// Same as `append`, but the `ChangeTicks` are provided by the caller.
	pub fn append_with_ticks<C>(&mut self, entity: Entity, components: C, ticks: ChangeTicks)
	where
		C: ComponentSet,
	{
		self.run(move |world| {
			let _ = world.append_with_ticks(entity, components, ticks);
		});
	}

	/// Queue the deletion of a set of components from the given `Entity`.
	pub fn delete<C>(&mut self, entity: Entity)
	where
		C: ComponentSet,
	{
		self.run(move |world| {
			world.delete::<C>(entity);
		});
	}

	/// Get a slice containing all entities in the `World`.
	pub fn entities(&self) -> &[Entity] {
		self.entities.as_ref()
	}
}

pub(crate) type Command = Box<dyn FnOnce(&mut World) + Send + 'static>;

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
			.flat_map(|buffer| buffer.get_mut().drain(..))
	}
}
