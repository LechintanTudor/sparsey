use crate::resources::Resources;
use crate::systems::RunExclusive;
use crate::world::World;
use std::fmt;

type BoxedExclusiveSystemFn = Box<dyn FnMut(&mut World, &mut Resources)>;

/// Encapsulates a system that requires exclusive access to [`World`] and [`Resources`].
pub struct ExclusiveSystem {
    system_fn: BoxedExclusiveSystemFn,
}

impl fmt::Debug for ExclusiveSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExclusiveSystem").finish_non_exhaustive()
    }
}

impl RunExclusive<(), ()> for &'_ mut ExclusiveSystem {
    /// Executes the system in the provided context.
    #[inline]
    fn run_exclusive(self, world: &mut World, resources: &mut Resources) {
        (self.system_fn)(world, resources)
    }
}

/// Helper trait for creating an [`ExclusiveSystem`] from a function.
pub trait IntoExclusiveSystem<Params> {
    /// Creates the exclusive system.
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
