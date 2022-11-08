use crate::resources::Resources;
use crate::world::World;

pub trait RunExclusive<Params, Return> {
    fn run_exclusive(self, world: &mut World, resources: &mut Resources) -> Return;
}

impl<F, Return> RunExclusive<(&mut World, &mut Resources), Return> for F
where
    F: FnOnce(&mut World, &mut Resources) -> Return,
{
    fn run_exclusive(self, world: &mut World, resources: &mut Resources) -> Return {
        self(world, resources)
    }
}

impl<F, Return> RunExclusive<(&mut Resources, &mut World), Return> for F
where
    F: FnOnce(&mut Resources, &mut World) -> Return,
{
    fn run_exclusive(self, world: &mut World, resources: &mut Resources) -> Return {
        self(resources, world)
    }
}

impl<F, Return> RunExclusive<(&mut World,), Return> for F
where
    F: FnOnce(&mut World) -> Return,
{
    fn run_exclusive(self, world: &mut World, _resources: &mut Resources) -> Return {
        self(world)
    }
}

impl<F, Return> RunExclusive<(&mut Resources,), Return> for F
where
    F: FnOnce(&mut Resources) -> Return,
{
    fn run_exclusive(self, _world: &mut World, resources: &mut Resources) -> Return {
        self(resources)
    }
}

pub fn run_exclusive<Params, Return>(
    world: &mut World,
    resources: &mut Resources,
    system: impl RunExclusive<Params, Return>,
) -> Return {
    system.run_exclusive(world, resources)
}
