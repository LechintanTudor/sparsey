use crate::resources::Resources;
use crate::systems::{RunExclusive, RunLocal, SystemBorrow};
use crate::world::World;
use std::fmt;

type BoxedLocalSystemFn = Box<dyn FnMut(&World, &Resources) + 'static>;

/// Encapsulates a system that borrows non-thread-safe resources.
pub struct LocalSystem {
    system_fn: BoxedLocalSystemFn,
    borrows: Vec<SystemBorrow>,
}

impl fmt::Debug for LocalSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LocalSystem").field("borrows", &self.borrows).finish_non_exhaustive()
    }
}

impl LocalSystem {
    /// Returns the assets borrowed by the system during execution as a slice.
    #[inline]
    pub fn borrows(&self) -> &[SystemBorrow] {
        &self.borrows
    }
}

impl RunExclusive<(), ()> for &'_ mut LocalSystem {
    #[inline]
    fn run_exclusive(self, world: &mut World, resources: &mut Resources) {
        (self.system_fn)(world, resources)
    }
}

impl RunLocal<(), ()> for &'_ mut LocalSystem {
    #[inline]
    fn get_borrows(&self) -> Vec<SystemBorrow> {
        self.borrows.clone()
    }

    #[inline]
    fn run_local(self, world: &World, resources: &Resources) {
        (self.system_fn)(world, resources)
    }
}

/// Helper trait for creating a [`LocalSystem`] from a function.
pub trait IntoLocalSystem<Params> {
    /// Creates the local system.
    fn local_system(self) -> LocalSystem;
}

impl IntoLocalSystem<()> for LocalSystem {
    #[inline]
    fn local_system(self) -> LocalSystem {
        self
    }
}

macro_rules! impl_into_local_system {
    ($($param:ident),*) => {
        impl<Func, $($param),*> IntoLocalSystem<($($param,)*)> for Func
        where
            Func: RunLocal<($($param,)*), ()> + 'static,
            for<'a> &'a mut Func: RunLocal<($($param,)*), ()>,
        {
            fn local_system(mut self) -> LocalSystem {
                LocalSystem {
                    borrows: self.get_borrows(),
                    system_fn: Box::new(move |world: &World, resources: &Resources| {
                        (&mut self).run_local(world, resources)
                    }),
                }
            }
        }
    };
}

crate::utils::impl_generic_0_to_16!(impl_into_local_system);
