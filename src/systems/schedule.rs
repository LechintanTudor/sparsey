use crate::resources::{Resources, SyncResources};
use crate::systems::{
    ExclusiveSystem, IntoExclusiveSystem, IntoLocalSystem, IntoSystem, LocalSystem, System,
    SystemBorrow,
};
use crate::world::World;

#[derive(Debug)]
enum SimpleScheduleStep {
    System(System),
    LocalSystem(LocalSystem),
    ExclusiveSystem(ExclusiveSystem),
    Barrier,
}

/// Enables creating a [`Schedule`] using the builder pattern.
#[derive(Default, Debug)]
pub struct ScheduleBuilder {
    steps: Vec<SimpleScheduleStep>,
    final_steps: Vec<SimpleScheduleStep>,
}

impl ScheduleBuilder {
    /// Adds a system to the schedule.
    pub fn add_system<P>(&mut self, system: impl IntoSystem<P>) -> &mut Self {
        self.steps.push(SimpleScheduleStep::System(system.system()));
        self
    }

    /// Adds a local system to the schedule. Prevents future systems from running in parallel
    /// with the previously added systems. Prefer to use
    /// [`ScheduleBuilder::add_local_system_to_end`] whenever possible.
    pub fn add_local_system<P>(&mut self, local_system: impl IntoLocalSystem<P>) -> &mut Self {
        self.steps
            .push(SimpleScheduleStep::LocalSystem(local_system.local_system()));
        self
    }

    /// Adds an exclusive system to the schedule. Prevents future systems from running in parallel
    /// with the previously added systems. Prefer to use
    /// [`ScheduleBuilder::add_exclusive_system_to_end`] whenever possible.
    pub fn add_exclusive_system<P>(
        &mut self,
        exclusive_system: impl IntoExclusiveSystem<P>,
    ) -> &mut Self {
        self.steps.push(SimpleScheduleStep::ExclusiveSystem(
            exclusive_system.exclusive_system(),
        ));
        self
    }

    /// Adds a barrier, preventing future systems from running in parallel with the previously added
    /// systems.
    pub fn add_barrier(&mut self) -> &mut Self {
        self.steps.push(SimpleScheduleStep::Barrier);
        self
    }

    /// Adds a local system to the end of the schedule.
    pub fn add_local_system_to_end<P>(
        &mut self,
        local_system: impl IntoLocalSystem<P>,
    ) -> &mut Self {
        self.final_steps
            .push(SimpleScheduleStep::LocalSystem(local_system.local_system()));
        self
    }

    /// Adds an exclusive system to the end of the schedule.
    pub fn add_exclusive_system_to_end<P>(
        &mut self,
        exclusive_system: impl IntoExclusiveSystem<P>,
    ) -> &mut Self {
        self.final_steps.push(SimpleScheduleStep::ExclusiveSystem(
            exclusive_system.exclusive_system(),
        ));
        self
    }

    /// Appends the steps of the given `ScheduleBuilder` to the current schedule.
    pub fn append(&mut self, other: &mut ScheduleBuilder) -> &mut Self {
        self.steps.append(&mut other.steps);
        self.final_steps.append(&mut other.final_steps);
        self
    }

    /// Builds the schedule.
    pub fn build(&mut self) -> Schedule {
        self.build_with_max_threads(usize::MAX)
    }

    /// Builds the schedule allowing at most `max_threads` systems to run in parallel.
    pub fn build_with_max_threads(&mut self, max_threads: usize) -> Schedule {
        fn step_to_non_conflicting_systems<'a>(
            step: &'a mut ScheduleStep,
            system: &System,
        ) -> Option<&'a mut Vec<System>> {
            match step {
                ScheduleStep::Systems(systems) => {
                    let systems_conflict = systems
                        .iter()
                        .flat_map(|s| s.borrows())
                        .any(|p1| system.borrows().iter().any(|p2| p1.conflicts_with(p2)));

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

        self.steps
            .drain(..)
            .chain(self.final_steps.drain(..))
            .for_each(|step| {
                match step {
                    SimpleScheduleStep::System(system) => {
                        let systems = steps
                            .iter_mut()
                            .rev()
                            .map_while(|step| step_to_non_conflicting_systems(step, &system))
                            .filter(|systsems| systsems.len() < max_threads)
                            .last();

                        match systems {
                            Some(systems) => systems.push(system),
                            None => steps.push(ScheduleStep::Systems(vec![system])),
                        }
                    }
                    SimpleScheduleStep::LocalSystem(system) => {
                        match steps.last_mut() {
                            Some(ScheduleStep::LocalSystems(systems)) => systems.push(system),
                            _ => steps.push(ScheduleStep::LocalSystems(vec![system])),
                        }
                    }
                    SimpleScheduleStep::ExclusiveSystem(local_fn) => {
                        match steps.last_mut() {
                            Some(ScheduleStep::ExclusiveSystems(local_fns)) => {
                                local_fns.push(local_fn)
                            }
                            _ => steps.push(ScheduleStep::ExclusiveSystems(vec![local_fn])),
                        }
                    }
                    SimpleScheduleStep::Barrier => {
                        if matches!(steps.last(), Some(ScheduleStep::Systems(_))) {
                            steps.push(ScheduleStep::Barrier)
                        }
                    }
                }
            });

        Schedule { steps }
    }
}

/// Steps that can be run by a [`Schedule`].
#[derive(Debug)]
pub enum ScheduleStep {
    /// Runs the systems in parallel.
    Systems(Vec<System>),
    /// Runs the local systems sequentially.
    LocalSystems(Vec<LocalSystem>),
    /// Runs the exclusive systems sequentially.
    ExclusiveSystems(Vec<ExclusiveSystem>),
    /// Prevents future systems from running in parallel with previous ones.
    Barrier,
}

/// Schedules systems to run sequentially or in parallel without data conflicts.
#[derive(Debug)]
pub struct Schedule {
    steps: Vec<ScheduleStep>,
}

impl Schedule {
    /// Enables creating a schedule using the builder pattern.
    pub fn builder() -> ScheduleBuilder {
        Default::default()
    }

    /// Registered the storages used by the systems.
    pub fn set_up(&self, world: &mut World) {
        fn register(world: &mut World, param: &SystemBorrow) {
            unsafe {
                match param {
                    SystemBorrow::Comp(c) | SystemBorrow::CompMut(c) => {
                        world.register_with(c.type_id(), || c.create_storage())
                    }
                    _ => (),
                }
            }
        }

        for step in self.steps.iter() {
            match step {
                ScheduleStep::Systems(systems) => {
                    for param in systems.iter().flat_map(System::borrows) {
                        register(world, param);
                    }
                }
                ScheduleStep::LocalSystems(local_systems) => {
                    for param in local_systems.iter().flat_map(LocalSystem::borrows) {
                        register(world, param);
                    }
                }
                _ => (),
            }
        }
    }

    /// Runs the systems in parallel on the global rayon thread pool if parallelism is enabled, or
    /// sequentially otherwise. Calls `World::maintain` before each local system and local function,
    /// after each barrier and right before the function returns.
    pub fn run(&mut self, world: &mut World, resources: &mut Resources) {
        #[cfg(feature = "parallel")]
        self.run_par(world, resources);

        #[cfg(not(feature = "parallel"))]
        self.run_seq(world, resources);
    }

    /// Runs the systems sequentially. Calls `World::maintain` before each local system and local
    /// function, after each barrier and right before the function returns.
    pub fn run_seq(&mut self, world: &mut World, resources: &mut Resources) {
        self.run_generic(world, resources, |systems, world, resources| {
            for system in systems {
                crate::run(world, resources, system);
            }
        });
    }

    /// Runs the systems in parallel on the global rayon thread pool. Calls `World::maintain` before
    /// each local system and local function, after each barrier and right before the function
    /// returns.
    #[cfg(feature = "parallel")]
    pub fn run_par(&mut self, world: &mut World, resources: &mut Resources) {
        use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

        self.run_generic(world, resources, |systems, world, resources| {
            if systems.len() > 1 {
                systems
                    .par_iter_mut()
                    .for_each(|system| crate::run(world, resources, system));
            } else {
                crate::run(world, resources, systems.last_mut().unwrap())
            }
        });
    }

    /// Runs the systems in parallel on the given thread pool. Calls `World::maintain` before each
    /// local system and local function, after each barrier and right before the function
    /// returns.
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
                        crate::run(world, resources, system);
                    });
                });
            } else {
                crate::run(world, resources, systems.last_mut().unwrap());
            }
        });
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
                }
                ScheduleStep::LocalSystems(local_systems) => {
                    for local_system in local_systems.iter_mut() {
                        world.maintain();
                        crate::run_local(world, resources, local_system);
                    }
                }
                ScheduleStep::ExclusiveSystems(exclusive_systems) => {
                    for exclusive_system in exclusive_systems {
                        world.maintain();
                        crate::run_exclusive(world, resources, exclusive_system);
                    }
                }
                ScheduleStep::Barrier => world.maintain(),
            }
        }

        world.maintain();
    }

    /// Returns the maximum number of systems that can run in parallel.
    pub fn max_threads(&self) -> usize {
        fn step_to_system_count(step: &ScheduleStep) -> Option<usize> {
            match step {
                ScheduleStep::Systems(systems) => Some(systems.len()),
                _ => None,
            }
        }

        self.steps
            .iter()
            .flat_map(step_to_system_count)
            .max()
            .unwrap_or(1)
    }

    /// Consumes the schedule and returns the steps comprising it.
    pub fn into_steps(self) -> Vec<ScheduleStep> {
        self.steps
    }
}
