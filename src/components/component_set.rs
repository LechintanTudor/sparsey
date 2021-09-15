use crate::components::{Component, ComponentStorages, FamilyMask};
use crate::group::iter_group_family_indexes;
use crate::storage::{ComponentStorage, Entity, EntityStorage, TypedComponentStorage};
use crate::utils::{panic_missing_comp, ChangeTicks};
use atomic_refcell::AtomicRefMut;
use std::any::TypeId;

/// Trait used to insert and remove components from the `World`.
pub unsafe trait ComponentSet
where
    Self: Sized + Send + Sync + 'static,
{
    /// Inserts the components into the storages.
    unsafe fn insert(
        storages: &mut ComponentStorages,
        entity: Entity,
        components: Self,
        ticks: ChangeTicks,
    );

    /// Creates new entities with components produced by `components_iter`.
    unsafe fn extend<I>(
        storages: &mut ComponentStorages,
        entities: &mut EntityStorage,
        components_iter: I,
        ticks: ChangeTicks,
    ) where
        I: IntoIterator<Item = Self>;

    /// Removes the components from the storages and returns them if
    /// all of them were successfully removed.
    unsafe fn remove(storages: &mut ComponentStorages, entity: Entity) -> Option<Self>;

    /// Deletes the components from storages. This is faster than removing them.
    unsafe fn delete(storages: &mut ComponentStorages, entity: Entity);
}

macro_rules! impl_component_set {
    ($len:tt, $(($comp:ident, $idx:tt)),*) => {
        unsafe impl<$($comp),*> ComponentSet for ($($comp,)*)
        where
            $($comp: Component,)*
        {
            #[allow(unused_mut)]
            #[allow(unused_variables)]
            unsafe fn insert(
                storages: &mut ComponentStorages,
                entity: Entity,
                components: Self,
                ticks: ChangeTicks,
            ) {
                let mut family_mask = 0_u16;

                $(
                    {
                        let (mut storage, mask) = get_with_family_mask_mut::<$comp>(storages);
                        storage.insert(entity, components.$idx, ticks);
                        family_mask |= mask;
                    }
                )*

                for i in iter_group_family_indexes(family_mask) {
                    storages.group_components(i, entity);
                }
            }

            #[allow(unused_mut)]
            #[allow(unused_variables)]
            unsafe fn extend<It>(
                storages: &mut ComponentStorages,
                entities: &mut EntityStorage,
                components_iter: It,
                ticks: ChangeTicks,
            )
            where
                It: IntoIterator<Item = Self>
            {
                let initial_entity_count = entities.as_ref().len();
                let mut family_mask = 0_u16;

                {
                    let mut borrowed_storages = (
                        $({
                            let (storage, mask) = borrow_with_family_mask_mut::<$comp>(storages);
                            family_mask = family_mask | mask;
                            storage
                        },)*
                    );

                    components_iter.into_iter().for_each(|components| {
                        let entity = entities.create();
                        $(borrowed_storages.$idx.insert(entity, components.$idx, ticks);)*
                    });
                }

                let new_entities = &entities.as_ref()[initial_entity_count..];

                for i in iter_group_family_indexes(family_mask) {
                    for &entity in new_entities {
                        storages.group_components(i, entity)
                    }
                }
            }

            #[allow(unused_mut)]
            #[allow(unused_variables)]
            unsafe fn remove(storages: &mut ComponentStorages, entity: Entity) -> Option<Self> {
                let mut family_mask = 0_u16;
                $(family_mask |= storages.get_family_mask(&TypeId::of::<$comp>());)*

                for i in iter_group_family_indexes(family_mask) {
                    storages.ungroup_components(i, entity);
                }

                let components = ($(get_mut::<$comp>(storages).remove(entity),)*);
                Some(($(components.$idx?,)*))
            }

            #[allow(unused_mut)]
            #[allow(unused_variables)]
            unsafe fn delete(storages: &mut ComponentStorages, entity: Entity) {
                let mut family_mask = 0_u16;
                $(family_mask |= storages.get_family_mask(&TypeId::of::<$comp>());)*

                for i in iter_group_family_indexes(family_mask) {
                    storages.ungroup_components(i, entity);
                }

                $(get_mut::<$comp>(storages).remove(entity);)*
            }
        }
    };
}

fn get_mut<T>(storages: &mut ComponentStorages) -> TypedComponentStorage<T, &mut ComponentStorage>
where
    T: Component,
{
    storages
        .get_mut(&TypeId::of::<T>())
        .map(|storage| unsafe { TypedComponentStorage::new(storage) })
        .unwrap_or_else(|| panic_missing_comp::<T>())
}

fn get_with_family_mask_mut<T>(
    storages: &mut ComponentStorages,
) -> (TypedComponentStorage<T, &mut ComponentStorage>, FamilyMask)
where
    T: Component,
{
    storages
        .get_with_family_mask_mut(&TypeId::of::<T>())
        .map(|(storage, mask)| unsafe { (TypedComponentStorage::new(storage), mask) })
        .unwrap_or_else(|| panic_missing_comp::<T>())
}

fn borrow_with_family_mask_mut<T>(
    storages: &ComponentStorages,
) -> (
    TypedComponentStorage<T, AtomicRefMut<ComponentStorage>>,
    FamilyMask,
)
where
    T: Component,
{
    storages
        .borrow_with_family_mask_mut(&TypeId::of::<T>())
        .map(|(storage, mask)| unsafe { (TypedComponentStorage::new(storage), mask) })
        .unwrap_or_else(|| panic_missing_comp::<T>())
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_component_set!(0,);
    impl_component_set!(1, (A, 0));
    impl_component_set!(2, (A, 0), (B, 1));
    impl_component_set!(3, (A, 0), (B, 1), (C, 2));
    impl_component_set!(4, (A, 0), (B, 1), (C, 2), (D, 3));
    impl_component_set!(5, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
    impl_component_set!(6, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
    impl_component_set!(7, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
    impl_component_set!(8, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7));
    impl_component_set!(9, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8));
    impl_component_set!(10, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9));
    impl_component_set!(11, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10));
    impl_component_set!(12, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11));
    impl_component_set!(13, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12));
    impl_component_set!(14, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13));
    impl_component_set!(15, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14));
    impl_component_set!(16, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14), (P, 15));
}
