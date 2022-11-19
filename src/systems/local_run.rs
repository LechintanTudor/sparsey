use crate::resources::Resources;
use crate::systems::{BorrowedSystemParam, LocalSystemParam, RunExclusive};
use crate::world::World;

pub trait RunLocal<Params, Return>: RunExclusive<Params, Return> {
    fn get_borrowed_params(&self) -> Vec<BorrowedSystemParam>;

    fn run_local(self, world: &World, resources: &Resources) -> Return;
}

pub fn run_local<Params, Return>(
    world: &World,
    resources: &Resources,
    f: impl RunLocal<Params, Return>,
) -> Return {
    f.run_local(world, resources)
}

macro_rules! impl_run_local {
    ($($param:ident),*) => {
        impl<Func, Return, $($param),*> RunLocal<($($param,)*), Return> for Func
        where
            Func: FnOnce($($param),*) -> Return
                + FnOnce($(<$param as LocalSystemParam>::Param<'_>),*) -> Return,
            $($param: LocalSystemParam,)*
        {
            fn get_borrowed_params(&self) -> Vec<BorrowedSystemParam> {
                vec![$($param::as_borrowed_param()),*]
            }

            #[allow(unused_variables)]
            fn run_local(self, world: &World, resources: &Resources) -> Return {
                self($(<$param as LocalSystemParam>::borrow(world, resources)),*)
            }
        }
    };
}

crate::utils::impl_generic_0_to_16!(impl_run_local);
