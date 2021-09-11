use crate::systems::{
    BorrowRegistry, LocalSystemParam, Registry, RegistryAccess, SystemParam, SystemResult,
};
use crate::world::World;

/// Trait implemented by systems. `Send` systems can be run from any thread.
pub unsafe trait Runnable {
    /// Returns all data accessed by the system.
    fn accesses(&self) -> &[RegistryAccess];

    /// Runs the system in the given `Registry`.
    fn run(&mut self, registry: &Registry) -> SystemResult;
}

// Encapsulates a system function that can be run locally. Implements
// `Runnable`.
pub struct LocalSystem {
    function: Box<dyn FnMut(&Registry) -> SystemResult + 'static>,
    accesses: Vec<RegistryAccess>,
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
    accesses: Vec<RegistryAccess>,
}

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
        LocalSystem {
            function: self.function,
            accesses: self.accesses,
        }
    }
}

/// Helper trait for creating a `System` from a system function.
pub unsafe trait IntoSystem<Params, Return>
where
    Self: IntoLocalSystem<Params, Return>,
{
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
        LocalFn {
            function: Box::new(self),
        }
    }
}

macro_rules! impl_into_system {
    ($($param:ident),*) => {
        unsafe impl<Func, $($param),*> IntoLocalSystem<($($param,)*), ()> for Func
        where
            Func: FnMut($($param),*)
                + FnMut($(<$param as BorrowRegistry>::Item),*)
                +'static,
            $($param: LocalSystemParam,)*
        {
            #[allow(unused_unsafe)]
            #[allow(unused_variables)]
            fn local_system(mut self) -> LocalSystem {
                LocalSystem {
                    function: Box::new(move |registry| unsafe {
                        self($(<$param as BorrowRegistry>::borrow(registry)),*);
                        Ok(())
                    }),
                    accesses: vec![
                        $(<$param as BorrowRegistry>::access()),*
                    ],
                }
            }
        }

        unsafe impl<Func, $($param),*> IntoLocalSystem<($($param,)*), SystemResult> for Func
        where
            Func: FnMut($($param),*) -> SystemResult
                + FnMut($(<$param as BorrowRegistry>::Item),*) -> SystemResult
                + 'static,
            $($param: LocalSystemParam,)*
        {
            #[allow(unused_unsafe)]
            #[allow(unused_variables)]
            fn local_system(mut self) -> LocalSystem {
                LocalSystem {
                    function: Box::new(move |registry| unsafe {
                        self($(<$param as BorrowRegistry>::borrow(registry)),*)
                    }),
                    accesses: vec![
                        $(<$param as BorrowRegistry>::access()),*
                    ],
                }
            }
        }

        unsafe impl<Func, $($param),*> IntoSystem<($($param,)*), ()> for Func
        where
            Func: FnMut($($param),*)
                + FnMut($(<$param as BorrowRegistry>::Item),*)
                + Send + 'static,
            $($param: SystemParam,)*
        {
            #[allow(unused_unsafe)]
            #[allow(unused_variables)]
            fn system(mut self) -> System {
                System {
                    function: Box::new(move |registry| unsafe {
                        self($(<$param as BorrowRegistry>::borrow(registry)),*);
                        Ok(())
                    }),
                    accesses: vec![
                        $(<$param as BorrowRegistry>::access()),*
                    ],
                }
            }
        }

        unsafe impl<Func, $($param),*> IntoSystem<($($param,)*), SystemResult> for Func
        where
            Func: FnMut($($param),*) -> SystemResult
                + FnMut($(<$param as BorrowRegistry>::Item),*) -> SystemResult
                + Send + 'static,
            $($param: SystemParam,)*
        {
            #[allow(unused_unsafe)]
            #[allow(unused_variables)]
            fn system(mut self) -> System {
                System {
                    function: Box::new(move |registry| unsafe {
                        self($(<$param as BorrowRegistry>::borrow(registry)),*)
                    }),
                    accesses: vec![
                        $(<$param as BorrowRegistry>::access()),*
                    ],
                }
            }
        }
    };
}

impl_into_system!();
impl_into_system!(A);
impl_into_system!(A, B);
impl_into_system!(A, B, C);
impl_into_system!(A, B, C, D);
impl_into_system!(A, B, C, D, E);
impl_into_system!(A, B, C, D, E, F);
impl_into_system!(A, B, C, D, E, F, G);
impl_into_system!(A, B, C, D, E, F, G, H);
impl_into_system!(A, B, C, D, E, F, G, H, I);
impl_into_system!(A, B, C, D, E, F, G, H, I, J);
impl_into_system!(A, B, C, D, E, F, G, H, I, J, K);
impl_into_system!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_into_system!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_into_system!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_into_system!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_into_system!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
