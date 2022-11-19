use crate::resources::Resources;
use crate::systems::{BorrowedSystemParam, RunExclusive, RunLocal};
use crate::world::World;

type BoxedLocalSystemFn = Box<dyn FnMut(&World, &Resources) + 'static>;

/// Encapsulates a system that can run locally.
pub struct LocalSystem {
    system_fn: BoxedLocalSystemFn,
    borrowed_params: Vec<BorrowedSystemParam>,
}

impl LocalSystem {
    /// Returns the system parameter types as a slice.
    #[inline]
    pub fn borrowed_params(&self) -> &[BorrowedSystemParam] {
        &self.borrowed_params
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
    fn get_borrowed_params(&self) -> Vec<BorrowedSystemParam> {
        self.borrowed_params.clone()
    }

    #[inline]
    fn run_local(self, world: &World, resources: &Resources) {
        (self.system_fn)(world, resources)
    }
}

/// Helper trait for creating a local system from a function.
pub trait IntoLocalSystem<Params> {
    /// Creates a local system.
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
                    borrowed_params: self.get_borrowed_params(),
                    system_fn: Box::new(move |world: &World, resources: &Resources| {
                        (&mut self).run_local(world, resources)
                    }),
                }
            }
        }
    };
}

crate::utils::impl_generic_0_to_16!(impl_into_local_system);
