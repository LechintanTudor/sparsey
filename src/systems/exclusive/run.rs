use crate::resources::Resources;
use crate::systems::{GenericSystemParam, LocalSystemParam};
use crate::world::World;

pub trait RunExclusive<Params, Return> {
    fn run_exclusive(self, world: &mut World, resources: &mut Resources) -> Return;
}

impl<F, Return> RunExclusive<(&mut World, &mut Resources), Return> for F
where
    F: FnOnce(&mut World, &mut Resources) -> Return,
{
    fn run_exclusive(self, world: &mut World, resources: &mut Resources) -> Return {
        self(world, resources)
    }
}

impl<F, Return> RunExclusive<(&mut Resources, &mut World), Return> for F
where
    F: FnOnce(&mut Resources, &mut World) -> Return,
{
    fn run_exclusive(self, world: &mut World, resources: &mut Resources) -> Return {
        self(resources, world)
    }
}

impl<F, Return> RunExclusive<(&mut World,), Return> for F
where
    F: FnOnce(&mut World) -> Return,
{
    fn run_exclusive(self, world: &mut World, _resources: &mut Resources) -> Return {
        self(world)
    }
}

impl<F, Return> RunExclusive<(&mut Resources,), Return> for F
where
    F: FnOnce(&mut Resources) -> Return,
{
    fn run_exclusive(self, _world: &mut World, resources: &mut Resources) -> Return {
        self(resources)
    }
}

pub fn run_exclusive<Params, Return>(
    world: &mut World,
    resources: &mut Resources,
    system: impl RunExclusive<Params, Return>,
) -> Return {
    system.run_exclusive(world, resources)
}

macro_rules! impl_run_exclusive {
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
    };
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_run_exclusive!();
    impl_run_exclusive!(('a, A));
    impl_run_exclusive!(('a, A), ('b, B));
    impl_run_exclusive!(('a, A), ('b, B), ('c, C));
    impl_run_exclusive!(('a, A), ('b, B), ('c, C), ('d, D));
    impl_run_exclusive!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E));
    impl_run_exclusive!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F));
    impl_run_exclusive!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G));
    impl_run_exclusive!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H));
    impl_run_exclusive!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I));
    impl_run_exclusive!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J));
    impl_run_exclusive!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K));
    impl_run_exclusive!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K), ('l, L));
    impl_run_exclusive!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K), ('l, L), ('m, M));
    impl_run_exclusive!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K), ('l, L), ('m, M), ('n, N));
    impl_run_exclusive!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K), ('l, L), ('m, M), ('n, N), ('o, O));
    impl_run_exclusive!(('a, A), ('b, B), ('c, C), ('d, D), ('e, E), ('f, F), ('g, G), ('h, H), ('i, I), ('j, J), ('k, K), ('l, L), ('m, M), ('n, N), ('o, O), ('p, P));
}
