use crate::resources::Resources;
use crate::systems::RunExclusive;
use crate::world::World;

pub struct ExclusiveSystem {
    function: Box<dyn FnMut(&mut World, &mut Resources)>,
}

impl<'a> RunExclusive<(), ()> for &'a mut ExclusiveSystem {
    fn run_exclusive(self, world: &mut World, resources: &mut Resources) {
        (self.function)(world, resources)
    }
}

pub trait IntoExclusiveSystem<Params> {
    fn exclusive_system(self) -> ExclusiveSystem;
}

impl<F, Params> IntoExclusiveSystem<Params> for F
where
    F: 'static,
    for<'a> &'a mut F: RunExclusive<Params, ()>,
{
    fn exclusive_system(mut self) -> ExclusiveSystem {
        ExclusiveSystem {
            function: Box::new(move |world, resources| {
                (&mut self).run_exclusive(world, resources);
            }),
        }
    }
}
