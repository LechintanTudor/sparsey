use crate::dispatcher::{CommandBuffers, Registry, System, ThreadLocalSystem};
use crate::resources::Resources;
use crate::world::World;

enum SimpleStep {
    RunSystem(Box<dyn System>),
    RunThreadLocalSystem(Box<dyn ThreadLocalSystem>),
    FlushCommands,
}

enum Step {
    RunSystems(Vec<Box<dyn System>>),
    RunThreadLocalSystems(Vec<Box<dyn ThreadLocalSystem>>),
    FlushCommands,
}

#[derive(Default)]
pub struct DispatcherBuilder {
    simple_steps: Vec<SimpleStep>,
}

impl DispatcherBuilder {
    pub fn with_system(mut self, system: Box<dyn System>) -> Self {
        self.simple_steps.push(SimpleStep::RunSystem(system));
        self
    }

    pub fn with_thread_local_system(mut self, system: Box<dyn ThreadLocalSystem>) -> Self {
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

    pub fn build(mut self) -> Dispatcher {
        let mut steps = Vec::<Step>::new();
        let mut command_buffer_count = 100;

        for simple_step in self.simple_steps.drain(..) {
            match simple_step {
                SimpleStep::RunSystem(system) => match steps.last_mut() {
                    Some(Step::RunSystems(systems)) => {
                        let systems_conflict = systems
                            .iter()
                            .flat_map(|system| unsafe { system.registry_access() })
                            .any(|access1| unsafe {
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

        Dispatcher {
            steps,
            command_buffers: CommandBuffers::new(command_buffer_count),
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

    pub fn run(&mut self, world: &mut World, resources: &mut Resources) {
        for step in self.steps.iter_mut() {
            match step {
                Step::RunSystems(systems) => {
                    for system in systems {
                        unsafe {
                            system.run_unsafe(Registry::new(
                                world,
                                resources.internal(),
                                &self.command_buffers,
                            ));
                        }
                    }
                }
                Step::RunThreadLocalSystems(systems) => {
                    for system in systems {
                        unsafe {
                            system.run_unsafe(Registry::new(
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
}
