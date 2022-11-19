use crate::resources::{Resources, SyncResources};
use crate::systems::{BorrowedSystemParam, Run, RunExclusive, RunLocal};
use crate::world::World;

type BoxedSystemFn = Box<dyn FnMut(&World, SyncResources) + Send + 'static>;

/// Encapsulates a system that can run on any thread.
pub struct System {
    system_fn: BoxedSystemFn,
    borrowed_params: Vec<BorrowedSystemParam>,
}

impl System {
    /// Returns the system parameter types as a slice.
    #[inline]
    pub fn borrowed_params(&self) -> &[BorrowedSystemParam] {
        &self.borrowed_params
    }
}

impl RunExclusive<(), ()> for &'_ mut System {
    #[inline]
    fn run_exclusive(self, world: &mut World, resources: &mut Resources) {
        (self.system_fn)(world, resources.sync())
    }
}

impl RunLocal<(), ()> for &'_ mut System {
    #[inline]
    fn get_borrowed_params(&self) -> Vec<BorrowedSystemParam> {
        self.borrowed_params.clone()
    }

    #[inline]
    fn run_local(self, world: &World, resources: &Resources) {
        (self.system_fn)(world, resources.sync())
    }
}

impl Run<(), ()> for &'_ mut System {
    #[inline]
    fn run(self, world: &World, resources: SyncResources) {
        (self.system_fn)(world, resources)
    }
}

/// Helper trait for creating a system from a function.
pub trait IntoSystem<Params> {
    /// Creates a system.
    fn system(self) -> System;
}

impl IntoSystem<()> for System {
    #[inline]
    fn system(self) -> System {
        self
    }
}

macro_rules! impl_into_system {
    ($($param:ident),*) => {
        impl<Func, $($param),*> IntoSystem<($($param,)*)> for Func
        where
            Func: Run<($($param,)*), ()> + Send + 'static,
            for<'a> &'a mut Func: Run<($($param,)*), ()>,
        {
            fn system(mut self) -> System {
                System {
                    borrowed_params: self.get_borrowed_params(),
                    system_fn: Box::new(move |world: &World, resources: SyncResources| {
                        (&mut self).run(world, resources);
                    }),
                }
            }
        }
    };
}

crate::utils::impl_generic_0_to_16!(impl_into_system);
