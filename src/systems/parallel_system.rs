use crate::resources::{Resources, SyncResources};
use crate::systems::{Run, RunExclusive, RunLocal, SystemBorrow};
use crate::world::World;

type BoxedSystemFn = Box<dyn FnMut(&World, SyncResources) + Send + 'static>;

/// Encapsulates a system that can run on any thread.
pub struct System {
    system_fn: BoxedSystemFn,
    borrows: Vec<SystemBorrow>,
}

impl System {
    /// Returns the assets borrowed by the system during execution as a slice.
    #[inline]
    pub fn borrows(&self) -> &[SystemBorrow] {
        &self.borrows
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
    fn get_borrows(&self) -> Vec<SystemBorrow> {
        self.borrows.clone()
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

/// Helper trait for creating a [`System`] from a function.
pub trait IntoSystem<Params> {
    /// Creates the system.
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
                    borrows: self.get_borrows(),
                    system_fn: Box::new(move |world: &World, resources: SyncResources| {
                        (&mut self).run(world, resources);
                    }),
                }
            }
        }
    };
}

crate::utils::impl_generic_0_to_16!(impl_into_system);
