use crate::dispatcher::{
    CommandBuffers, Environment, LocalSystem, LocallyRunnable, System, SystemAccess,
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
    pub fn run_locally(&mut self, world: &mut World, resources: &mut Resources) {
        for step in self.steps.iter_mut() {
            match step {
                Step::RunSystems(systems) => {
                    for system in systems {
                        unsafe {
                            system.run(Environment::new(
                                world,
                                resources.internal(),
                                &self.command_buffers,
                            ));
                        }
                    }
                }
                Step::RunLocalSystems(systems) => {
                    for system in systems {
                        unsafe {
                            system.run(Environment::new(
                                world,
                                resources.internal(),
                                &self.command_buffers,
                            ));
                        }
                    }
                }
                Step::FlushCommands => {
                    world.maintain();

                    for command in self.command_buffers.drain() {
                        command(world, resources);
                    }
                }
            }
        }
    }

    /// Run all systems, potentially in parallel, on the given `ThreadPool`.
    #[cfg(feature = "parallel")]
    pub fn run(&mut self, world: &mut World, resources: &mut Resources, thread_pool: &ThreadPool) {
        for step in self.steps.iter_mut() {
            match step {
                Step::RunSystems(systems) => {
                    let resources = unsafe { resources.internal() };
                    let world = &world;
                    let command_buffers = &self.command_buffers;

                    if systems.len() > 1 {
                        thread_pool.install(|| {
                            systems.par_iter_mut().for_each(|system| unsafe {
                                system.run(Environment::new(world, resources, command_buffers));
                            });
                        });
                    } else {
                        if let Some(system) = systems.iter_mut().next() {
                            unsafe {
                                system.run(Environment::new(world, resources, command_buffers));
                            }
                        }
                    }
                }
                Step::RunLocalSystems(systems) => {
                    for system in systems {
                        unsafe {
                            system.run(Environment::new(
                                world,
                                resources.internal(),
                                &self.command_buffers,
                            ));
                        }
                    }
                }
                Step::FlushCommands => {
                    for command in self.command_buffers.drain() {
                        command(world, resources);
                    }
                }
            }
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
