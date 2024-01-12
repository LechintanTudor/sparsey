use crate::entity::EntityStorage;
use crate::resource::ResourceStorage;
use crate::system::{SystemBorrow, SystemParam, SystemParamKind};
use crate::World;

pub trait Run<TRegistry, TParams, TReturn> {
    const PARAMS: &'static [SystemParamKind];

    fn run(self, registry: &TRegistry) -> TReturn;
}

impl World {
    pub fn run<TParams, TReturn>(&self, f: impl Run<Self, TParams, TReturn>) -> TReturn {
        Run::run(f, self)
    }
}

impl EntityStorage {
    pub fn run<TParams, TReturn>(&self, f: impl Run<Self, TParams, TReturn>) -> TReturn {
        Run::run(f, self)
    }
}

impl ResourceStorage {
    pub fn run<TParams, TReturn>(&self, f: impl Run<Self, TParams, TReturn>) -> TReturn {
        Run::run(f, self)
    }
}

macro_rules! impl_run {
    ($($Param:ident),*) => {
        impl_run_in!(world: World; $($Param),*);
        impl_run_in!(entities: EntityStorage; $($Param),*);
        impl_run_in!(resources: ResourceStorage; $($Param),*);
    };
}

macro_rules! impl_run_in {
    ($registry:ident: $Registry:ty; $($Param:ident),*) => {
        impl<TFunc, $($Param,)* TReturn> Run<$Registry, ($($Param,)*), TReturn> for TFunc
        where
            TFunc: FnOnce($($Param),*) -> TReturn
                 + FnOnce($(<$Param as SystemParam>::Param<'_>),*) -> TReturn,
            $($Param: SystemBorrow<$Registry>,)*
        {
            const PARAMS: &'static [SystemParamKind] = &[$($Param::KIND),*];

            #[allow(unused_variables)]
            fn run(self, $registry: &$Registry) -> TReturn {
                self($($Param::borrow($registry),)*)
            }
        }
    };
}

impl_run!();
impl_run!(A);
impl_run!(A, B);
impl_run!(A, B, C);
impl_run!(A, B, C, D);
impl_run!(A, B, C, D, E);
impl_run!(A, B, C, D, E, F);
impl_run!(A, B, C, D, E, F, G);
impl_run!(A, B, C, D, E, F, G, H);
impl_run!(A, B, C, D, E, F, G, H, I);
impl_run!(A, B, C, D, E, F, G, H, I, J);
impl_run!(A, B, C, D, E, F, G, H, I, J, K);
impl_run!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_run!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_run!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_run!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_run!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
