use crate::world::World;
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};

pub(crate) type Command = Box<dyn FnOnce(&mut World) + Send + 'static>;
pub(crate) type CommandBuffer = Vec<Command>;

pub(crate) struct CommandBuffers {
	buffers: Vec<UnsafeCell<CommandBuffer>>,
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

	pub fn next(&self) -> Option<&mut CommandBuffer> {
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
