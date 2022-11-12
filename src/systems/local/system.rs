use crate::resources::Resources;
use crate::systems::{RunExclusive, RunLocally, SystemParamType};
use crate::world::World;

type BoxedLocalSystemFn = Box<dyn FnMut(&World, &Resources) + 'static>;

/// Encapsulates a system that can run locally.
pub struct LocalSystem {
    function: BoxedLocalSystemFn,
    params: Vec<SystemParamType>,
}

impl LocalSystem {
    /// Returns the system parameter types as a slice.
    pub fn params(&self) -> &[SystemParamType] {
        &self.params
    }
}

impl<'a> RunExclusive<(), ()> for &'a mut LocalSystem {
    fn run_exclusive(self, world: &mut World, resources: &mut Resources) {
        (self.function)(world, resources)
    }
}

impl<'a> RunLocally<(), ()> for &'a mut LocalSystem {
    fn param_types(&self) -> Vec<SystemParamType> {
        self.params.clone()
    }

    fn run_locally(self, world: &World, resources: &Resources) {
        (self.function)(world, resources)
    }
}

/// Helper trait for creating a local system from a function.
pub trait IntoLocalSystem<Params> {
    /// Creates a local system.
    fn local_system(self) -> LocalSystem;
}

impl IntoLocalSystem<()> for LocalSystem {
    fn local_system(self) -> LocalSystem {
        self
    }
}

macro_rules! impl_into_local_system {
    ($($param:ident),*) => {
        impl<Func, $($param),*> IntoLocalSystem<($($param,)*)> for Func
        where
            Func: RunLocally<($($param,)*), ()> + 'static,
            for<'a> &'a mut Func: RunLocally<($($param,)*), ()>,
        {
            fn local_system(mut self) -> LocalSystem {
                let params = self.param_types();

                LocalSystem {
                    function: Box::new(move |world: &World, resources: &Resources| {
                        (&mut self).run_locally(world, resources)
                    }),
                    params,
                }
            }
        }
    };
}

impl_into_local_system!();
impl_into_local_system!(A);
impl_into_local_system!(A, B);
impl_into_local_system!(A, B, C);
impl_into_local_system!(A, B, C, D);
impl_into_local_system!(A, B, C, D, E);
impl_into_local_system!(A, B, C, D, E, F);
impl_into_local_system!(A, B, C, D, E, F, G);
impl_into_local_system!(A, B, C, D, E, F, G, H);
impl_into_local_system!(A, B, C, D, E, F, G, H, I);
impl_into_local_system!(A, B, C, D, E, F, G, H, I, J);
impl_into_local_system!(A, B, C, D, E, F, G, H, I, J, K);
impl_into_local_system!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_into_local_system!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_into_local_system!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_into_local_system!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_into_local_system!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
