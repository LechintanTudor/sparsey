use crate::resources::SyncResources;
use crate::systems::{LocalSystemParam, RunLocal, SystemParam};
use crate::world::World;

/// Helper trait for executing systems with shared access to [`World`] and [`SyncResources`].
pub trait Run<Params, Return>: RunLocal<Params, Return> {
    /// Executes the system in the provided context.
    fn run(self, world: &World, resources: SyncResources) -> Return;
}

/// Executes a system in the provided context.
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
    ($($param:ident),*) => {
        impl<Func, Return, $($param),*> Run<($($param,)*), Return> for Func
        where
            Func: FnOnce($($param),*) -> Return
                + FnOnce($(<$param as LocalSystemParam>::Param<'_>),*) -> Return,
            $($param: SystemParam,)*
        {
            #[allow(unused_variables)]
            fn run(self, world: &World, resources: SyncResources) -> Return {
                self($(<$param as SystemParam>::borrow(world, resources)),*)
            }
        }
    };
}

crate::utils::impl_generic_0_to_16!(impl_run);
