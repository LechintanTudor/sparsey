use crate::components::{iter_bit_indexes, Component, ComponentStorages, FamilyMask, GroupMask};
use crate::storage::{ComponentStorage, Entity, EntityStorage, TypedComponentStorage};
use crate::utils::{panic_missing_comp, ChangeTicks};
use atomic_refcell::AtomicRefMut;
use std::any::TypeId;

/// Trait implemented by `Component` tuples up to arity 16.
///  Handles adding/removing components to/from storages.
///
/// # Safety
/// All operations must preserve component grouping.
pub unsafe trait ComponentSet
where
    Self: Sized + Send + Sync + 'static,
{
    /// Inserts the `entity` and its `components` into the `storages`.
    fn insert(
        storages: &mut ComponentStorages,
        entity: Entity,
        components: Self,
        ticks: ChangeTicks,
    );

    /// Creates new entities with `Component`s produced by `components_iter`.
    /// Returns the newly created entities as a slice.
    fn extend<'a, I>(
        entities: &'a mut EntityStorage,
        storages: &mut ComponentStorages,
        components_iter: I,
        ticks: ChangeTicks,
    ) -> &'a [Entity]
    where
        I: IntoIterator<Item = Self>;

    /// Removes the `entity` and its `Component`s from the `storages` and
    /// returns the `Component`s if they were successfully removed.
    #[must_use = "use `delete` to discard the components"]
    fn remove(storages: &mut ComponentStorages, entity: Entity) -> Option<Self>;

    /// Deletes the `entity` and its `Component`s from the `storages`. This is
    /// faster than removing them.
    fn delete(storages: &mut ComponentStorages, entity: Entity);
}

unsafe impl ComponentSet for () {
    #[inline]
    fn insert(_: &mut ComponentStorages, _: Entity, _: Self, _: ChangeTicks) {
        // Empty
    }

    fn extend<'a, I>(
        entities: &'a mut EntityStorage,
        _: &mut ComponentStorages,
        components_iter: I,
        _: ChangeTicks,
    ) -> &'a [Entity]
    where
        I: IntoIterator<Item = Self>,
    {
        let initial_entity_count = entities.len();
        components_iter.into_iter().for_each(|_| {
            entities.create();
        });
        unsafe { entities.get_unchecked(initial_entity_count..) }
    }

    #[inline]
    fn remove(_: &mut ComponentStorages, _: Entity) -> Option<Self> {
        Some(())
    }

    #[inline]
    fn delete(_: &mut ComponentStorages, _: Entity) {
        // Empty
    }
}

macro_rules! impl_component_set {
    ($(($comp:ident, $idx:tt)),+) => {
        unsafe impl<$($comp),+> ComponentSet for ($($comp,)+)
        where
            $($comp: Component,)+
        {
            #[allow(clippy::eval_order_dependence)]
            fn insert(
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
                )+

                for i in iter_bit_indexes(family_mask) {
                    unsafe {
                        storages.group_components(i, Some(&entity));
                    }
                }
            }

            #[allow(clippy::eval_order_dependence)]
            fn extend<'a, It>(
                entities: &'a mut EntityStorage,
                storages: &mut ComponentStorages,
                components_iter: It,
                ticks: ChangeTicks,
            ) -> &'a [Entity]
            where
                It: IntoIterator<Item = Self>
            {
                let initial_entity_count = entities.len();
                let mut family_mask = FamilyMask::default();

                {
                    let mut borrowed_storages = (
                        $({
                            let (storage, mask) = borrow_with_family_mask_mut::<$comp>(storages);
                            family_mask |= mask;
                            storage
                        },)+
                    );

                    components_iter.into_iter().for_each(|components| {
                        let entity = entities.create();
                        $(borrowed_storages.$idx.insert(entity, components.$idx, ticks);)+
                    });
                }

                let new_entities = unsafe { entities.get_unchecked(initial_entity_count..) };

                for i in iter_bit_indexes(family_mask) {
                    unsafe {
                        storages.group_components(i, new_entities);
                    }
                }

                new_entities
            }

            #[allow(clippy::eval_order_dependence)]
            fn remove(storages: &mut ComponentStorages, entity: Entity) -> Option<Self> {
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
                )+);

                for i in iter_bit_indexes(family_mask) {
                    unsafe {
                        storages.ungroup_components(i, group_mask, Some(&entity));
                    }
                }

                let components = unsafe { ($(
                    TypedComponentStorage::<$comp, _>::new(&mut *storage_ptrs.$idx).remove(entity),
                )+) };

                Some(($(components.$idx?,)+))
            }

            #[allow(clippy::eval_order_dependence)]
            fn delete(storages: &mut ComponentStorages, entity: Entity) {
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
                )+);

                for i in iter_bit_indexes(family_mask) {
                    unsafe {
                        storages.ungroup_components(i, group_mask, Some(&entity));
                    }
                }

                unsafe {
                    $(TypedComponentStorage::<$comp, _>::new(&mut *storage_ptrs.$idx).remove(entity);)+
                }
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
