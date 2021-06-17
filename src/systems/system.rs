use crate::resources::Resources;
use crate::systems::{
	BorrowEnvironment, Environment, LocalSystemParam, SystemAccess, SystemParam, SystemResult,
};
use crate::world::World;

/// Trait implemented by `Systems` which can run on the thread
/// in which they were created.
pub unsafe trait LocallyRunnable {
	/// Get a list of all data acessess in the `run` function.
	fn accesses(&self) -> &[SystemAccess];

	/// Run the system in the given `Environment`.
	/// Always safe to call in the thread in which the system was created.
	unsafe fn run(&mut self, environment: Environment) -> SystemResult;
}

/// Marker trait for `Systems` which can be run in threads
/// other than the one in which they were created.
pub unsafe trait Runnable
where
	Self: LocallyRunnable,
{
}

/// Encapsulates a locally runnable function. Implements the `LocallyRunnable`
/// trait.
pub struct LocalSystem {
	function: Box<dyn FnMut(Environment) -> SystemResult + 'static>,
	accesses: Vec<SystemAccess>,
}

impl LocalSystem {
	/// Create a `LocalSystem` with the given function.
	/// Generally used for creating stateful local systems.
	pub fn new<P, R, F>(function: F) -> Self
	where
		F: IntoLocalSystem<P, R>,
	{
		function.local_system()
	}
}

unsafe impl LocallyRunnable for LocalSystem {
	fn accesses(&self) -> &[SystemAccess] {
		&self.accesses
	}

	unsafe fn run(&mut self, environment: Environment) -> SystemResult {
		(self.function)(environment)
	}
}

/// Trait implemented by functions which can be turned into `LocalSystems`.
pub trait IntoLocalSystem<Params, Return> {
	/// Create a `LocalSystem` from `self`.
	fn local_system(self) -> LocalSystem;
}

/// Encapsulates a runnable function. Implements the `Runnable` trait.
pub struct System {
	function: Box<dyn FnMut(Environment) -> SystemResult + Send + 'static>,
	accesses: Vec<SystemAccess>,
}

impl System {
	/// Create a `System` with the given function.
	/// Generally used for creating stateful systems.
	pub fn new<P, R, F>(function: F) -> Self
	where
		F: IntoSystem<P, R>,
	{
		function.system()
	}
}

unsafe impl LocallyRunnable for System {
	fn accesses(&self) -> &[SystemAccess] {
		&self.accesses
	}

	unsafe fn run(&mut self, environment: Environment) -> SystemResult {
		(self.function)(environment)
	}
}

unsafe impl Runnable for System {}

impl IntoLocalSystem<(), ()> for System {
	fn local_system(self) -> LocalSystem {
		LocalSystem {
			function: self.function,
			accesses: self.accesses,
		}
	}
}

/// Trait implemented by functions which can be turned into `Systems`.
pub trait IntoSystem<Params, Return>
where
	Self: IntoLocalSystem<Params, Return>,
{
	/// Create a `System` from `self`.
	fn system(self) -> System;
}

/// Encapsulates a system function with exclusive access to `World` and
/// `Resources`.
pub struct LocalFn {
	function: Box<dyn FnMut(&mut World, &mut Resources) -> SystemResult + 'static>,
}

impl LocalFn {
	/// Create a `LocalFn` with the given function.
	/// Generally used for creating stateful systems.
	pub fn new<R, F>(function: F) -> Self
	where
		F: IntoLocalFn<R>,
	{
		function.local_fn()
	}

	/// Run the system on the given `World` and `Resources`.
	pub fn run(&mut self, world: &mut World, resources: &mut Resources) -> SystemResult {
		(self.function)(world, resources)
	}
}

/// Trait implemented by functions which can be turned into `LocalFns`.
pub trait IntoLocalFn<Return> {
	/// Create a `LocalSFn` from `self`.
	fn local_fn(self) -> LocalFn;
}

impl<F> IntoLocalFn<()> for F
where
	F: FnMut(&mut World, &mut Resources) + 'static,
{
	fn local_fn(mut self) -> LocalFn {
		LocalFn {
			function: Box::new(move |world, resources| {
				self(world, resources);
				Ok(())
			}),
		}
	}
}

impl<F> IntoLocalFn<SystemResult> for F
where
	F: FnMut(&mut World, &mut Resources) -> SystemResult + 'static,
{
	fn local_fn(self) -> LocalFn {
		LocalFn {
			function: Box::new(self),
		}
	}
}

macro_rules! impl_into_system {
    ($($param:ident),*) => {
        impl<Func, $($param),*> IntoLocalSystem<($($param,)*), ()> for Func
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
                    function: Box::new(move |environment| unsafe {
                        self($(<$param::Borrow as BorrowEnvironment>::borrow(&environment)),*);
                        Ok(())
                    }),
                    accesses: vec![
                        $(<$param::Borrow as BorrowEnvironment>::access()),*
                    ],
                }
            }
        }

        impl<Func, $($param),*> IntoLocalSystem<($($param,)*), SystemResult> for Func
        where
            Func:
                FnMut($($param),*) -> SystemResult +
                FnMut($(<$param::Borrow as BorrowEnvironment>::Item),*) -> SystemResult +
                'static,
            $($param: LocalSystemParam,)*
        {
            #[allow(unused_unsafe)]
            #[allow(unused_variables)]
            fn local_system(mut self) -> LocalSystem {
                LocalSystem {
                    function: Box::new(move |environment| unsafe {
                        self($(<$param::Borrow as BorrowEnvironment>::borrow(&environment)),*)
                    }),
                    accesses: vec![
                        $(<$param::Borrow as BorrowEnvironment>::access()),*
                    ],
                }
            }
        }

        impl<Func, $($param),*> IntoSystem<($($param,)*), ()> for Func
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
                    function: Box::new(move |environment| unsafe {
                        self($(<$param::Borrow as BorrowEnvironment>::borrow(&environment)),*);
                        Ok(())
                    }),
                    accesses: vec![
                        $(<$param::Borrow as BorrowEnvironment>::access()),*
                    ],
                }
            }
        }

        impl<Func, $($param),*> IntoSystem<($($param,)*), SystemResult> for Func
        where
            Func:
                FnMut($($param),*) -> SystemResult +
                FnMut($(<$param::Borrow as BorrowEnvironment>::Item),*) -> SystemResult +
                Send + 'static,
            $($param: SystemParam,)*
        {
            #[allow(unused_unsafe)]
            #[allow(unused_variables)]
            fn system(mut self) -> System {
                System {
                    function: Box::new(move |environment| unsafe {
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
