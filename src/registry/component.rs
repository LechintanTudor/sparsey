pub use self::impls::*;

use crate::registry::{BorrowFromWorld, SparseSetMut};
use crate::storage::Entity;
use std::any::TypeId;

pub trait Component
where
    Self: Send + Sync + 'static,
{
}

impl<T> Component for T where T: Send + Sync + 'static {}

pub trait ComponentSet<'a>
where
    Self: Sized,
{
    type Target: BorrowFromWorld<'a>;
    type Components: AsRef<[TypeId]>;

    fn components() -> Self::Components;

    fn insert(target: &mut Self::Target, entity: Entity, components: Self);

    fn remove(target: &mut Self::Target, entity: Entity) -> Option<Self>;

    fn delete(target: &mut Self::Target, entity: Entity);
}

macro_rules! impl_component_set {
    ($len:tt, $(($ty:ident, $idx:tt)),+) => {
        impl<'a, $($ty,)+> ComponentSet<'a> for ($($ty,)+)
        where
            $($ty: Component,)+
        {
            type Target = ($(SparseSetMut<'a, $ty>,)+);
            type Components = [TypeId; $len];

            fn components() -> Self::Components {
                [$(TypeId::of::<$ty>(),)+]
            }

            fn insert(target: &mut Self::Target, entity: Entity, components: Self) {
                $(target.$idx.0.insert(entity, components.$idx);)+
            }

            fn remove(target: &mut Self::Target, entity: Entity) -> Option<Self> {
                let components = (
                    $(target.$idx.0.remove(entity),)+
                );

                Some((
                    $(components.$idx?,)+
                ))
            }

            fn delete(target: &mut Self::Target, entity: Entity) {
                $(target.$idx.0.remove(entity);)+
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_component_set!(1, (A, 0));
    impl_component_set!(2, (A, 0), (B, 1));
    impl_component_set!(3, (A, 0), (B, 1), (C, 2));
    impl_component_set!(4, (A, 0), (B, 1), (C, 2), (D, 3));
    impl_component_set!(5, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
    impl_component_set!(6, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
    impl_component_set!(7, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
    impl_component_set!(8, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7));

    impl_component_set!(9, (A, 0), (B, 1), (C, 2), (D, 3), 
        (E, 4), (F, 5), (G, 6), (H, 7), (I, 8));

    impl_component_set!(10, (A, 0), (B, 1), (C, 2), (D, 3), 
        (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9));

    impl_component_set!(11, (A, 0), (B, 1), (C, 2), (D, 3), 
        (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10));

    impl_component_set!(12, (A, 0), (B, 1), (C, 2), (D, 3), 
        (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11));  
}
