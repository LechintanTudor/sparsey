use crate::data::{Component, Entity, SparseSetMutPtr};
use crate::world::World;
use std::any::TypeId;

pub unsafe trait ComponentSet
where
    Self: Sized,
{
    type Components: AsRef<[TypeId]>;
    type Storages;

    fn components() -> Self::Components;

    unsafe fn get_storages(world: &World) -> Self::Storages;

    unsafe fn insert(storages: &mut Self::Storages, entity: Entity, components: Self);

    unsafe fn remove(storages: &mut Self::Storages, entity: Entity) -> Option<Self>;

    unsafe fn delete(storages: &mut Self::Storages, entity: Entity);
}

macro_rules! impl_component_set {
    ($len:tt, $(($comp:ident, $idx:tt)),+) => {
        unsafe impl<$($comp),+> ComponentSet for ($($comp,)+)
        where
            $($comp: Component,)+
        {
            type Components = [TypeId; $len];
            type Storages = ($(SparseSetMutPtr<$comp>,)+);

            fn components() -> Self::Components {
                [$(TypeId::of::<$comp>()),+]
            }

            unsafe fn get_storages(world: &World) -> Self::Storages {
                ($(world.get_sparse_set_mut_ptr::<$comp>().unwrap(),)+)
            }

            unsafe fn insert(storages: &mut Self::Storages, entity: Entity, components: Self) {
                $(
                    storages.$idx.insert(entity, components.$idx);
                )+
            }

            unsafe fn remove(storages: &mut Self::Storages, entity: Entity) -> Option<Self> {
                let components = (
                    $(storages.$idx.remove(entity),)+
                );

                Some((
                    $(components.$idx?,)+
                ))
            }

            unsafe fn delete(storages: &mut Self::Storages, entity: Entity) {
                $(
                    storages.$idx.remove(entity);
                )+
            }
        }
    };
}

impl_component_set!(1, (A, 0));
impl_component_set!(2, (A, 0), (B, 1));
impl_component_set!(3, (A, 0), (B, 1), (C, 2));
impl_component_set!(4, (A, 0), (B, 1), (C, 2), (D, 3));
