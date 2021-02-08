use crate::dispatcher::{
    BorrowRegistry, Registry, RegistryAccess, SystemParameter, ThreadLocalSystemParameter,
};

pub trait ThreadLocalSystem
where
    Self: 'static,
{
    unsafe fn registry_access(&self) -> &[RegistryAccess];

    unsafe fn run_unsafe(&mut self, registry: Registry);
}

pub trait IntoThreadLocalSystem<Parameters> {
    fn thread_local_system(self) -> Box<dyn ThreadLocalSystem>;
}

pub unsafe trait System
where
    Self: Send + ThreadLocalSystem,
{
}

pub trait IntoSystem<Parameters>
where
    Self: IntoThreadLocalSystem<Parameters>,
{
    fn system(self) -> Box<dyn System>;
}

struct SystemWrapper<F> {
    function: F,
    accesses: Vec<RegistryAccess>,
}

impl<F> SystemWrapper<F> {
    fn new(function: F, accesses: Vec<RegistryAccess>) -> Self {
        Self { function, accesses }
    }
}

impl<F> ThreadLocalSystem for SystemWrapper<F>
where
    F: FnMut(Registry) + 'static,
{
    unsafe fn registry_access(&self) -> &[RegistryAccess] {
        &self.accesses
    }

    unsafe fn run_unsafe(&mut self, registry: Registry) {
        (self.function)(registry);
    }
}

unsafe impl<F> System for SystemWrapper<F> where F: FnMut(Registry) + Send + 'static {}

macro_rules! impl_into_system {
    ($($param:ident),*) => {
        impl<Function, $($param),*> IntoThreadLocalSystem<($($param,)*)> for Function
        where
            Function: FnMut($(<$param::Borrow as BorrowRegistry>::Item),*) + 'static,
            $($param: ThreadLocalSystemParameter),*
        {
            fn thread_local_system(mut self) -> Box<dyn ThreadLocalSystem> {
                #[allow(unused_unsafe)]
                unsafe {
                    Box::new(SystemWrapper::new(
                        #[allow(unused_variables)]
                        move |registry: Registry| {
                            self(
                                $(<$param::Borrow as BorrowRegistry>::borrow_registry(&registry)),*
                            );
                        },
                        vec![
                            $(<$param::Borrow as BorrowRegistry>::registry_access()),*
                        ]
                    ))
                }
            }
        }

        impl<Function, $($param),*> IntoSystem<($($param,)*)> for Function
        where
            Function: FnMut($(<$param::Borrow as BorrowRegistry>::Item),*) + Send + 'static,
            $($param: SystemParameter),*
        {
            fn system(mut self) -> Box<dyn System> {
                #[allow(unused_unsafe)]
                unsafe {
                    Box::new(SystemWrapper::new(
                        #[allow(unused_variables)]
                        move |registry: Registry| {
                            self(
                                $(<$param::Borrow as BorrowRegistry>::borrow_registry(&registry)),*
                            );
                        },
                        vec![
                            $(<$param::Borrow as BorrowRegistry>::registry_access()),*
                        ]
                    ))
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
