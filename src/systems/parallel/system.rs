use crate::resources::SyncResources;
use crate::systems::{LocalSystemParam, SystemParam, SystemParamType};
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
    ($(($lifetime:lifetime, $param:ident)),*) => {
        impl<Func, $($param),*> IntoSystem<($($param,)*)> for Func
        where
            Func: FnMut($($param),*)
                + for<$($lifetime),*> FnMut($(<$param as LocalSystemParam>::Param<$lifetime>),*)
                + Send
                + 'static,
            $($param: SystemParam,)*
        {
            fn system(mut self) -> System {
                #[allow(unused_variables)]
                let function = Box::new(move |world: &World, resources: SyncResources| {
                    self($(<$param as SystemParam>::borrow(world, resources)),*);
                });

                let params = vec![$($param::param_type(),)*];

                System { function, params }
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
