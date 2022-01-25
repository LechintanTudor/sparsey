use crate::systems::SystemParamType;
use crate::world::SyncWorld;

pub struct System {
    function: Box<dyn FnMut(&SyncWorld) + Send + 'static>,
    param_types: Box<[SystemParamType]>,
}

impl System {
    pub fn param_types(&self) -> &[SystemParamType] {
        &self.param_types
    }

    pub fn run(&mut self, world: &SyncWorld) {
        (self.function)(world)
    }
}
