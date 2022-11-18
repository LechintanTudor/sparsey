use crate::resources::{Resources, SyncResources};
use crate::systems::{Run, RunExclusive, RunLocally, SystemParamType};
use crate::world::World;

type BoxedSystemFn = Box<dyn FnMut(&World, SyncResources) + Send + 'static>;

/// Encapsulates a system that can run on any thread.
pub struct System {
    function: BoxedSystemFn,
    params: Vec<SystemParamType>,
}

impl System {
    /// Returns the system parameter types as a slice.
    pub fn params(&self) -> &[SystemParamType] {
        &self.params
    }
}

impl<'a> RunExclusive<(), ()> for &'a mut System {
    fn run_exclusive(self, world: &mut World, resources: &mut Resources) {
        (self.function)(world, resources.sync())
    }
}

impl<'a> RunLocally<(), ()> for &'a mut System {
    fn param_types(&self) -> Vec<SystemParamType> {
        self.params.clone()
    }

    fn run_locally(self, world: &World, resources: &Resources) {
        (self.function)(world, resources.sync())
    }
}

impl<'a> Run<(), ()> for &'a mut System {
    fn run(self, world: &World, resources: SyncResources) {
        (self.function)(world, resources)
    }
}

/// Helper trait for creating a system from a function.
pub trait IntoSystem<Params> {
    /// Creates a system.
    fn system(self) -> System;
}

impl IntoSystem<()> for System {
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
                let params = self.param_types();

                System {
                    function: Box::new(move |world: &World, resources: SyncResources| {
                        (&mut self).run(world, resources);
                    }),
                    params,
                }
            }
        }
    };
}

impl_into_system!();
impl_into_system!(A);
impl_into_system!(A, B);
impl_into_system!(A, B, C);
impl_into_system!(A, B, C, D);
impl_into_system!(A, B, C, D, E);
impl_into_system!(A, B, C, D, E, F);
impl_into_system!(A, B, C, D, E, F, G);
impl_into_system!(A, B, C, D, E, F, G, H);
impl_into_system!(A, B, C, D, E, F, G, H, I);
impl_into_system!(A, B, C, D, E, F, G, H, I, J);
impl_into_system!(A, B, C, D, E, F, G, H, I, J, K);
impl_into_system!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_into_system!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_into_system!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_into_system!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_into_system!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
