use crate::dispatcher::{
    BorrowEnvironment, Environment, LocalSystemParam, SystemAccess, SystemParam,
};

/// Trait implemented by systems which can run on the thread
/// in which they were created.
pub unsafe trait LocallyRunnable {
    /// Get a list of all data acessess in the `run` function.
    fn accesses(&self) -> &[SystemAccess];

    /// Run the system in the given `Environment`.
    /// Always safe to call in the thread in which the system was created.
    unsafe fn run(&mut self, environment: Environment);
}

/// Marker trait for systems which can be run in threads other than
/// the one in which they were created.
pub unsafe trait Runnable {}

/// Encapsulates a locally runnable function. Implements the `LocallyRunnable` trait.
pub struct LocalSystem {
    runnable: Box<dyn FnMut(Environment) + 'static>,
    accesses: Vec<SystemAccess>,
}

impl LocalSystem {
    /// Create a `LocalSystem` with the given function.
    /// Generally used for creating stateful local systems.
    pub fn new<P, F>(function: F) -> Self
    where
        F: IntoLocalSystem<P>,
    {
        function.local_system()
    }
}

unsafe impl LocallyRunnable for LocalSystem {
    fn accesses(&self) -> &[SystemAccess] {
        &self.accesses
    }

    unsafe fn run(&mut self, environment: Environment) {
        (self.runnable)(environment);
    }
}

pub trait IntoLocalSystem<Params> {
    fn local_system(self) -> LocalSystem;
}

/// Encapsulates a runnable function. Implements the `Runnable` trait.
pub struct System {
    runnable: Box<dyn FnMut(Environment) + Send + 'static>,
    accesses: Vec<SystemAccess>,
}

impl System {
    /// Create a `System` with the given function.
    /// Generally used for creating stateful systems.
    pub fn new<P, F>(function: F) -> Self
    where
        F: IntoSystem<P>,
    {
        function.system()
    }
}

unsafe impl LocallyRunnable for System {
    fn accesses(&self) -> &[SystemAccess] {
        &self.accesses
    }

    unsafe fn run(&mut self, environment: Environment) {
        (self.runnable)(environment);
    }
}

unsafe impl Runnable for System {}

impl IntoLocalSystem<()> for System {
    fn local_system(self) -> LocalSystem {
        LocalSystem {
            runnable: self.runnable,
            accesses: self.accesses,
        }
    }
}

pub trait IntoSystem<Params>
where
    Self: IntoLocalSystem<Params>,
{
    fn system(self) -> System;
}

macro_rules! impl_into_system {
    ($($param:ident),*) => {
        impl<Func, $($param),*> IntoLocalSystem<($($param,)*)> for Func
        where
            Func:
                FnMut($($param),*) +
                FnMut($(<$param::Borrow as BorrowEnvironment>::Item),*) +
                'static,
            $($param: LocalSystemParam,)*
        {
            #[allow(unused_unsafe)]
            #[allow(unused_variables)]
            fn local_system(mut self) -> LocalSystem {
                LocalSystem {
                    runnable: Box::new(move |environment| unsafe {
                        self($(<$param::Borrow as BorrowEnvironment>::borrow(&environment)),*)
                    }),
                    accesses: vec![
                        $(<$param::Borrow as BorrowEnvironment>::access()),*
                    ],
                }
            }
        }

        impl<Func, $($param),*> IntoSystem<($($param,)*)> for Func
        where
            Func:
                FnMut($($param),*) +
                FnMut($(<$param::Borrow as BorrowEnvironment>::Item),*) +
                Send + 'static,
            $($param: SystemParam,)*
        {
            #[allow(unused_unsafe)]
            #[allow(unused_variables)]
            fn system(mut self) -> System {
                System {
                    runnable: Box::new(move |environment| unsafe {
                        self($(<$param::Borrow as BorrowEnvironment>::borrow(&environment)),*)
                    }),
                    accesses: vec![
                        $(<$param::Borrow as BorrowEnvironment>::access()),*
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
