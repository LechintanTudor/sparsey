use crate::components::{iter_bit_indexes, Component, ComponentStorages, FamilyMask, GroupMask};
use crate::storage::{ComponentStorage, Entity, EntityStorage};
use crate::utils::{impl_generic_tuple_1_16, panic_missing_comp};
use atomic_refcell::AtomicRefMut;
use std::any::TypeId;

/// Handles adding/removing `Component`s to/from storages.
/// Trait implemented by `Component` tuples up to arity 16.
///
/// # Safety
/// All operations must preserve component grouping.
pub unsafe trait ComponentSet: Sized + Send + Sync + 'static {
    /// Inserts the `entity` and its `components` into the `storages`.
    fn insert(storages: &mut ComponentStorages, entity: Entity, components: Self);

    /// Creates new entities with `Component`s produced by `components_iter`.
    /// Returns the newly created entities as a slice.
    fn extend<'a, I>(
        entities: &'a mut EntityStorage,
        storages: &mut ComponentStorages,
        components_iter: I,
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
    #[inline(always)]
    fn insert(_: &mut ComponentStorages, _: Entity, _: Self) {
        // Empty
    }

    fn extend<'a, I>(
        entities: &'a mut EntityStorage,
        _: &mut ComponentStorages,
        components_iter: I,
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

    #[inline(always)]
    fn remove(_: &mut ComponentStorages, _: Entity) -> Option<Self> {
        Some(())
    }

    #[inline(always)]
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
            ) {
                let mut family_mask = FamilyMask::default();

                $(
                    {
                        let (storage, mask) = get_with_family_mask_mut::<$comp>(storages);
                        unsafe { storage.insert::<$comp>(entity, components.$idx); }
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

                        unsafe {
                            $(borrowed_storages.$idx.insert::<$comp>(entity, components.$idx);)+
                        }
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

                let components = unsafe { (
                    $((&mut *storage_ptrs.$idx).remove::<$comp>(entity),)+
                ) };

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
                    $((&mut *storage_ptrs.$idx).remove::<$comp>(entity);)+
                }
            }
        }
    };
}

fn get_with_family_mask_mut<T>(
    storages: &mut ComponentStorages,
) -> (&mut ComponentStorage, FamilyMask)
where
    T: Component,
{
    storages
        .get_with_family_mask_mut(&TypeId::of::<T>())
        .unwrap_or_else(|| panic_missing_comp::<T>())
}

fn borrow_with_family_mask_mut<T>(
    storages: &ComponentStorages,
) -> (AtomicRefMut<ComponentStorage>, FamilyMask)
where
    T: Component,
{
    storages
        .borrow_with_family_mask_mut(&TypeId::of::<T>())
        .unwrap_or_else(|| panic_missing_comp::<T>())
}

impl_generic_tuple_1_16!(impl_component_set);
