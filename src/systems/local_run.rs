use crate::resources::Resources;
use crate::systems::{LocalSystemParam, RunExclusive, SystemBorrow};
use crate::world::World;

/// Helper trait for executing systems that borrow non-thread-safe resources, with shared access to
/// [`World`] and [`Resources`].
pub trait RunLocal<Params, Return>: RunExclusive<Params, Return> {
    /// Returns the assets borrowed by the system during execution.
    fn get_borrows(&self) -> Vec<SystemBorrow>;

    /// Executes the system in the provided context.
    fn run_local(self, world: &World, resources: &Resources) -> Return;
}

/// Executes a local system in the provided context.
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
            fn get_borrows(&self) -> Vec<SystemBorrow> {
                vec![$($param::as_system_borrow()),*]
            }

            #[allow(unused_variables)]
            fn run_local(self, world: &World, resources: &Resources) -> Return {
                self($(<$param as LocalSystemParam>::borrow(world, resources)),*)
            }
        }
    };
}

crate::utils::impl_generic_0_to_16!(impl_run_local);
