use crate::components::{Component, ComponentStorages, FamilyMask, FamilyMaskIter, GroupMask};
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
    /// Returns the newly created entities as a slice.
    unsafe fn extend<'a, I>(
        entities: &'a mut EntityStorage,
        storages: &mut ComponentStorages,
        components_iter: I,
        ticks: ChangeTicks,
    ) -> &'a [Entity]
    where
        I: IntoIterator<Item = Self>;

    /// Removes the components from the storages and returns them if
    /// all of them were successfully removed.
    #[must_use = "use `delete` to discard the components"]
    unsafe fn remove(storages: &mut ComponentStorages, entity: Entity) -> Option<Self>;

    /// Deletes the components from storages. This is faster than removing them.
    unsafe fn delete(storages: &mut ComponentStorages, entity: Entity);
}

macro_rules! impl_component_set {
    ($(($comp:ident, $idx:tt)),*) => {
        unsafe impl<$($comp),*> ComponentSet for ($($comp,)*)
        where
            $($comp: Component,)*
        {
            #[allow(clippy::eval_order_dependence)]
            #[allow(unused_mut)]
            #[allow(unused_variables)]
            unsafe fn insert(
                storages: &mut ComponentStorages,
                entity: Entity,
                components: Self,
                ticks: ChangeTicks,
            ) {
                let mut family_mask = FamilyMask::default();

                $(
                    {
                        let (mut storage, mask) = get_with_family_mask_mut::<$comp>(storages);
                        storage.insert(entity, components.$idx, ticks);
                        family_mask |= mask;
                    }
                )*

                for i in FamilyMaskIter::new(family_mask) {
                    storages.group_components(i, Some(&entity));
                }
            }

            #[allow(clippy::eval_order_dependence)]
            #[allow(unused_mut)]
            #[allow(unused_variables)]
            unsafe fn extend<'a, It>(
                entities: &'a mut EntityStorage,
                storages: &mut ComponentStorages,
                components_iter: It,
                ticks: ChangeTicks,
            ) -> &'a [Entity]
            where
                It: IntoIterator<Item = Self>
            {
                let initial_entity_count = entities.as_ref().len();
                let mut family_mask = FamilyMask::default();

                {
                    let mut borrowed_storages = (
                        $({
                            let (storage, mask) = borrow_with_family_mask_mut::<$comp>(storages);
                            family_mask |= mask;
                            storage
                        },)*
                    );

                    components_iter.into_iter().for_each(|components| {
                        let entity = entities.create();
                        $(borrowed_storages.$idx.insert(entity, components.$idx, ticks);)*
                    });
                }

                let new_entities = entities.get_unchecked(initial_entity_count..);

                for i in FamilyMaskIter::new(family_mask) {
                    storages.group_components(i, new_entities);
                }

                new_entities
            }

            #[allow(clippy::eval_order_dependence)]
            #[allow(unused_mut)]
            #[allow(unused_variables)]
            unsafe fn remove(storages: &mut ComponentStorages, entity: Entity) -> Option<Self> {
                let mut family_mask = FamilyMask::default();
                let mut group_mask = GroupMask::default();

                let storage_ptrs = ($(
                    {
                        let (storage_ptr, family, group) = storages.get_as_ptr_with_masks(&TypeId::of::<$comp>());

                        if storage_ptr.is_null() {
                            panic_missing_comp::<$comp>();
                        }

                        family_mask |= family;
                        group_mask |= group;
                        storage_ptr
                    },
                )*);

                for i in FamilyMaskIter::new(family_mask) {
                    storages.ungroup_components(i, group_mask, Some(&entity));
                }

                let components = ($(
                    TypedComponentStorage::<$comp, _>::new(&mut *storage_ptrs.$idx).remove(entity),
                )*);

                Some(($(components.$idx?,)*))
            }

            #[allow(clippy::eval_order_dependence)]
            #[allow(unused_mut)]
            #[allow(unused_variables)]
            unsafe fn delete(storages: &mut ComponentStorages, entity: Entity) {
                let mut family_mask = FamilyMask::default();
                let mut group_mask = GroupMask::default();

                let storage_ptrs = ($(
                    {
                        let (storage_ptr, family, group) = storages.get_as_ptr_with_masks(&TypeId::of::<$comp>());

                        if storage_ptr.is_null() {
                            panic_missing_comp::<$comp>();
                        }

                        family_mask |= family;
                        group_mask |= group;
                        storage_ptr
                    },
                )*);

                for i in FamilyMaskIter::new(family_mask) {
                    storages.ungroup_components(i, group_mask, Some(&entity));
                }

                $(TypedComponentStorage::<$comp, _>::new(&mut *storage_ptrs.$idx).remove(entity);)*
            }
        }
    };
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

    impl_component_set!();
    impl_component_set!((A, 0));
    impl_component_set!((A, 0), (B, 1));
    impl_component_set!((A, 0), (B, 1), (C, 2));
    impl_component_set!((A, 0), (B, 1), (C, 2), (D, 3));
    impl_component_set!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
    impl_component_set!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
    impl_component_set!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
    impl_component_set!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7));
    impl_component_set!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8));
    impl_component_set!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9));
    impl_component_set!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10));
    impl_component_set!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11));
    impl_component_set!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12));
    impl_component_set!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13));
    impl_component_set!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14));
    impl_component_set!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14), (P, 15));
}
