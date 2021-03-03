use crate::dispatcher::{
    BorrowEnvironment, Environment, LocalSystemParam, SystemAccess, SystemParam,
};

pub trait LocallyRunnable {
    fn access(&self) -> &[SystemAccess];

    unsafe fn run(&mut self, environment: Environment);
}

pub unsafe trait Runnable {}

pub struct LocalSystem {
    runnable: Box<dyn FnMut(Environment) + 'static>,
    accesses: Vec<SystemAccess>,
}

impl LocallyRunnable for LocalSystem {
    fn access(&self) -> &[SystemAccess] {
        &self.accesses
    }

    unsafe fn run(&mut self, environment: Environment) {
        (self.runnable)(environment);
    }
}

pub trait IntoLocalSystem<Params> {
    fn local_system(self) -> LocalSystem;
}

pub struct System {
    runnable: Box<dyn FnMut(Environment) + Send + 'static>,
    accesses: Vec<SystemAccess>,
}

impl LocallyRunnable for System {
    fn access(&self) -> &[SystemAccess] {
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
