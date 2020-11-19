use crate::registry::Component;
use std::any::TypeId;

pub trait GroupDefinition {
    fn components() -> Vec<TypeId>;
}

macro_rules! impl_group_def {
    ($($comp:ident),+) => {
        impl<$($comp,)+> GroupDefinition for ($($comp,)+)
        where
            $($comp: Component,)+
        {
            fn components() -> Vec<TypeId> {
                vec![$(TypeId::of::<$comp>(),)+]
            }
        }
    };
}

impl_group_def!(A, B);
impl_group_def!(A, B, C);
impl_group_def!(A, B, C, D);
impl_group_def!(A, B, C, D, E);
impl_group_def!(A, B, C, D, E, F);
impl_group_def!(A, B, C, D, E, F, G);
impl_group_def!(A, B, C, D, E, F, G, H);
