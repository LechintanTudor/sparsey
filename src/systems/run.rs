use crate::resources::{Resources, SyncResources};
use crate::systems::{
    LocalSystemData, ResourcesSystemData, SyncResourcesSystemData, SystemBorrows, SystemData,
    SystemDataDescriptor, WorldSystemData,
};
use crate::utils::impl_generic_0_to_16;
use crate::world::World;

/// Trait implemented by systems that can run using only data borrowed from [`World`].
pub trait RunInWorld<TParams, TReturn>: SystemBorrows<TParams, TReturn> {
    /// Runs the system.
    fn run(self, world: &World) -> TReturn;
}

/// Trait implemented by systems that can run using only data borrowed from [`Resources`].
pub trait RunInResources<TParams, TReturn>: SystemBorrows<TParams, TReturn> {
    /// Runs the system.
    fn run(self, resources: &Resources) -> TReturn;
}

/// Trait implemented by systems that can run using only data borrowed from [`SyncResources`].
pub trait RunInSyncResources<TParams, TReturn>: RunInResources<TParams, TReturn> {
    /// Runs the system.
    fn run(self, resources: SyncResources<'_>) -> TReturn;
}

/// Trait implemented by systems that can run on the local thread.
pub trait RunLocalSystem<TParams, TReturn>: SystemBorrows<TParams, TReturn> {
    /// Runs the system.
    fn run(self, world: &World, resources: &Resources) -> TReturn;
}

/// Trait implemented by systems that can run on any thread.
pub trait RunSystem<TParams, TReturn>: RunLocalSystem<TParams, TReturn> {
    /// Runs the system.
    fn run(self, world: &World, resources: SyncResources<'_>) -> TReturn;
}

/// Trait implemented by systems that require exclusive access to [`World`] and [`Resources`].
pub trait RunExclusiveSystem<TParams, TReturn> {
    /// Runs the system.
    fn run(self, world: &mut World, resources: &mut Resources) -> TReturn;
}

impl World {
    /// Runs the system.
    #[inline]
    pub fn run<TParams, TReturn>(&self, f: impl RunInWorld<TParams, TReturn>) -> TReturn {
        RunInWorld::run(f, self)
    }
}

impl Resources {
    /// Runs the system.
    #[inline]
    pub fn run<TParams, TReturn>(&self, f: impl RunInResources<TParams, TReturn>) -> TReturn {
        RunInResources::run(f, self)
    }
}

impl SyncResources<'_> {
    /// Runs the system.
    #[inline]
    pub fn run<TParams, TReturn>(&self, f: impl RunInSyncResources<TParams, TReturn>) -> TReturn {
        RunInSyncResources::run(f, *self)
    }
}

/// Runs the system in the given [`World`] and [`SyncResources`].
pub fn run<'a, TResources, TParams, TReturn>(
    world: &World,
    resources: TResources,
    f: impl RunSystem<TParams, TReturn>,
) -> TReturn
where
    TResources: Into<SyncResources<'a>>,
{
    RunSystem::run(f, world, resources.into())
}

/// Runs the system locally in the given [`World`] and [`Resources`].
pub fn run_local<TParams, TReturn>(
    world: &World,
    resources: &Resources,
    f: impl RunLocalSystem<TParams, TReturn>,
) -> TReturn {
    RunLocalSystem::run(f, world, resources)
}

/// Runs the system with exclusive access to the given [`World`] and [`Resources`].
pub fn run_exclusive<TParams, TReturn>(
    world: &mut World,
    resources: &mut Resources,
    f: impl RunExclusiveSystem<TParams, TReturn>,
) -> TReturn {
    RunExclusiveSystem::run(f, world, resources)
}

macro_rules! impl_run {
    ($($TParam:ident),*) => {
        impl<TFunc, $($TParam,)* TReturn> RunInWorld<($($TParam,)*), TReturn> for TFunc
        where
            TFunc: FnOnce($($TParam),*) -> TReturn
                 + FnOnce($(<$TParam as SystemDataDescriptor>::SystemData<'_>),*) -> TReturn,
            $($TParam: WorldSystemData,)*
        {
            #[allow(unused_variables)]
            fn run(self, world: &World) -> TReturn {
                self($($TParam::borrow(world)),*)
            }
        }

        impl<TFunc, $($TParam,)* TReturn> RunInResources<($($TParam,)*), TReturn> for TFunc
        where
            TFunc: FnOnce($($TParam),*) -> TReturn
                 + FnOnce($(<$TParam as SystemDataDescriptor>::SystemData<'_>),*) -> TReturn,
            $($TParam: ResourcesSystemData,)*
        {
            #[allow(unused_variables)]
            fn run(self, resources: &Resources) -> TReturn {
                self($($TParam::borrow(resources)),*)
            }
        }

        impl<TFunc, $($TParam,)* TReturn> RunInSyncResources<($($TParam,)*), TReturn> for TFunc
        where
            TFunc: FnOnce($($TParam),*) -> TReturn
                 + FnOnce($(<$TParam as SystemDataDescriptor>::SystemData<'_>),*) -> TReturn,
            $($TParam: SyncResourcesSystemData,)*
        {
            #[allow(unused_variables)]
            fn run(self, resources: SyncResources<'_>) -> TReturn {
                self($(<$TParam as SyncResourcesSystemData>::borrow(resources)),*)
            }
        }

        impl<TFunc, $($TParam,)* TReturn> RunLocalSystem<($($TParam,)*), TReturn> for TFunc
        where
            TFunc: FnOnce($($TParam),*) -> TReturn
                 + FnOnce($(<$TParam as SystemDataDescriptor>::SystemData<'_>),*) -> TReturn,
            $($TParam: LocalSystemData,)*
        {
            #[allow(unused_variables)]
            fn run(self, world: &World, resources: &Resources) -> TReturn {
                self($($TParam::borrow(world, resources)),*)
            }
        }

        impl<TFunc, $($TParam,)* TReturn> RunSystem<($($TParam,)*), TReturn> for TFunc
        where
            TFunc: FnOnce($($TParam),*) -> TReturn
                 + FnOnce($(<$TParam as SystemDataDescriptor>::SystemData<'_>),*) -> TReturn,
            $($TParam: SystemData,)*
        {
            #[allow(unused_variables)]
            fn run(self, world: &World, resources: SyncResources<'_>) -> TReturn {
                self($(<$TParam as SystemData>::borrow(world, resources)),*)
            }
        }

        impl<TFunc, $($TParam,)* TReturn> RunExclusiveSystem<($($TParam,)*), TReturn> for TFunc
        where
            TFunc: FnOnce($($TParam),*) -> TReturn
                 + FnOnce($(<$TParam as SystemDataDescriptor>::SystemData<'_>),*) -> TReturn,
            $($TParam: LocalSystemData,)*
        {
            #[allow(unused_variables)]
            fn run(self, world: &mut World, resources: &mut Resources) -> TReturn {
                self($($TParam::borrow(world, resources)),*)
            }
        }
    };
}

impl_generic_0_to_16!(impl_run);

impl<F, Return> RunExclusiveSystem<(&mut World, &mut Resources), Return> for F
where
    F: FnOnce(&mut World, &mut Resources) -> Return,
{
    fn run(self, world: &mut World, resources: &mut Resources) -> Return {
        self(world, resources)
    }
}

impl<F, Return> RunExclusiveSystem<(&mut Resources, &mut World), Return> for F
where
    F: FnOnce(&mut Resources, &mut World) -> Return,
{
    fn run(self, world: &mut World, resources: &mut Resources) -> Return {
        self(resources, world)
    }
}

impl<F, Return> RunExclusiveSystem<(&mut World,), Return> for F
where
    F: FnOnce(&mut World) -> Return,
{
    fn run(self, world: &mut World, _: &mut Resources) -> Return {
        self(world)
    }
}

impl<F, Return> RunExclusiveSystem<(&mut Resources,), Return> for F
where
    F: FnOnce(&mut Resources) -> Return,
{
    fn run(self, _: &mut World, resources: &mut Resources) -> Return {
        self(resources)
    }
}
