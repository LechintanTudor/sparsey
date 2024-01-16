mod borrow;
mod param;
mod run;

pub use self::borrow::*;
pub use self::param::*;
pub use self::run::*;

use crate::entity::EntityStorage;
use crate::resource::ResourceStorage;
use crate::World;

pub struct System<TRegistry = World, TReturn = ()> {
    system_fn: Box<dyn FnMut(&TRegistry) -> TReturn + Send + Sync + 'static>,
    params: &'static [SystemParamKind],
}

impl<TRegistry, TReturn> System<TRegistry, TReturn> {
    #[must_use]
    pub fn new<TParams>(f: impl IntoSystem<TRegistry, TParams, TReturn>) -> Self {
        f.system()
    }

    pub fn run(&mut self, registry: &TRegistry) -> TReturn {
        (self.system_fn)(registry)
    }

    #[must_use]
    pub fn params(&self) -> &[SystemParamKind] {
        self.params
    }
}

pub trait IntoSystem<TRegistry, TParams, TReturn> {
    #[must_use]
    fn system(self) -> System<TRegistry, TReturn>;
}

macro_rules! impl_into_system {
    ($($Param:ident),*) => {
        impl_into_system_in!(world: World; $($Param),*);
        impl_into_system_in!(entities: EntityStorage; $($Param),*);
        impl_into_system_in!(resources: ResourceStorage; $($Param),*);
    };
}

macro_rules! impl_into_system_in {
    ($registry:ident: $Registry:ident; $($Param:ident),*) => {
        impl<TFunc, TReturn, $($Param),*> IntoSystem<$Registry, ($($Param,)*), TReturn> for TFunc
        where
            TFunc: Run<$Registry, ($($Param,)*), TReturn> + Send + Sync + 'static,
            for<'a> &'a mut TFunc: Run<$Registry, ($($Param,)*), TReturn>,
        {
            fn system(mut self) -> System<$Registry, TReturn> {
                System {
                    system_fn: Box::new(move |$registry: &$Registry| {
                        Run::run(&mut self, $registry)
                    }),
                    params: TFunc::PARAMS,
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
