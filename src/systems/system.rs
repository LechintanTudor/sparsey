use crate::systems::{
    BorrowRegistry, GatBorrow, GatBorrowItem, LocalSystemParam, Registry, RegistryAccess,
    SystemParam, SystemResult,
};
use crate::utils::impl_generic_0_16;
use crate::world::World;

/// Trait implemented by systems..
pub unsafe trait Runnable {
    /// Returns all data accessed by the system.
    fn accesses(&self) -> &[RegistryAccess];

    /// Runs the system in the given `Registry`.
    fn run(&mut self, registry: &Registry) -> SystemResult;
}

/// Encapsulates a system function that can be run locally. Implements
/// `Runnable`.
pub struct LocalSystem {
    function: Box<dyn FnMut(&Registry) -> SystemResult + 'static>,
    accesses: Box<[RegistryAccess]>,
}

unsafe impl Sync for LocalSystem {}

unsafe impl Runnable for LocalSystem {
    fn accesses(&self) -> &[RegistryAccess] {
        &self.accesses
    }

    fn run(&mut self, registry: &Registry) -> SystemResult {
        (self.function)(registry)
    }
}

/// Helper trait for creating a `LocalSystem` from a system function.
pub unsafe trait IntoLocalSystem<Params, Return> {
    /// Creates a `LocalSystem` with the system function.
    fn local_system(self) -> LocalSystem;
}

/// Encapsulates a system function that can be run on any thread. Implements
/// `Runnable`.
pub struct System {
    function: Box<dyn FnMut(&Registry) -> SystemResult + Send + 'static>,
    accesses: Box<[RegistryAccess]>,
}

unsafe impl Send for System {}
unsafe impl Sync for System {}

unsafe impl Runnable for System {
    fn accesses(&self) -> &[RegistryAccess] {
        &self.accesses
    }

    fn run(&mut self, registry: &Registry) -> SystemResult {
        (self.function)(registry)
    }
}

unsafe impl IntoLocalSystem<(), ()> for System {
    fn local_system(self) -> LocalSystem {
        LocalSystem { function: self.function, accesses: self.accesses }
    }
}

/// Helper trait for creating a `System` from a system function.
pub unsafe trait IntoSystem<Params, Return>: IntoLocalSystem<Params, Return> {
    /// Creates a `System` with the system function.
    fn system(self) -> System;
}

/// Encapsulates a system function with exclusive access to `World`.
pub struct LocalFn {
    function: Box<dyn FnMut(&mut World) -> SystemResult + 'static>,
}

impl LocalFn {
    /// Runs the system on the given `World`.
    pub fn run(&mut self, world: &mut World) -> SystemResult {
        (self.function)(world)
    }
}

/// Helper trait for creating a `LocalFn`.
pub trait IntoLocalFn<Return> {
    /// Creates a `LocalFn` with the system function.
    fn local_fn(self) -> LocalFn;
}

impl<F> IntoLocalFn<()> for F
where
    F: FnMut(&mut World) + 'static,
{
    fn local_fn(mut self) -> LocalFn {
        LocalFn {
            function: Box::new(move |world| {
                self(world);
                Ok(())
            }),
        }
    }
}

impl<F> IntoLocalFn<SystemResult> for F
where
    F: FnMut(&mut World) -> SystemResult + 'static,
{
    fn local_fn(self) -> LocalFn {
        LocalFn { function: Box::new(self) }
    }
}

macro_rules! impl_into_system {
    ($($param:ident),*) => {
        unsafe impl<Func, $($param),*> IntoLocalSystem<($($param,)*), ()> for Func
        where
            Func:  'static,
            for<'a> &'a mut Func: FnMut($($param),*)
                + FnMut($(GatBorrowItem<$param>),*),
            $($param: LocalSystemParam,)*
        {
            #[allow(unused_unsafe, unused_variables, non_snake_case)]
            fn local_system(mut self) -> LocalSystem {
                fn inner<$($param),*>(
                    mut f: impl FnMut($($param),*),
                    $($param: $param,)*
                ) {
                    f($($param,)*)
                }

                LocalSystem {
                    function: Box::new(move |registry| unsafe {
                        let ($($param,)*) = ($(GatBorrow::<$param>::borrow(registry),)*);
                        inner(&mut self, $($param),*);
                        Ok(())
                    }),
                    accesses: Box::new([
                        $(GatBorrow::<$param>::access()),*
                    ]),
                }
            }
        }

        unsafe impl<Func, $($param),*> IntoLocalSystem<($($param,)*), SystemResult> for Func
        where
            Func:  'static,
            for<'a> &'a mut Func: FnMut($($param),*) -> SystemResult
                + FnMut($(GatBorrowItem<$param>),*) -> SystemResult,
            $($param: LocalSystemParam,)*
        {
            #[allow(unused_unsafe, unused_variables, non_snake_case)]
            fn local_system(mut self) -> LocalSystem {
                fn inner<$($param),*>(
                    mut f: impl FnMut($($param),*) -> SystemResult,
                    $($param: $param,)*
                ) -> SystemResult {
                    f($($param,)*)
                }

                LocalSystem {
                    function: Box::new(move |registry| unsafe {
                        let ($($param,)*) = ($(GatBorrow::<$param>::borrow(registry),)*);
                        inner(&mut self, $($param),*)
                    }),
                    accesses: Box::new([
                        $(GatBorrow::<$param>::access()),*
                    ]),
                }
            }
        }

        unsafe impl<Func, $($param),*> IntoSystem<($($param,)*), ()> for Func
        where
            Func: Send + 'static,
            for<'a> &'a mut Func: FnMut($($param),*)
                + FnMut($(GatBorrowItem<$param>),*),
            $($param: SystemParam,)*
        {
            #[allow(unused_unsafe, unused_variables, non_snake_case)]
            fn system(mut self) -> System {
                fn inner<$($param),*>(
                    mut f: impl FnMut($($param),*),
                    $($param: $param,)*
                ) {
                    f($($param,)*)
                }

                System {
                    function: Box::new(move |registry| unsafe {
                        let ($($param,)*) = ($(GatBorrow::<$param>::borrow(registry),)*);
                        inner(&mut self, $($param),*);
                        Ok(())
                    }),
                    accesses: Box::new([
                        $(GatBorrow::<$param>::access()),*
                    ]),
                }
            }
        }

        unsafe impl<Func, $($param),*> IntoSystem<($($param,)*), SystemResult> for Func
        where
            Func: Send + 'static,
            for<'a> &'a mut Func: FnMut($($param),*) -> SystemResult
                + FnMut($(GatBorrowItem<$param>),*) -> SystemResult,
            $($param: SystemParam,)*
        {
            #[allow(unused_unsafe, unused_variables, non_snake_case)]
            fn system(mut self) -> System {
                fn inner<$($param),*>(
                    mut f: impl FnMut($($param),*) -> SystemResult,
                    $($param: $param,)*
                ) -> SystemResult {
                    f($($param,)*)
                }

                System {
                    function: Box::new(move |registry| unsafe {
                        let ($($param,)*) = ($(GatBorrow::<$param>::borrow(registry),)*);
                        inner(&mut self, $($param),*)
                    }),
                    accesses: Box::new([
                        $(GatBorrow::<$param>::access()),*
                    ]),
                }
            }
        }
    };
}

impl_generic_0_16!(impl_into_system);
