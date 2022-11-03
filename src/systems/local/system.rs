use crate::resources::Resources;
use crate::systems::{LocalSystemParam, SystemParamType};
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
    ($(($lifetime:lifetime, $param:ident)),*) => {
        impl<Func, $($param),*> IntoLocalSystem<($($param,)*)> for Func
        where
            Func: FnMut($($param),*)
                + for<$($lifetime),*> FnMut($(<$param as LocalSystemParam>::Param<$lifetime>),*)
                + 'static,
            $($param: LocalSystemParam,)*
        {
            fn local_system(mut self) -> LocalSystem {
                #[allow(unused_variables)]
                let function = Box::new(move |world: &World, resources: &Resources| {
                    (self)($(<$param as LocalSystemParam>::borrow(world, resources)),*);
                });

                let params = vec![$($param::param_type(),)*];

                LocalSystem { function, params }
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_into_system!();
    impl_into_system!(('a, A));
    impl_into_system!(('a, A), ('b, B));
    impl_into_system!(('a, A), ('b, B), ('c, C));
    impl_into_system!(('a, A), ('b, B), ('c, C), ('d, D));
    impl_into_system!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E));
    impl_into_system!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F));
    impl_into_system!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G));
    impl_into_system!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H));
    impl_into_system!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I));
    impl_into_system!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J));
    impl_into_system!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K));
    impl_into_system!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K), ('l, L));
    impl_into_system!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K), ('l, L), ('m, M));
    impl_into_system!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K), ('l, L), ('m, M), ('n, N));
    impl_into_system!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K), ('l, L), ('m, M), ('n, N), ('o, O));
}
