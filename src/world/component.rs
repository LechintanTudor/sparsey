pub use self::impls::*;

use crate::storage::Entity;
use crate::world::World;
use std::any::TypeId;

pub type ComponentTypeId = TypeId;

pub trait Component
where
    Self: Send + Sync + 'static,
{
}

impl<T> Component for T where T: Send + Sync + 'static {}

pub trait ComponentSet
where
    Self: Sized,
{
    type Components: AsRef<[TypeId]>;

    unsafe fn components() -> Self::Components;

    unsafe fn insert_raw(world: &World, entity: Entity, components: Self);

    unsafe fn remove_raw(world: &World, entity: Entity) -> Option<Self>;

    unsafe fn delete_raw(world: &World, entity: Entity);
}

macro_rules! impl_component_set {
    ($len:tt, $(($ty:ident, $idx:tt)),+) => {
        impl<$($ty,)+> ComponentSet for ($($ty,)+)
        where
            $($ty: Component,)+
        {
            type Components = [TypeId; $len];

            unsafe fn components() -> Self::Components {
                [$(TypeId::of::<$ty>(),)+]
            }

            unsafe fn insert_raw(world: &World, entity: Entity, components: Self) {
                let mut sparse_sets = (
                    $(world.borrow_sparse_set_mut::<$ty>().unwrap(),)+
                );

                $(sparse_sets.$idx.insert(entity, components.$idx);)+
            }

            unsafe fn remove_raw(world: &World, entity: Entity) -> Option<Self> {
                let mut sparse_sets = (
                    $(world.borrow_sparse_set_mut::<$ty>().unwrap(),)+
                );

                let components = (
                    $(sparse_sets.$idx.remove(entity),)+
                );

                Some((
                    $(components.$idx?,)+
                ))
            }

            unsafe fn delete_raw(world: &World, entity: Entity) {
                let mut sparse_sets = (
                    $(world.borrow_sparse_set_mut::<$ty>().unwrap(),)+
                );

                $(sparse_sets.$idx.remove(entity);)+
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
