use rustc_hash::FxHashMap;

use crate::systems::{IntoSystem, System};

type SystemId = usize;

struct Group {
    systems: Vec<System>,
}

struct Stage {
    groups: Vec<Group>,
}

pub struct ScheduleBuilder {
    system_id: SystemId,
    system_map: FxHashMap<String, SystemId>,
    barrier: usize,
    stages: Vec<Stage>,
}

impl ScheduleBuilder {
    pub fn add<Params, Return>(
        &mut self,
        system: impl IntoSystem<Params, Return>,
        name: &str,
        dependencies: &[&str],
    ) -> &mut Self {
        let system_id = self.next_system_id();

        let dependencies = dependencies
            .iter()
            .map(|&d| *self.system_map.get(d).expect("Invalid dependency"))
            .collect::<Vec<_>>();

        self
    }

    fn next_system_id(&mut self) -> SystemId {
        let system_id = self.system_id;
        self.system_id += 1;
        system_id
    }
}

pub struct Schedule {}
