use crate::resources::SyncResources;
use crate::systems::{GenericSystemParam, RunLocally, SystemParam};
use crate::world::World;

pub trait Run<Params, Return>: RunLocally<Params, Return> {
    fn run(self, world: &World, resources: SyncResources) -> Return;
}

pub fn run<'a, R, Params, Return>(
    world: &'a World,
    resources: R,
    f: impl Run<Params, Return>,
) -> Return
where
    R: Into<SyncResources<'a>>,
{
    f.run(world, resources.into())
}

macro_rules! impl_run {
    ($(($lifetime:lifetime, $param:ident)),*) => {
        impl<Func, Return, $($param),*> Run<($($param,)*), Return> for Func
        where
            Func: FnOnce($($param),*) -> Return
                + for<$($lifetime),*> FnOnce($(<$param as GenericSystemParam>::Param<$lifetime>),*) -> Return,
            $($param: SystemParam,)*
        {
            #[allow(unused_variables)]
            fn run(self, world: &World, resources: SyncResources) -> Return {
                self($(<$param as SystemParam>::borrow(world, resources)),*)
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_run!();
    impl_run!(('a, A));
    impl_run!(('a, A), ('b, B));
    impl_run!(('a, A), ('b, B), ('c, C));
    impl_run!(('a, A), ('b, B), ('c, C), ('d, D));
    impl_run!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E));
    impl_run!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F));
    impl_run!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G));
    impl_run!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H));
    impl_run!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I));
    impl_run!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J));
    impl_run!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K));
    impl_run!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K), ('l, L));
    impl_run!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K), ('l, L), ('m, M));
    impl_run!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K), ('l, L), ('m, M), ('n, N));
    impl_run!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K), ('l, L), ('m, M), ('n, N), ('o, O));
    impl_run!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K), ('l, L), ('m, M), ('n, N), ('o, O), ('p, P));
}
