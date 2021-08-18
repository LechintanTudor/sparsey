use crate::components::{Component, ComponentStorages};
use crate::group::GroupFamilyIndexes;
use crate::storage::{ComponentStorage, Entity, TypedComponentStorage};
use crate::utils::{panic_missing_comp, ChangeTicks};
use atomic_refcell::AtomicRefMut;
use std::any::TypeId;
use std::marker::PhantomData;

/// Used internally by `ComponentSet` to manage a component storage of type `T`.
pub struct ComponentStorageRefMut<'a, T>(
	TypedComponentStorage<AtomicRefMut<'a, ComponentStorage>, T>,
);

/// Trait used to insert and remove components from the `World`.
pub unsafe trait ComponentSet
where
	Self: Sized + Send + Sync + 'static,
{
	/// Used for borrowing storages.
	type Storages: for<'a> BorrowStorages<'a>;

	/// Inserts the components into the borrowed storages.
	unsafe fn insert(
		storages: &mut <Self::Storages as BorrowStorages>::Storages,
		entity: Entity,
		components: Self,
		ticks: ChangeTicks,
	);

	/// Removes the components from the borrowed storages and returns them if
	/// all of them were successfully removed.
	unsafe fn remove(
		storages: &mut <Self::Storages as BorrowStorages>::Storages,
		entity: Entity,
	) -> Option<Self>;

	/// Deletes the components from the borrowed storages. This is faster than
	/// removing them.
	unsafe fn delete(storages: &mut <Self::Storages as BorrowStorages>::Storages, entity: Entity);
}

/// Trait implemented by `StorageBorrower` to borrow component storages.
pub trait BorrowStorages<'a> {
	/// Borrowed storages.
	type Storages;

	/// Borrows the storages.
	fn borrow(components: &'a ComponentStorages) -> Self::Storages;

	/// Returns the group family indexes.
	fn families(components: &'a ComponentStorages) -> GroupFamilyIndexes;

	/// Borrows the storages and returns the group family indexes.
	fn borrow_with_families(
		components: &'a ComponentStorages,
	) -> (Self::Storages, GroupFamilyIndexes);
}

/// Struct used to borrow component storages.
pub struct StorageBorrower<T> {
	storages: PhantomData<*const T>,
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
                storages: &mut <Self::Storages as BorrowStorages>::Storages,
                entity: Entity,
                components: Self,
                ticks: ChangeTicks,
            ) {
                $(
                    storages.$idx.0.insert(entity, components.$idx, ticks);
                )*
            }

            #[allow(unused_variables)]
            unsafe fn remove(
                storages: &mut <Self::Storages as BorrowStorages>::Storages,
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
                storages: &mut <Self::Storages as BorrowStorages>::Storages,
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
            type Storages = ($(ComponentStorageRefMut<'a, $comp>,)*);

            #[allow(unused_mut)]
            #[allow(unused_variables)]
            fn borrow_with_families(components: &'a ComponentStorages) -> (Self::Storages, GroupFamilyIndexes) {
                let mut families = GroupFamilyIndexes::default();
                (($(borrow_with_family::<$comp>(components, &mut families),)*), families)
            }

            #[allow(unused_variables)]
            fn borrow(components: &'a ComponentStorages) -> Self::Storages {
                ($(borrow::<$comp>(components),)*)
            }

            #[allow(unused_mut)]
            #[allow(unused_variables)]
            fn families(components: &'a ComponentStorages) -> GroupFamilyIndexes {
                let mut families = GroupFamilyIndexes::default();
                $(update_used_group_families::<$comp>(&mut families, components);)*
                families
            }
        }
    };
}

fn borrow<T>(components: &ComponentStorages) -> ComponentStorageRefMut<T>
where
	T: Component,
{
	components
		.borrow_mut(&TypeId::of::<T>())
		.map(|storage| unsafe { ComponentStorageRefMut(TypedComponentStorage::new(storage)) })
		.unwrap_or_else(|| panic_missing_comp::<T>())
}

fn borrow_with_family<'a, T>(
	components: &'a ComponentStorages,
	families: &mut GroupFamilyIndexes,
) -> ComponentStorageRefMut<'a, T>
where
	T: Component,
{
	components
		.borrow_with_familiy_mut(&TypeId::of::<T>())
		.map(|(storage, family)| unsafe {
			if let Some(family) = family {
				families.insert_unchecked(family);
			}
			ComponentStorageRefMut(TypedComponentStorage::new(storage))
		})
		.unwrap_or_else(|| panic_missing_comp::<T>())
}

fn update_used_group_families<T>(families: &mut GroupFamilyIndexes, components: &ComponentStorages)
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
