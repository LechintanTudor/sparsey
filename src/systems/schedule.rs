use crate::resources::{Resources, SyncResources};
use crate::systems::{
    IntoLocalFn, IntoLocalSystem, IntoSystem, LocalFn, LocalSystem, System, SystemParamType,
};
use crate::world::World;
use std::cmp::Ordering;
use std::fmt;

const DEFAULT_MAX_SYSTEMS_PER_STEP: usize = 5;

enum SimpleScheduleStep {
    System(System),
    LocalSystem(LocalSystem),
    LocalFn(LocalFn),
    Barrier,
}

impl fmt::Debug for SimpleScheduleStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::System(_) => write!(f, "SimpleScheduleStep::System"),
            Self::LocalSystem(_) => write!(f, "SimpleScheduleStep::LocalSystem"),
            Self::LocalFn(_) => write!(f, "SimpleScheduleStep::LocalFn"),
            Self::Barrier => write!(f, "SimpleScheduleStep::Barrier"),
        }
    }
}

#[derive(Default, Debug)]
pub struct ScheduleBuilder {
    steps: Vec<SimpleScheduleStep>,
    final_steps: Vec<SimpleScheduleStep>,
}

impl ScheduleBuilder {
    pub fn add_system<P>(&mut self, system: impl IntoSystem<P>) -> &mut Self {
        self.steps.push(SimpleScheduleStep::System(system.system()));
        self
    }

    pub fn add_local_system<P>(&mut self, system: impl IntoLocalSystem<P>) -> &mut Self {
        self.final_steps.push(SimpleScheduleStep::LocalSystem(system.local_system()));
        self
    }

    pub fn add_local_fn(&mut self, local_fn: impl IntoLocalFn) -> &mut Self {
        self.final_steps.push(SimpleScheduleStep::LocalFn(local_fn.local_fn()));
        self
    }

    pub fn add_barrier(&mut self) -> &mut Self {
        self.steps.push(SimpleScheduleStep::Barrier);
        self
    }

    pub fn add_barrier_system<P>(&mut self, system: impl IntoLocalSystem<P>) -> &mut Self {
        self.steps.push(SimpleScheduleStep::LocalSystem(system.local_system()));
        self
    }

    pub fn add_barrier_fn(
        &mut self,
        local_fn: impl FnMut(&mut World, &mut Resources) + 'static,
    ) -> &mut Self {
        self.steps.push(SimpleScheduleStep::LocalFn(local_fn.local_fn()));
        self
    }

    pub fn append(&mut self, other: &mut ScheduleBuilder) -> &mut Self {
        self.steps.append(&mut other.steps);
        self.final_steps.append(&mut other.final_steps);
        self
    }

    pub fn build(&mut self) -> Schedule {
        self.build_with_max_threads(DEFAULT_MAX_SYSTEMS_PER_STEP)
    }

    pub fn build_with_max_threads(&mut self, max_threads: usize) -> Schedule {
        fn step_to_non_conflicting_systems<'a>(
            step: &'a mut ScheduleStep,
            system: &System,
        ) -> Option<&'a mut Vec<System>> {
            match step {
                ScheduleStep::Systems(systems) => {
                    let systems_conflict = systems
                        .iter()
                        .flat_map(|s| s.params())
                        .any(|p1| system.params().iter().any(|p2| p1.conflicts_with(p2)));

                    if systems_conflict {
                        None
                    } else {
                        Some(systems)
                    }
                }
                _ => None,
            }
        }

        let mut steps = Vec::<ScheduleStep>::new();

        self.steps.drain(..).chain(self.final_steps.drain(..)).for_each(|step| match step {
            SimpleScheduleStep::System(system) => {
                let systems = steps
                    .iter_mut()
                    .rev()
                    .map_while(|step| step_to_non_conflicting_systems(step, &system))
                    .min_by(|s1, s2| s1.len().cmp(&s2.len()).then(Ordering::Greater))
                    .filter(|s| s.len() < max_threads);

                match systems {
                    Some(systems) => systems.push(system),
                    None => steps.push(ScheduleStep::Systems(vec![system])),
                }
            }
            SimpleScheduleStep::LocalSystem(system) => match steps.last_mut() {
                Some(ScheduleStep::LocalSystems(systems)) => systems.push(system),
                _ => steps.push(ScheduleStep::LocalSystems(vec![system])),
            },
            SimpleScheduleStep::LocalFn(local_fn) => match steps.last_mut() {
                Some(ScheduleStep::LocalFns(local_fns)) => local_fns.push(local_fn),
                _ => steps.push(ScheduleStep::LocalFns(vec![local_fn])),
            },
            SimpleScheduleStep::Barrier => {
                if matches!(steps.last(), Some(ScheduleStep::Systems(_))) {
                    steps.push(ScheduleStep::Barrier)
                }
            }
        });

        Schedule { steps }
    }
}

pub enum ScheduleStep {
    Systems(Vec<System>),
    LocalSystems(Vec<LocalSystem>),
    LocalFns(Vec<LocalFn>),
    Barrier,
}

impl fmt::Debug for ScheduleStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Systems(systems) => {
                f.debug_tuple("ScheduleStep::Systems").field(&systems.len()).finish()
            }
            Self::LocalSystems(systems) => {
                f.debug_tuple("ScheduleStep::LocalSystems").field(&systems.len()).finish()
            }
            Self::LocalFns(local_fns) => {
                f.debug_tuple("ScheduleStep::LocalFns").field(&local_fns.len()).finish()
            }
            Self::Barrier => {
                write!(f, "ScheduleStep::Barrier")
            }
        }
    }
}

#[derive(Debug)]
pub struct Schedule {
    steps: Vec<ScheduleStep>,
}

impl Schedule {
    pub fn builder() -> ScheduleBuilder {
        Default::default()
    }

    pub fn set_up(&self, world: &mut World) {
        fn register(world: &mut World, param: &SystemParamType) {
            unsafe {
                match param {
                    SystemParamType::Comp(c) => {
                        world.register_with(c.type_id(), || c.create_storage())
                    }
                    SystemParamType::CompMut(c) => {
                        world.register_with(c.type_id(), || c.create_storage())
                    }
                    _ => (),
                }
            }
        }

        for step in self.steps.iter() {
            match step {
                ScheduleStep::Systems(systems) => {
                    for param in systems.iter().flat_map(|s| s.params()) {
                        register(world, param);
                    }
                }
                ScheduleStep::LocalSystems(systems) => {
                    for param in systems.iter().flat_map(|s| s.params()) {
                        register(world, param);
                    }
                }
                _ => (),
            }
        }
    }

    pub fn run(&mut self, world: &mut World, resources: &mut Resources) {
        #[cfg(feature = "parallel")]
        self.run_par(world, resources);

        #[cfg(not(feature = "parallel"))]
        self.run_seq(world, resources);
    }

    pub fn run_seq(&mut self, world: &mut World, resources: &mut Resources) {
        self.run_generic(world, resources, |systems, world, resources| {
            for system in systems {
                system.run(world, resources);
            }
        })
    }

    #[cfg(feature = "parallel")]
    pub fn run_par(&mut self, world: &mut World, resources: &mut Resources) {
        use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

        self.run_generic(world, resources, |systems, world, resources| {
            if systems.len() > 1 {
                systems.par_iter_mut().for_each(|system| {
                    system.run(world, resources);
                });
            } else {
                systems.last_mut().unwrap().run(world, resources);
            }
        })
    }

    #[cfg(feature = "parallel")]
    pub fn run_in_thread_pool(
        &mut self,
        world: &mut World,
        resources: &mut Resources,
        thread_pool: &rayon::ThreadPool,
    ) {
        use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

        self.run_generic(world, resources, |systems, world, resources| {
            if systems.len() > 1 {
                thread_pool.install(|| {
                    systems.par_iter_mut().for_each(|system| {
                        system.run(world, resources);
                    });
                });
            } else {
                systems.last_mut().unwrap().run(world, resources);
            }
        })
    }

    fn run_generic(
        &mut self,
        world: &mut World,
        resources: &mut Resources,
        mut system_runner: impl FnMut(&mut [System], &World, SyncResources) + Send,
    ) {
        for step in self.steps.iter_mut() {
            match step {
                ScheduleStep::Systems(systems) => {
                    system_runner(systems, world, resources.sync());
                    world.maintain();
                }
                ScheduleStep::LocalSystems(systems) => {
                    for system in systems.iter_mut() {
                        system.run(world, resources);
                    }
                }
                ScheduleStep::LocalFns(fns) => {
                    for local_fn in fns {
                        (local_fn).run(world, resources);
                    }
                }
                ScheduleStep::Barrier => (),
            }
        }
    }

    pub fn max_threads(&self) -> usize {
        fn step_to_system_count(step: &ScheduleStep) -> Option<usize> {
            match step {
                ScheduleStep::Systems(systems) => Some(systems.len()),
                _ => None,
            }
        }

        self.steps.iter().flat_map(step_to_system_count).max().unwrap_or(1)
    }

    pub fn into_steps(self) -> Vec<ScheduleStep> {
        self.steps
    }
}