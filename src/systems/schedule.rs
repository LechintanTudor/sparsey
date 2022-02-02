use crate::resources::Resources;
use crate::systems::{IntoLocalSystem, IntoSystem, LocalSystem, System, SystemParamType};
use crate::world::World;

const DEFAULT_MAX_SYSTEMS_PER_STAGE: usize = 5;

pub enum ScheduleStage {
    Systems(Vec<System>),
    LocalSystems(Vec<LocalSystem>),
    LocalFns(Vec<Box<dyn FnMut(&mut World, &mut Resources)>>),
    Barrier,
}

pub struct ScheduleBuilder {
    max_systems_per_stage: usize,
    stages: Vec<ScheduleStage>,
    final_stages: Vec<ScheduleStage>,
}

impl ScheduleBuilder {
    pub fn set_max_systems_per_stage(&mut self, max_systems_per_stage: usize) -> &mut Self {
        self.max_systems_per_stage = max_systems_per_stage.max(1);
        self
    }

    pub fn add_system<P, R>(&mut self, system: impl IntoSystem<P, R>) -> &mut Self {
        let system = system.system();

        fn stage_to_systems(stage: &mut ScheduleStage) -> Option<&mut Vec<System>> {
            match stage {
                ScheduleStage::Systems(systems) => Some(systems),
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
            .stages
            .iter_mut()
            .rev()
            .flat_map(stage_to_systems)
            .take_while(|other_systems| no_params_conflict(other_systems, &system))
            .fuse()
            .filter(|systems| systems.len() < self.max_systems_per_stage)
            .min_by_key(|systems| systems.len());

        match systems {
            Some(systems) => systems.push(system),
            None => self.stages.push(ScheduleStage::Systems(vec![system])),
        }

        self
    }

    pub fn add_local_system<P, R>(&mut self, system: impl IntoLocalSystem<P, R>) -> &mut Self {
        let system = system.local_system();

        match self.final_stages.last_mut() {
            Some(ScheduleStage::LocalSystems(systems)) => systems.push(system),
            _ => self.final_stages.push(ScheduleStage::LocalSystems(vec![system])),
        };

        self
    }

    pub fn add_local_fn(
        &mut self,
        local_fn: impl FnMut(&mut World, &mut Resources) + 'static,
    ) -> &mut Self {
        let local_fn = Box::new(local_fn);

        match self.final_stages.last_mut() {
            Some(ScheduleStage::LocalFns(fns)) => fns.push(local_fn),
            _ => self.final_stages.push(ScheduleStage::LocalFns(vec![local_fn])),
        }

        self
    }

    pub fn add_barrier(&mut self) -> &mut Self {
        if matches!(self.stages.last(), Some(ScheduleStage::Systems(_))) {
            self.stages.push(ScheduleStage::Barrier);
        }

        self
    }

    pub fn add_barrier_system<P, R>(&mut self, system: impl IntoLocalSystem<P, R>) -> &mut Self {
        let system = system.local_system();

        match self.stages.last_mut() {
            Some(ScheduleStage::LocalSystems(systems)) => systems.push(system),
            _ => self.stages.push(ScheduleStage::LocalSystems(vec![system])),
        };

        self
    }

    pub fn add_barrier_fn(
        &mut self,
        local_fn: impl FnMut(&mut World, &mut Resources) + 'static,
    ) -> &mut Self {
        let local_fn = Box::new(local_fn);

        match self.stages.last_mut() {
            Some(ScheduleStage::LocalFns(fns)) => fns.push(local_fn),
            _ => self.stages.push(ScheduleStage::LocalFns(vec![local_fn])),
        }

        self
    }

    pub fn build(&mut self) -> Schedule {
        let mut stages = std::mem::take(&mut self.stages);
        let mut final_stages = std::mem::take(&mut self.final_stages);
        stages.append(&mut final_stages);

        Schedule { stages }
    }
}

pub struct Schedule {
    stages: Vec<ScheduleStage>,
}

impl Schedule {
    pub fn builder() -> ScheduleBuilder {
        ScheduleBuilder {
            max_systems_per_stage: DEFAULT_MAX_SYSTEMS_PER_STAGE,
            stages: Default::default(),
            final_stages: Default::default(),
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

        for stage in self.stages.iter() {
            match stage {
                ScheduleStage::Systems(systems) => {
                    for param in systems.iter().flat_map(|s| s.params()) {
                        register(world, param);
                    }
                }
                ScheduleStage::LocalSystems(systems) => {
                    for param in systems.iter().flat_map(|s| s.params()) {
                        register(world, param);
                    }
                }
                _ => (),
            }
        }
    }

    pub fn run(&mut self, world: &mut World, resources: &mut Resources) {
        for stage in self.stages.iter_mut() {
            match stage {
                ScheduleStage::Systems(systems) => {
                    let sync_resources = resources.sync();

                    for system in systems.iter_mut() {
                        system.run(world, sync_resources);
                    }
                }
                ScheduleStage::LocalSystems(systems) => {
                    world.maintain();

                    for system in systems.iter_mut() {
                        system.run(world, resources);
                    }
                }
                ScheduleStage::LocalFns(fns) => {
                    world.maintain();

                    for local_fn in fns {
                        (local_fn)(world, resources);
                    }
                }
                ScheduleStage::Barrier => world.maintain(),
            }
        }

        world.maintain();
    }
}
