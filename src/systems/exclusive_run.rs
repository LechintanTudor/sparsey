use crate::resources::Resources;
use crate::systems::{LocalSystemData, SystemDataDescriptor};
use crate::world::World;

/// Helper trait for executing systems that require exclusive access to [`World`] and [`Resources`].
pub trait RunExclusive<Params, Return> {
    /// Executes the system in the provided context.
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

/// Executes an exclusive system in the provided context.
pub fn run_exclusive<Params, Return>(
    world: &mut World,
    resources: &mut Resources,
    system: impl RunExclusive<Params, Return>,
) -> Return {
    system.run_exclusive(world, resources)
}

macro_rules! impl_run_exclusive {
    ($($param:ident),*) => {
        impl<Func, Return, $($param),*> RunExclusive<($($param,)*), Return> for Func
        where
            Func: FnOnce($($param),*) -> Return
                + FnOnce($(<$param as SystemDataDescriptor>::SystemData<'_>),*) -> Return,
            $($param: LocalSystemData,)*
        {
            #[allow(unused_variables)]
            fn run_exclusive(self, world: &mut World, resources: &mut Resources) -> Return {
                self($(<$param as LocalSystemData>::borrow(world, resources)),*)
            }
        }
    };
}

crate::utils::impl_generic_0_to_16!(impl_run_exclusive);
