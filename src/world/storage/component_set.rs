pub use self::impls::*;

use crate::data::{Component, Entity};
use crate::query::SparseSetRefMutBorrow;
use crate::world::Components;
use std::any::TypeId;
use std::marker::PhantomData;

pub unsafe trait ComponentSet
where
    Self: Sized,
{
    type Components: AsRef<[TypeId]>;
    type Storages: for<'a> BorrowStorages<'a>;

    fn components() -> Self::Components;

    unsafe fn borrow_storages(components: &Components) -> <Self::Storages as BorrowStorages>::Item;

    unsafe fn insert(
        storages: &mut <Self::Storages as BorrowStorages>::Item,
        entity: Entity,
        components: Self,
    );

    unsafe fn remove(
        storages: &mut <Self::Storages as BorrowStorages>::Item,
        entity: Entity,
    ) -> Option<Self>;

    unsafe fn delete(storages: &mut <Self::Storages as BorrowStorages>::Item, entity: Entity);
}

pub trait BorrowStorages<'a> {
    type Item;

    unsafe fn borrow(components: &'a Components) -> Self::Item;
}

pub struct StorageBorrower<T>
where
    T: Send + Sync + 'static,
{
    _phantom: PhantomData<*const T>,
}

macro_rules! impl_component_set {
    ($len:tt, $(($comp:ident, $idx:tt)),+) => {
        unsafe impl<$($comp),+> ComponentSet for ($($comp,)+)
        where
            $($comp: Component,)+
        {
            type Components = [TypeId; $len];
            type Storages = StorageBorrower<($($comp,)+)>;

            fn components() -> Self::Components {
                [$(TypeId::of::<$comp>()),+]
            }

            unsafe fn borrow_storages(components: &Components) -> <Self::Storages as BorrowStorages>::Item {
                ($(components.borrow_sparse_set_mut::<$comp>().unwrap(),)+)
            }

            unsafe fn insert(storages: &mut <Self::Storages as BorrowStorages>::Item, entity: Entity, components: Self) {
                $(
                    storages.$idx.insert(entity, components.$idx);
                )+
            }

            unsafe fn remove(storages: &mut <Self::Storages as BorrowStorages>::Item, entity: Entity) -> Option<Self> {
                let components = (
                    $(storages.$idx.remove(entity),)+
                );

                Some((
                    $(components.$idx?,)+
                ))
            }

            unsafe fn delete(storages: &mut <Self::Storages as BorrowStorages>::Item, entity: Entity) {
                $(
                    storages.$idx.remove(entity);
                )+
            }
        }

        impl<'a, $($comp),+> BorrowStorages<'a> for StorageBorrower<($($comp,)+)>
        where
            $($comp: Component,)+
        {
            type Item = ($(SparseSetRefMutBorrow<'a, $comp>,)+);

            unsafe fn borrow(components: &'a Components) -> Self::Item {
                (
                    $(components.borrow_sparse_set_mut::<$comp>().unwrap(),)+
                )
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_component_set!(1,  (A, 0));
    impl_component_set!(2,  (A, 0), (B, 1));
    impl_component_set!(3,  (A, 0), (B, 1), (C, 2));
    impl_component_set!(4,  (A, 0), (B, 1), (C, 2), (D, 3));
    impl_component_set!(5,  (A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
    impl_component_set!(6,  (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
    impl_component_set!(7,  (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
    impl_component_set!(8,  (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7));
    impl_component_set!(9,  (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8));
    impl_component_set!(10, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9));
    impl_component_set!(11, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10));
    impl_component_set!(12, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11));
    impl_component_set!(13, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12));
    impl_component_set!(14, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13));
    impl_component_set!(15, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14));
    impl_component_set!(16, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14), (P, 15));
}
