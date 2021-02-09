use crate::dispatcher::{
    CommandBuffers, Registry, RegistryAccess, Runnable, System, ThreadLocalRunnable,
    ThreadLocalSystem,
};
use crate::resources::Resources;
use crate::world::World;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use rayon::ThreadPool;

enum SimpleStep {
    RunSystem(System),
    RunThreadLocalSystem(ThreadLocalSystem),
    FlushCommands,
}

enum Step {
    RunSystems(Vec<System>),
    RunThreadLocalSystems(Vec<ThreadLocalSystem>),
    FlushCommands,
}

#[derive(Default)]
pub struct DispatcherBuilder {
    simple_steps: Vec<SimpleStep>,
}

impl DispatcherBuilder {
    pub fn with_system(mut self, system: System) -> Self {
        self.simple_steps.push(SimpleStep::RunSystem(system));
        self
    }

    pub fn with_thread_local_system(mut self, system: ThreadLocalSystem) -> Self {
        self.simple_steps
            .push(SimpleStep::RunThreadLocalSystem(system));
        self
    }

    pub fn with_barrier(mut self) -> Self {
        self.simple_steps.push(SimpleStep::FlushCommands);
        self
    }

    pub fn merge(mut self, mut other: DispatcherBuilder) -> Self {
        self.simple_steps.extend(other.simple_steps.drain(..));
        self
    }

    pub fn build(self) -> Dispatcher {
        let steps = merge_and_optimize_steps(self.simple_steps);
        let command_buffers = CommandBuffers::new(count_command_buffers(&steps));

        Dispatcher {
            command_buffers,
            steps,
        }
    }
}

pub struct Dispatcher {
    steps: Vec<Step>,
    command_buffers: CommandBuffers,
}

impl Dispatcher {
    pub fn builder() -> DispatcherBuilder {
        DispatcherBuilder::default()
    }

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
                                system.run(Registry::new(world, resources, command_buffers));
                            });
                        });
                    } else {
                        if let Some(system) = systems.iter_mut().next() {
                            unsafe {
                                system.run(Registry::new(world, resources, command_buffers));
                            }
                        }
                    }
                }
                Step::RunThreadLocalSystems(systems) => {
                    for system in systems {
                        unsafe {
                            system.run_thread_local(Registry::new(
                                world,
                                resources.internal(),
                                &self.command_buffers,
                            ));
                        }
                    }
                }
                Step::FlushCommands => {
                    for command in self.command_buffers.drain() {
                        command.run(world, resources);
                    }
                }
            }
        }
    }

    pub fn max_parallel_systems(&self) -> usize {
        let mut max_parallel_systems = 0;

        for step in self.steps.iter() {
            match step {
                Step::RunSystems(systems) => {
                    max_parallel_systems = max_parallel_systems.max(systems.len());
                }
                Step::RunThreadLocalSystems(_) => {
                    max_parallel_systems = max_parallel_systems.max(1);
                }
                _ => (),
            }
        }

        max_parallel_systems
    }
}

fn count_command_buffers(steps: &[Step]) -> usize {
    let mut command_buffer_count = 0;

    for step in steps {
        match step {
            Step::RunSystems(systems) => {
                let step_command_buffer_count: usize = systems
                    .iter()
                    .flat_map(|system| system.registry_access())
                    .map(|access| matches!(access, RegistryAccess::Commands) as usize)
                    .sum();

                command_buffer_count += step_command_buffer_count;
            }
            Step::RunThreadLocalSystems(systems) => {
                let step_command_buffer_count: usize = systems
                    .iter()
                    .flat_map(|system| system.registry_access())
                    .map(|access| matches!(access, RegistryAccess::Commands) as usize)
                    .sum();

                command_buffer_count += step_command_buffer_count;
            }
            _ => (),
        }
    }

    command_buffer_count
}

fn merge_and_optimize_steps(mut simple_steps: Vec<SimpleStep>) -> Vec<Step> {
    let mut steps = Vec::<Step>::new();
    println!("{}", simple_steps.len());

    for simple_step in simple_steps
        .drain(..)
        .chain(Some(SimpleStep::FlushCommands))
    {
        match simple_step {
            SimpleStep::RunSystem(system) => match steps.last_mut() {
                Some(Step::RunSystems(systems)) => {
                    let systems_conflict = systems
                        .iter()
                        .flat_map(|system| system.registry_access())
                        .any(|access1| {
                            system
                                .registry_access()
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
            SimpleStep::RunThreadLocalSystem(system) => match steps.last_mut() {
                Some(Step::RunThreadLocalSystems(thread_local_systems)) => {
                    thread_local_systems.push(system);
                }
                _ => steps.push(Step::RunThreadLocalSystems(vec![system])),
            },
            SimpleStep::FlushCommands => match steps.last() {
                Some(Step::FlushCommands) | None => (),
                _ => steps.push(Step::FlushCommands),
            },
        }
    }

    steps
}
