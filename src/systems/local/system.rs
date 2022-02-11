use crate::resources::Resources;
use crate::systems::{BorrowLocalSystemData, SystemParam, SystemParamType};
use crate::world::World;

/// Encapsulates a system that can run locally.
pub struct LocalSystem {
    function: Box<dyn FnMut(&World, &Resources) + 'static>,
    params: Vec<SystemParamType>,
}

impl LocalSystem {
    /// Returns the system parameter types as a slice.
    pub fn params(&self) -> &[SystemParamType] {
        &self.params
    }

    /// Runs the system.
    pub fn run(&mut self, world: &World, resources: &Resources) {
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

macro_rules! impl_into_system {
    ($($param:ident),*) => {
        impl<Func, $($param),*> IntoLocalSystem<($($param,)*)> for Func
        where
            Func: Send + 'static,
            for<'a> &'a mut Func: FnMut($($param),*)
                + FnMut($(<$param as BorrowLocalSystemData>::Item),*)
                + Send,
            $($param: SystemParam,)*
        {
            fn local_system(mut self) -> LocalSystem {
                #[allow(clippy::too_many_arguments, non_snake_case)]
                fn call_inner<$($param),*>(
                    mut f: impl FnMut($($param),*) + Send,
                    $($param: $param),*
                ) {
                    f($($param),*)
                }

                #[allow(unused_variables)]
                let function = Box::new(move |world: &World, resources: &Resources| {
                    call_inner(
                        &mut self,
                        $(<$param as BorrowLocalSystemData>::borrow(world, resources)),*
                    )
                });

                let params = vec![$($param::param_type(),)*];

                LocalSystem { function, params }
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
