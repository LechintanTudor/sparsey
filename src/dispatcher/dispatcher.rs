use crate::dispatcher::{
	CommandBuffers, Environment, LocalSystem, LocallyRunnable, RunError, RunResult, System,
	SystemAccess, SystemError,
};
use crate::resources::Resources;
use crate::world::World;
use std::mem;

#[cfg(feature = "parallel")]
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
#[cfg(feature = "parallel")]
use rayon::ThreadPool;

/// Implements the builder pattern to create a `Dispatcher`.
#[derive(Default)]
pub struct DispatcherBuilder {
	simple_steps: Vec<SimpleStep>,
}

impl DispatcherBuilder {
	/// Add a system to the `Dispatcher`.
	pub fn add_system(&mut self, system: System) -> &mut Self {
		self.simple_steps.push(SimpleStep::RunSystem(system));
		self
	}

	/// Add a local system to the `Dispatcher` which runs on the current thread.
	pub fn add_local_system(&mut self, system: LocalSystem) -> &mut Self {
		self.simple_steps.push(SimpleStep::RunLocalSystem(system));
		self
	}

	/// Add a flush barrier which runs all the commands which need exclusive access
	/// to the `World` and `Resources`.
	pub fn add_flush(&mut self) -> &mut Self {
		self.simple_steps.push(SimpleStep::FlushCommands);
		self
	}

	/// Merge two `Dispatchers`. After the call `other` is empty.
	pub fn merge(&mut self, other: &mut DispatcherBuilder) -> &mut Self {
		self.simple_steps.extend(other.simple_steps.drain(..));
		self
	}

	/// Build a `Dispatcher` with the given systems and barriers.
	pub fn build(&mut self) -> Dispatcher {
		let steps = merge_and_optimize_steps(mem::take(&mut self.simple_steps));
		let command_buffers = CommandBuffers::new(required_command_buffers(&steps));

		Dispatcher {
			command_buffers,
			steps,
		}
	}
}

/// Used to run `Systems`, potentially in parallel.
pub struct Dispatcher {
	steps: Vec<Step>,
	command_buffers: CommandBuffers,
}

impl Dispatcher {
	/// Creates a `DispatcherBuilder` to enable creating a `Dispatcher`
	/// using the builder pattern.
	pub fn builder() -> DispatcherBuilder {
		DispatcherBuilder::default()
	}

	/// Adds the required component storages to the `World` to avoid
	/// having to add them manually via `World::register`.
	pub fn set_up(&self, world: &mut World) {
		for step in self.steps.iter() {
			match step {
				Step::RunSystems(systems) => {
					for access in systems.iter().flat_map(|sys| sys.accesses()) {
						match access {
							SystemAccess::Comp(comp) => {
								world.register_storage(comp.new_sparse_set())
							}
							SystemAccess::CompMut(comp) => {
								world.register_storage(comp.new_sparse_set())
							}
							_ => (),
						}
					}
				}
				Step::RunLocalSystems(systems) => {
					for access in systems.iter().flat_map(|sys| sys.accesses()) {
						match access {
							SystemAccess::Comp(comp) => {
								world.register_storage(comp.new_sparse_set())
							}
							SystemAccess::CompMut(comp) => {
								world.register_storage(comp.new_sparse_set())
							}
							_ => (),
						}
					}
				}
				_ => (),
			}
		}
	}

	/// Run all systems on the current thread.
	pub fn run_locally(&mut self, world: &mut World, resources: &mut Resources) -> RunResult {
		let mut errors = Vec::<SystemError>::new();

		for step in self.steps.iter_mut() {
			match step {
				Step::RunSystems(systems) => unsafe {
					run_locally(
						systems,
						world,
						resources,
						&self.command_buffers,
						&mut errors,
					);
				},
				Step::RunLocalSystems(systems) => unsafe {
					run_locally(
						systems,
						world,
						resources,
						&self.command_buffers,
						&mut errors,
					);
				},
				Step::FlushCommands => {
					world.maintain();

					for command in self.command_buffers.drain() {
						command(world, resources);
					}
				}
			}
		}

		if errors.len() != 0 {
			Err(RunError::from(errors))
		} else {
			Ok(())
		}
	}

	/// Run all systems, potentially in parallel, on the given `ThreadPool`.
	#[cfg(feature = "parallel")]
	pub fn run(
		&mut self,
		world: &mut World,
		resources: &mut Resources,
		thread_pool: &ThreadPool,
	) -> RunResult {
		let mut errors = Vec::<SystemError>::new();

		for step in self.steps.iter_mut() {
			match step {
				Step::RunSystems(systems) => {
					if systems.len() > 1 {
						unsafe {
							run(
								systems,
								world,
								resources,
								&self.command_buffers,
								thread_pool,
								&mut errors,
							);
						}
					} else {
						unsafe {
							run_locally(
								systems,
								world,
								resources,
								&self.command_buffers,
								&mut errors,
							);
						}
					}
				}
				Step::RunLocalSystems(systems) => unsafe {
					run_locally(
						systems,
						world,
						resources,
						&self.command_buffers,
						&mut errors,
					);
				},
				Step::FlushCommands => {
					for command in self.command_buffers.drain() {
						command(world, resources);
					}
				}
			}
		}

		if errors.len() != 0 {
			Err(RunError::from(errors))
		} else {
			Ok(())
		}
	}

	/// Get the maximum number of systems which can run in parallel.
	/// Mostly used for setting up the number of threads in the `ThreadPool`.
	pub fn max_parallel_systems(&self) -> usize {
		let mut max_parallel_systems = 0;

		for step in self.steps.iter() {
			match step {
				Step::RunSystems(systems) => {
					max_parallel_systems = max_parallel_systems.max(systems.len());
				}
				Step::RunLocalSystems(_) => {
					max_parallel_systems = max_parallel_systems.max(1);
				}
				_ => (),
			}
		}

		max_parallel_systems
	}
}

enum SimpleStep {
	RunSystem(System),
	RunLocalSystem(LocalSystem),
	FlushCommands,
}

enum Step {
	RunSystems(Vec<System>),
	RunLocalSystems(Vec<LocalSystem>),
	FlushCommands,
}

fn required_command_buffers(steps: &[Step]) -> usize {
	let mut max_buffer_count = 0;
	let mut buffer_count = 0;

	for step in steps {
		match step {
			Step::RunSystems(systems) => {
				let step_buffer_count: usize = systems
					.iter()
					.flat_map(|system| system.accesses())
					.map(|access| matches!(access, SystemAccess::Commands) as usize)
					.sum();

				buffer_count += step_buffer_count;
			}
			Step::RunLocalSystems(systems) => {
				let step_buffer_count: usize = systems
					.iter()
					.flat_map(|system| system.accesses())
					.map(|access| matches!(access, SystemAccess::Commands) as usize)
					.sum();

				buffer_count += step_buffer_count;
			}
			Step::FlushCommands => {
				max_buffer_count = max_buffer_count.max(buffer_count);
			}
		}
	}

	max_buffer_count
}

fn merge_and_optimize_steps(mut simple_steps: Vec<SimpleStep>) -> Vec<Step> {
	let mut steps = Vec::<Step>::new();

	for simple_step in simple_steps
		.drain(..)
		.chain(Some(SimpleStep::FlushCommands))
	{
		match simple_step {
			SimpleStep::RunSystem(system) => match steps.last_mut() {
				Some(Step::RunSystems(systems)) => {
					let systems_conflict =
						systems
							.iter()
							.flat_map(|system| system.accesses())
							.any(|access1| {
								system
									.accesses()
									.iter()
									.any(|access2| access1.conflicts(access2))
							});

					if systems_conflict {
						steps.push(Step::RunSystems(vec![system]));
					} else {
						systems.push(system);
					}
				}
				_ => {
					steps.push(Step::RunSystems(vec![system]));
				}
			},
			SimpleStep::RunLocalSystem(system) => match steps.last_mut() {
				Some(Step::RunLocalSystems(thread_local_systems)) => {
					thread_local_systems.push(system);
				}
				_ => steps.push(Step::RunLocalSystems(vec![system])),
			},
			SimpleStep::FlushCommands => match steps.last() {
				Some(Step::FlushCommands) | None => (),
				_ => steps.push(Step::FlushCommands),
			},
		}
	}

	steps
}

unsafe fn run_locally<S>(
	systems: &mut [S],
	world: &World,
	resources: &Resources,
	command_buffers: &CommandBuffers,
	errors: &mut Vec<SystemError>,
) where
	S: LocallyRunnable,
{
	let resources = resources.internal();

	let new_errors = systems.iter_mut().flat_map(|sys| {
		sys.run(Environment::new(world, resources, command_buffers))
			.err()
	});

	errors.extend(new_errors);
}

#[cfg(feature = "parallel")]
unsafe fn run(
	systems: &mut [System],
	world: &World,
	resources: &Resources,
	command_buffers: &CommandBuffers,
	thread_pool: &ThreadPool,
	errors: &mut Vec<SystemError>,
) {
	let resources = resources.internal();

	thread_pool.install(|| {
		let new_errors = systems
			.par_iter_mut()
			.flat_map_iter(|sys| {
				sys.run(Environment::new(world, resources, command_buffers))
					.err()
			})
			.collect::<Vec<_>>();

		errors.extend(new_errors);
	});
}
