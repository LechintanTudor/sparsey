use crate::resources::{Resources, SyncResources};
use crate::systems::{
    IntoLocalFn, IntoLocalSystem, IntoSystem, LocalFn, LocalSystem, System, SystemParamType,
};
use crate::world::World;

const DEFAULT_MAX_SYSTEMS_PER_STEP: usize = 5;

pub enum ScheduleStep {
    Systems(Vec<System>),
    LocalSystems(Vec<LocalSystem>),
    LocalFns(Vec<LocalFn>),
    Barrier,
}

pub struct ScheduleBuilder {
    max_systems_per_step: usize,
    steps: Vec<ScheduleStep>,
    final_steps: Vec<ScheduleStep>,
}

impl ScheduleBuilder {
    pub fn set_max_systems_per_step(&mut self, max_systems_per_step: usize) -> &mut Self {
        self.max_systems_per_step = max_systems_per_step.max(1);
        self
    }

    pub fn add_system<P, R>(&mut self, system: impl IntoSystem<P, R>) -> &mut Self {
        let system = system.system();

        fn step_to_systems(step: &mut ScheduleStep) -> Option<&mut Vec<System>> {
            match step {
                ScheduleStep::Systems(systems) => Some(systems),
                _ => None,
            }
        }

        fn no_params_conflict(other_systems: &[System], system: &System) -> bool {
            !other_systems
                .iter()
                .flat_map(|s| s.params())
                .any(|p1| system.params().iter().any(|p2| p1.conflicts_with(p2)))
        }

        let systems = self
            .steps
            .iter_mut()
            .rev()
            .flat_map(step_to_systems)
            .take_while(|other_systems| no_params_conflict(other_systems, &system))
            .fuse()
            .filter(|systems| systems.len() < self.max_systems_per_step)
            .min_by_key(|systems| systems.len());

        match systems {
            Some(systems) => systems.push(system),
            None => self.steps.push(ScheduleStep::Systems(vec![system])),
        }

        self
    }

    pub fn add_local_system<P, R>(&mut self, system: impl IntoLocalSystem<P, R>) -> &mut Self {
        let system = system.local_system();

        match self.final_steps.last_mut() {
            Some(ScheduleStep::LocalSystems(systems)) => systems.push(system),
            _ => self.final_steps.push(ScheduleStep::LocalSystems(vec![system])),
        };

        self
    }

    pub fn add_local_fn<R>(&mut self, local_fn: impl IntoLocalFn<R>) -> &mut Self {
        let local_fn = local_fn.local_fn();

        match self.final_steps.last_mut() {
            Some(ScheduleStep::LocalFns(fns)) => fns.push(local_fn),
            _ => self.final_steps.push(ScheduleStep::LocalFns(vec![local_fn])),
        }

        self
    }

    pub fn add_barrier(&mut self) -> &mut Self {
        if matches!(self.steps.last(), Some(ScheduleStep::Systems(_))) {
            self.steps.push(ScheduleStep::Barrier);
        }

        self
    }

    pub fn add_barrier_system<P, R>(&mut self, system: impl IntoLocalSystem<P, R>) -> &mut Self {
        let system = system.local_system();

        match self.steps.last_mut() {
            Some(ScheduleStep::LocalSystems(systems)) => systems.push(system),
            _ => self.steps.push(ScheduleStep::LocalSystems(vec![system])),
        };

        self
    }

    pub fn add_barrier_fn(
        &mut self,
        local_fn: impl FnMut(&mut World, &mut Resources) + 'static,
    ) -> &mut Self {
        let local_fn = local_fn.local_fn();

        match self.steps.last_mut() {
            Some(ScheduleStep::LocalFns(fns)) => fns.push(local_fn),
            _ => self.steps.push(ScheduleStep::LocalFns(vec![local_fn])),
        }

        self
    }

    pub fn build(&mut self) -> Schedule {
        let mut steps = std::mem::take(&mut self.steps);
        let mut final_steps = std::mem::take(&mut self.final_steps);
        steps.append(&mut final_steps);

        Schedule { steps }
    }
}

pub struct Schedule {
    steps: Vec<ScheduleStep>,
}

impl Schedule {
    pub fn builder() -> ScheduleBuilder {
        ScheduleBuilder {
            max_systems_per_step: DEFAULT_MAX_SYSTEMS_PER_STEP,
            steps: Default::default(),
            final_steps: Default::default(),
        }
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

    pub fn max_threads(&self) -> usize {
        fn step_to_system_count(step: &ScheduleStep) -> Option<usize> {
            match step {
                ScheduleStep::Systems(systems) => Some(systems.len()),
                _ => None,
            }
        }

        self.steps.iter().flat_map(step_to_system_count).max().unwrap_or(1)
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
                for system in systems.iter_mut() {
                    system.run(world, resources);
                }
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
                for system in systems.iter_mut() {
                    system.run(world, resources);
                }
            }
        })
    }

    pub fn into_steps(self) -> Vec<ScheduleStep> {
        self.steps
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
                ScheduleStep::Barrier => world.maintain(),
            }
        }
    }
}
