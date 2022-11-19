use crate::resources::Resources;
use crate::systems::RunExclusive;
use crate::world::World;

type BoxedExclusiveSystemFn = Box<dyn FnMut(&mut World, &mut Resources)>;

pub struct ExclusiveSystem {
    system_fn: BoxedExclusiveSystemFn,
}

impl RunExclusive<(), ()> for &'_ mut ExclusiveSystem {
    #[inline]
    fn run_exclusive(self, world: &mut World, resources: &mut Resources) {
        (self.system_fn)(world, resources)
    }
}

pub trait IntoExclusiveSystem<Params> {
    fn exclusive_system(self) -> ExclusiveSystem;
}

impl IntoExclusiveSystem<()> for ExclusiveSystem {
    #[inline]
    fn exclusive_system(self) -> ExclusiveSystem {
        self
    }
}

impl<F, Params> IntoExclusiveSystem<Params> for F
where
    F: RunExclusive<Params, ()> + 'static,
    for<'a> &'a mut F: RunExclusive<Params, ()>,
{
    fn exclusive_system(mut self) -> ExclusiveSystem {
        ExclusiveSystem {
            system_fn: Box::new(move |world, resources| {
                (&mut self).run_exclusive(world, resources);
            }),
        }
    }
}
