use crate::storage::Entity;
use crate::world::{SparseSetRefMut, World};
use std::any::TypeId;
use std::marker::PhantomData;

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
    type Borrow: for<'a> BorrowSparseSetSet<'a>;

    unsafe fn components() -> Self::Components;

    unsafe fn append(
        sparse_sets: &mut <Self::Borrow as BorrowSparseSetSet>::SparseSetSet,
        entity: Entity,
        components: Self,
    );

    unsafe fn remove(
        sparse_sets: &mut <Self::Borrow as BorrowSparseSetSet>::SparseSetSet,
        entity: Entity,
    ) -> Option<Self>;

    unsafe fn delete(
        sparse_sets: &mut <Self::Borrow as BorrowSparseSetSet>::SparseSetSet,
        entity: Entity,
    );
}

pub trait BorrowSparseSetSet<'a> {
    type SparseSetSet: 'a;

    unsafe fn borrow(world: &'a World) -> Self::SparseSetSet;
}

pub struct SparseSetSetBorrower<S> {
    _phantom: PhantomData<S>,
}

#[allow(unused_macros)]
macro_rules! impl_component_set {
    ($len:tt, $(($comp:ident, $idx:tt)),+) => {
        impl<$($comp),+> ComponentSet for ($($comp,)+)
        where
            $($comp: Component,)+
        {
            type Components = [TypeId; $len];
            type Borrow = SparseSetSetBorrower<Self>;

            unsafe fn components() -> Self::Components {
                [
                    $(TypeId::of::<$comp>()),+
                ]
            }

            unsafe fn append(
                sparse_sets: &mut <Self::Borrow as BorrowSparseSetSet>::SparseSetSet,
                entity: Entity,
                components: Self,
            ) {
                $(
                    sparse_sets.$idx.insert(entity, components.$idx);
                )+
            }

            unsafe fn remove(
                sparse_sets: &mut <Self::Borrow as BorrowSparseSetSet>::SparseSetSet,
                entity: Entity,
            ) -> Option<Self> {
                let removed_components = (
                    $(sparse_sets.$idx.remove(entity),)+
                );

                Some((
                    $(removed_components.$idx?,)+
                ))
            }

            unsafe fn delete(
                sparse_sets: &mut <Self::Borrow as BorrowSparseSetSet>::SparseSetSet,
                entity: Entity,
            ) {
                $(sparse_sets.$idx.remove(entity);)+
            }
        }

        impl<'a, $($comp),+> BorrowSparseSetSet<'a> for SparseSetSetBorrower<($($comp,)+)>
        where
            $($comp: Component,)+
        {
            type SparseSetSet = ($(SparseSetRefMut<'a, $comp>,)+);

            unsafe fn borrow(world: &'a World) -> Self::SparseSetSet {
                ($(world.borrow_sparse_set_mut::<$comp>().unwrap(),)+)
            }
        }
    };
}

impl_component_set!(1, (A, 0));
impl_component_set!(2, (A, 0), (B, 1));
impl_component_set!(3, (A, 0), (B, 1), (C, 2));
impl_component_set!(4, (A, 0), (B, 1), (C, 2), (D, 3));
