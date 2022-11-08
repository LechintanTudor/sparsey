use crate::resources::Resources;
use crate::systems::{GenericSystemParam, LocalSystemParam, RunExclusive, SystemParamType};
use crate::world::World;

pub trait RunLocally<Params, Return>: RunExclusive<Params, Return> {
    fn param_types(&self) -> Vec<SystemParamType>;

    fn run_locally(self, world: &World, resources: &Resources) -> Return;
}

pub fn run_locally<'a, Params, Return>(
    world: &'a World,
    resources: &'a Resources,
    f: impl RunLocally<Params, Return>,
) -> Return {
    f.run_locally(world, resources)
}

macro_rules! impl_run_locally {
    ($(($lifetime:lifetime, $param:ident)),*) => {
        impl<Func, Return, $($param),*> RunExclusive<($($param,)*), Return> for Func
        where
            Func: FnOnce($($param),*) -> Return
                + for<$($lifetime),*> FnOnce($(<$param as GenericSystemParam>::Param<$lifetime>),*) -> Return,
            $($param: LocalSystemParam,)*
        {
            #[allow(unused_variables)]
            fn run_exclusive(self, world: &mut World, resources: &mut Resources) -> Return {
                self($(<$param as LocalSystemParam>::borrow(world, resources)),*)
            }
        }

        impl<Func, Return, $($param),*> RunLocally<($($param,)*), Return> for Func
        where
            Func: FnOnce($($param),*) -> Return
                + for<$($lifetime),*> FnOnce($(<$param as GenericSystemParam>::Param<$lifetime>),*) -> Return,
            $($param: LocalSystemParam,)*
        {
            fn param_types(&self) -> Vec<SystemParamType> {
                vec![$($param::param_type()),*]
            }

            #[allow(unused_variables)]
            fn run_locally(self, world: &World, resources: &Resources) -> Return {
                self($(<$param as LocalSystemParam>::borrow(world, resources)),*)
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_run_locally!();
    impl_run_locally!(('a, A));
    impl_run_locally!(('a, A), ('b, B));
    impl_run_locally!(('a, A), ('b, B), ('c, C));
    impl_run_locally!(('a, A), ('b, B), ('c, C), ('d, D));
    impl_run_locally!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E));
    impl_run_locally!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F));
    impl_run_locally!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G));
    impl_run_locally!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H));
    impl_run_locally!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I));
    impl_run_locally!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J));
    impl_run_locally!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K));
    impl_run_locally!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K), ('l, L));
    impl_run_locally!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K), ('l, L), ('m, M));
    impl_run_locally!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K), ('l, L), ('m, M), ('n, N));
    impl_run_locally!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K), ('l, L), ('m, M), ('n, N), ('o, O));
    impl_run_locally!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K), ('l, L), ('m, M), ('n, N), ('o, O), ('p, P));
}
