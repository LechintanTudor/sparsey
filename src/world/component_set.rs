pub use self::impls::*;

use crate::components::{Component, ComponentStorage, Entity, TypedComponentStorage};
use crate::misc::panic_missing_comp;
use crate::world::{ComponentStorages, UsedGroupFamilies};
use atomic_refcell::AtomicRefMut;
use std::any::TypeId;
use std::marker::PhantomData;

pub struct BorrowedComponentStorage<'a, T>(
	TypedComponentStorage<AtomicRefMut<'a, ComponentStorage>, T>,
)
where
	T: Component;

/// Trait implemented for component sets which can be
/// added, appended or removed to/from the `World`.
pub unsafe trait ComponentSet
where
	Self: Sized + Send + Sync + 'static,
{
	/// Storages to borrow from the `World` for adding/appending/removing components.
	type Storages: for<'a> BorrowStorages<'a>;

	/// Insert the component in the borrowed storages.
	unsafe fn insert(
		storages: &mut <Self::Storages as BorrowStorages>::StorageSet,
		entity: Entity,
		components: Self,
		tick: u32,
	);

	/// Remove components from the borrowed storages and return them if they exist.
	unsafe fn remove(
		storages: &mut <Self::Storages as BorrowStorages>::StorageSet,
		entity: Entity,
	) -> Option<Self>;

	/// Delete components from the borrowed storages. Faster than removing them.
	unsafe fn delete(storages: &mut <Self::Storages as BorrowStorages>::StorageSet, entity: Entity);
}

/// Trait implemented by `StorageBorrower` to borrow component storages.
/// Only exists because we don't have GATs in stable rust :(
pub trait BorrowStorages<'a> {
	/// Set of borrowed storages.
	type StorageSet;

	fn borrow(components: &'a ComponentStorages) -> Self::StorageSet;

	fn families(components: &'a ComponentStorages) -> UsedGroupFamilies;

	fn borrow_with_families(
		components: &'a ComponentStorages,
	) -> (Self::StorageSet, UsedGroupFamilies);
}

/// Struct used to borrow component storages. Implements `BorrowStorages` for all lifetimes.
/// Only exists because we don't have GATs in stable rust :(
pub struct StorageBorrower<T>
where
	T: Send + Sync + 'static,
{
	_phantom: PhantomData<*const T>,
}

macro_rules! impl_component_set {
    ($len:tt, $(($comp:ident, $idx:tt)),*) => {
        unsafe impl<$($comp),*> ComponentSet for ($($comp,)*)
        where
            $($comp: Component,)*
        {
            type Storages = StorageBorrower<($($comp,)*)>;

            #[allow(unused_variables)]
            unsafe fn insert(
                storages: &mut <Self::Storages as BorrowStorages>::StorageSet,
                entity: Entity,
                components: Self,
                tick: u32,
            ) {
                $(
                    storages.$idx.0.insert(entity, components.$idx, tick);
                )*
            }

            #[allow(unused_variables)]
            unsafe fn remove(
                storages: &mut <Self::Storages as BorrowStorages>::StorageSet,
                entity: Entity,
            ) -> Option<Self> {
                let components = (
                    $(storages.$idx.0.remove(entity),)*
                );

                Some((
                    $(components.$idx?,)*
                ))
            }

            #[allow(unused_variables)]
            unsafe fn delete(
                storages: &mut <Self::Storages as BorrowStorages>::StorageSet,
                entity: Entity,
            ) {
                $(
                    storages.$idx.0.remove(entity);
                )*
            }
        }

        impl<'a, $($comp),*> BorrowStorages<'a> for StorageBorrower<($($comp,)*)>
        where
            $($comp: Component,)*
        {
            type StorageSet = ($(BorrowedComponentStorage<'a, $comp>,)*);

            #[allow(unused_mut)]
            #[allow(unused_variables)]
            fn borrow_with_families(components: &'a ComponentStorages) -> (Self::StorageSet, UsedGroupFamilies) {
                let mut families = UsedGroupFamilies::default();
                (($(borrow_with_family::<$comp>(components, &mut families),)*), families)
            }

            #[allow(unused_variables)]
            fn borrow(components: &'a ComponentStorages) -> Self::StorageSet {
                ($(borrow::<$comp>(components),)*)
            }

            #[allow(unused_mut)]
            #[allow(unused_variables)]
            fn families(components: &'a ComponentStorages) -> UsedGroupFamilies {
                let mut families = UsedGroupFamilies::default();
                $(update_used_group_families::<$comp>(&mut families, components);)*
                families
            }
        }
    };
}

fn borrow_with_family<'a, T>(
	components: &'a ComponentStorages,
	families: &mut UsedGroupFamilies,
) -> BorrowedComponentStorage<'a, T>
where
	T: Component,
{
	components
		.borrow_with_familiy_mut(&TypeId::of::<T>())
		.map(|(storage, family)| unsafe {
			if let Some(family) = family {
				families.insert_unchecked(family);
			}
			BorrowedComponentStorage(TypedComponentStorage::new(storage))
		})
		.unwrap_or_else(|| panic_missing_comp::<T>())
}

fn borrow<'a, T>(components: &'a ComponentStorages) -> BorrowedComponentStorage<'a, T>
where
	T: Component,
{
	components
		.borrow_mut(&TypeId::of::<T>())
		.map(|storage| unsafe { BorrowedComponentStorage(TypedComponentStorage::new(storage)) })
		.unwrap_or_else(|| panic_missing_comp::<T>())
}

fn update_used_group_families<T>(families: &mut UsedGroupFamilies, components: &ComponentStorages)
where
	T: Component,
{
	if let Some(index) = components.group_family_of(&TypeId::of::<T>()) {
		unsafe {
			families.insert_unchecked(index);
		}
	}
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_component_set!(0,);
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
