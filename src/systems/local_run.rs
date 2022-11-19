use crate::resources::Resources;
use crate::systems::{BorrowedSystemParam, LocalSystemParam, RunExclusive};
use crate::world::World;

pub trait RunLocal<Params, Return>: RunExclusive<Params, Return> {
    fn param_types(&self) -> Vec<BorrowedSystemParam>;

    fn run_local(self, world: &World, resources: &Resources) -> Return;
}

pub fn run_local<'a, Params, Return>(
    world: &'a World,
    resources: &'a Resources,
    f: impl RunLocal<Params, Return>,
) -> Return {
    f.run_local(world, resources)
}

macro_rules! impl_run_local {
    ($(($lifetime:lifetime, $param:ident)),*) => {
        impl<Func, Return, $($param),*> RunLocal<($($param,)*), Return> for Func
        where
            Func: FnOnce($($param),*) -> Return
                + for<$($lifetime),*> FnOnce($(<$param as LocalSystemParam>::Param<$lifetime>),*) -> Return,
            $($param: LocalSystemParam,)*
        {
            fn param_types(&self) -> Vec<BorrowedSystemParam> {
                vec![$($param::as_borrowed_param()),*]
            }

            #[allow(unused_variables)]
            fn run_local(self, world: &World, resources: &Resources) -> Return {
                self($(<$param as LocalSystemParam>::borrow(world, resources)),*)
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_run_local!();
    impl_run_local!(('a, A));
    impl_run_local!(('a, A), ('b, B));
    impl_run_local!(('a, A), ('b, B), ('c, C));
    impl_run_local!(('a, A), ('b, B), ('c, C), ('d, D));
    impl_run_local!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E));
    impl_run_local!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F));
    impl_run_local!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G));
    impl_run_local!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H));
    impl_run_local!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I));
    impl_run_local!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J));
    impl_run_local!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K));
    impl_run_local!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K), ('l, L));
    impl_run_local!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K), ('l, L), ('m, M));
    impl_run_local!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K), ('l, L), ('m, M), ('n, N));
    impl_run_local!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K), ('l, L), ('m, M), ('n, N), ('o, O));
    impl_run_local!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K), ('l, L), ('m, M), ('n, N), ('o, O), ('p, P));
}
