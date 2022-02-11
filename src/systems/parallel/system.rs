use crate::resources::SyncResources;
use crate::systems::{BorrowLocalSystemData, BorrowSystemData, SystemParam, SystemParamType};
use crate::world::World;

/// Encapsulates a system that can run on any thread.
pub struct System {
    function: Box<dyn FnMut(&World, SyncResources) + Send + 'static>,
    params: Vec<SystemParamType>,
}

impl System {
    /// Returns the system parameter types as a slice.
    pub fn params(&self) -> &[SystemParamType] {
        &self.params
    }

    /// Runs the system.
    pub fn run(&mut self, world: &World, resources: SyncResources) {
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
            Func: Send + 'static,
            for<'a> &'a mut Func: FnMut($($param),*)
                + FnMut($(<$param as BorrowLocalSystemData>::Item),*)
                + Send,
            $($param: SystemParam,)*
        {
            fn system(mut self) -> System {
                #[allow(clippy::too_many_arguments, non_snake_case)]
                fn call_inner<$($param),*>(
                    mut f: impl FnMut($($param),*) + Send,
                    $($param: $param),*
                ) {
                    f($($param),*)
                }

                #[allow(unused_variables)]
                let function = Box::new(move |world: &World, resources: SyncResources| {
                    call_inner(
                        &mut self,
                        $(<$param as BorrowSystemData>::borrow(world, resources)),*
                    )
                });

                let params = vec![$($param::param_type(),)*];

                System { function, params }
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
