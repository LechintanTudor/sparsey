use crate::dispatcher::{Command, Commands, System, ThreadLocalSystem};
use crate::registry::{Resources, World};
use std::cell::UnsafeCell;

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
    raw_steps: Vec<SimpleStep>,
}

impl DispatcherBuilder {
    pub fn with_system(mut self, system: Box<dyn System>) -> Self {
        self.raw_steps.push(SimpleStep::RunSystem(system));
        self
    }

    pub fn with_thread_local_system(mut self, system: Box<dyn ThreadLocalSystem>) -> Self {
        self.raw_steps
            .push(SimpleStep::RunThreadLocalSystem(system));
        self
    }

    pub fn flush_command_buffers(mut self) -> Self {
        if !matches!(self.raw_steps.last(), Some(SimpleStep::FlushCommands)) {
            self.raw_steps.push(SimpleStep::FlushCommands);
        }

        self
    }

    pub fn merge(mut self, mut other: DispatcherBuilder) -> Self {
        for raw_command in other.raw_steps.drain(..) {
            if matches!(raw_command, SimpleStep::FlushCommands) {
                self = self.flush_command_buffers();
            } else {
                self.raw_steps.push(raw_command);
            }
        }

        self
    }

    pub fn build(self) -> Dispatcher {
        todo!()
    }
}

pub struct Dispatcher {
    step: Vec<Step>,
}

impl Dispatcher {
    pub fn builder() -> DispatcherBuilder {
        DispatcherBuilder::default()
    }

    pub fn run(&mut self, world: &mut World, resources: &mut Resources) {}
}
