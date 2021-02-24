use crate::dispatcher::{
    BorrowRegistry, Registry, RegistryAccess, SystemParameter, ThreadLocalSystemParameter,
};

pub trait ThreadLocalRunnable
where
    Self: 'static,
{
    fn registry_access(&self) -> &[RegistryAccess];

    fn run_thread_local(&mut self, registry: Registry);
}

pub trait Runnable
where
    Self: ThreadLocalRunnable + Send,
{
    unsafe fn run(&mut self, registry: Registry);
}

pub struct ThreadLocalSystem {
    runnable: Box<dyn FnMut(Registry) + 'static>,
    accesses: Vec<RegistryAccess>,
}

impl ThreadLocalRunnable for ThreadLocalSystem {
    fn registry_access(&self) -> &[RegistryAccess] {
        &self.accesses
    }

    fn run_thread_local(&mut self, registry: Registry) {
        (self.runnable)(registry);
    }
}

pub trait IntoThreadLocalSystem<Params> {
    fn thread_local_system(self) -> ThreadLocalSystem;
}

pub struct System {
    runnable: Box<dyn FnMut(Registry) + Send + 'static>,
    accesses: Vec<RegistryAccess>,
}

impl ThreadLocalRunnable for System {
    fn registry_access(&self) -> &[RegistryAccess] {
        &self.accesses
    }

    fn run_thread_local(&mut self, registry: Registry) {
        (self.runnable)(registry);
    }
}

impl Runnable for System {
    unsafe fn run(&mut self, registry: Registry) {
        (self.runnable)(registry);
    }
}

impl IntoThreadLocalSystem<()> for System {
    fn thread_local_system(self) -> ThreadLocalSystem {
        ThreadLocalSystem {
            runnable: self.runnable,
            accesses: self.accesses,
        }
    }
}

pub trait IntoSystem<Params>
where
    Self: IntoThreadLocalSystem<Params>,
{
    fn system(self) -> System;
}

macro_rules! impl_into_system {
    ($($param:ident),*) => {
        impl<Func, $($param),*> IntoThreadLocalSystem<($($param,)*)> for Func
        where
            Func:
                FnMut($($param),*) +
                FnMut($(<$param::Borrow as BorrowRegistry>::Item),*) +
                'static,
            $($param: ThreadLocalSystemParameter,)*
        {
            #[allow(unused_unsafe)]
            #[allow(unused_variables)]
            fn thread_local_system(mut self) -> ThreadLocalSystem {
                ThreadLocalSystem {
                    runnable: Box::new(move |registry| unsafe {
                        self($(<$param::Borrow as BorrowRegistry>::borrow_registry(&registry)),*)
                    }),
                    accesses: vec![
                        $(<$param::Borrow as BorrowRegistry>::registry_access()),*
                    ],
                }
            }
        }

        impl<Func, $($param),*> IntoSystem<($($param,)*)> for Func
        where
            Func:
                FnMut($($param),*) +
                FnMut($(<$param::Borrow as BorrowRegistry>::Item),*) +
                Send + 'static,
            $($param: SystemParameter,)*
        {
            #[allow(unused_unsafe)]
            #[allow(unused_variables)]
            fn system(mut self) -> System {
                System {
                    runnable: Box::new(move |registry| unsafe {
                        self($(<$param::Borrow as BorrowRegistry>::borrow_registry(&registry)),*)
                    }),
                    accesses: vec![
                        $(<$param::Borrow as BorrowRegistry>::registry_access()),*
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
