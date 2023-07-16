use crate::resources::{Resources, SyncResources};
use crate::systems::{
    RunExclusiveSystem, RunLocalSystem, RunSystem, SystemBorrows, SystemDataType,
};
use crate::utils::impl_generic_0_to_16;
use crate::world::World;
use std::fmt;

/// Encapsulates a system that can run on a local thread.
pub struct LocalSystem {
    #[allow(clippy::type_complexity)]
    system_fn: Box<dyn FnMut(&World, &Resources) + 'static>,
    borrows: Vec<SystemDataType>,
}

impl LocalSystem {
    /// Returns the data borrowed by the system during execution.
    #[inline]
    #[must_use]
    pub fn system_borrows(&self) -> &[SystemDataType] {
        &self.borrows
    }
}

impl SystemBorrows<(), ()> for &'_ LocalSystem {
    #[inline]
    fn collect_system_borrows(&self, borrows: &mut Vec<SystemDataType>) {
        borrows.extend_from_slice(&self.borrows)
    }
}

impl SystemBorrows<(), ()> for &'_ mut LocalSystem {
    #[inline]
    fn collect_system_borrows(&self, borrows: &mut Vec<SystemDataType>) {
        borrows.extend_from_slice(&self.borrows)
    }
}

impl RunLocalSystem<(), ()> for &'_ mut LocalSystem {
    #[inline]
    fn run(self, world: &World, resources: &Resources) {
        (self.system_fn)(world, resources)
    }
}

impl fmt::Debug for LocalSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LocalSystem")
            .field("borrows", &self.borrows)
            .finish_non_exhaustive()
    }
}

/// Trait for converting types into local systems.
pub trait IntoLocalSystem<TParam> {
    /// Converts `self` into a [`LocalSystem`].
    #[must_use]
    fn local_system(self) -> LocalSystem;
}

impl IntoLocalSystem<()> for LocalSystem {
    #[inline]
    fn local_system(self) -> LocalSystem {
        self
    }
}

/// Encapsulates a system that can run on any thread.
pub struct System {
    #[allow(clippy::type_complexity)]
    system_fn: Box<dyn FnMut(&World, SyncResources) + Send + Sync + 'static>,
    borrows: Vec<SystemDataType>,
}

impl System {
    /// Returns the data borrowed by the system during execution.
    #[inline]
    #[must_use]
    pub fn system_borrows(&self) -> &[SystemDataType] {
        &self.borrows
    }
}

impl SystemBorrows<(), ()> for &'_ System {
    #[inline]
    fn collect_system_borrows(&self, borrows: &mut Vec<SystemDataType>) {
        borrows.extend_from_slice(&self.borrows)
    }
}

impl SystemBorrows<(), ()> for &'_ mut System {
    #[inline]
    fn collect_system_borrows(&self, borrows: &mut Vec<SystemDataType>) {
        borrows.extend_from_slice(&self.borrows)
    }
}

impl RunLocalSystem<(), ()> for &'_ mut System {
    #[inline]
    fn run(self, world: &World, resources: &Resources) -> () {
        (self.system_fn)(world, resources.sync())
    }
}

impl RunSystem<(), ()> for &'_ mut System {
    #[inline]
    fn run(self, world: &World, resources: SyncResources<'_>) -> () {
        (self.system_fn)(world, resources)
    }
}

impl fmt::Debug for System {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("System")
            .field("borrows", &self.borrows)
            .finish_non_exhaustive()
    }
}

/// Trait for converting types into systems.
pub trait IntoSystem<TParam> {
    /// Converts `self` into a [`System`].
    #[must_use]
    fn system(self) -> System;
}

impl IntoSystem<()> for System {
    #[inline]
    fn system(self) -> System {
        self
    }
}

/// Encapsulates a system that requires exclusive access to [`World`] and [`Resources`].
pub struct ExclusiveSystem {
    #[allow(clippy::type_complexity)]
    system_fn: Box<dyn FnMut(&mut World, &mut Resources) + 'static>,
}

impl fmt::Debug for ExclusiveSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExclusiveSystem").finish_non_exhaustive()
    }
}

impl RunExclusiveSystem<(), ()> for &'_ mut ExclusiveSystem {
    /// Executes the system in the provided context.
    #[inline]
    fn run(self, world: &mut World, resources: &mut Resources) {
        (self.system_fn)(world, resources)
    }
}

/// Helper trait for creating an [`ExclusiveSystem`] from a function.
pub trait IntoExclusiveSystem<TParams> {
    /// Creates the exclusive system.
    #[must_use]
    fn exclusive_system(self) -> ExclusiveSystem;
}

impl IntoExclusiveSystem<()> for ExclusiveSystem {
    #[inline]
    fn exclusive_system(self) -> ExclusiveSystem {
        self
    }
}

impl<TFunc, TParams> IntoExclusiveSystem<TParams> for TFunc
where
    TFunc: RunExclusiveSystem<TParams, ()> + 'static,
    for<'a> &'a mut TFunc: RunExclusiveSystem<TParams, ()>,
{
    fn exclusive_system(mut self) -> ExclusiveSystem {
        ExclusiveSystem {
            system_fn: Box::new(move |world, resources| {
                (&mut self).run(world, resources);
            }),
        }
    }
}

macro_rules! impl_into_system {
    ($($TParam:ident),*) => {
        impl<TFunc, $($TParam),*> IntoLocalSystem<($($TParam,)*)> for TFunc
        where
            TFunc: RunLocalSystem<($($TParam,)*), ()> + 'static,
            for<'a> &'a mut TFunc: RunLocalSystem<($($TParam,)*), ()>,
        {
            fn local_system(mut self) -> LocalSystem {
                let mut borrows = Vec::<SystemDataType>::new();
                self.collect_system_borrows(&mut borrows);

                LocalSystem {
                    system_fn: Box::new(move |world: &World, resources: &Resources| {
                        RunLocalSystem::run(&mut self, world, resources)
                    }),
                    borrows,
                }
            }
        }

        impl<TFunc, $($TParam),*> IntoSystem<($($TParam,)*)> for TFunc
        where
            TFunc: RunSystem<($($TParam,)*), ()> + Send + Sync + 'static,
            for<'a> &'a mut TFunc: RunSystem<($($TParam,)*), ()>,
        {
            fn system(mut self) -> System {
                let mut borrows = Vec::<SystemDataType>::new();
                self.collect_system_borrows(&mut borrows);

                System {
                    system_fn: Box::new(move |world: &World, resources: SyncResources| {
                        RunSystem::run(&mut self, world, resources)
                    }),
                    borrows,
                }
            }
        }
    };
}

impl_generic_0_to_16!(impl_into_system);
