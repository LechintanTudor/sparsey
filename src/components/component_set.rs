use crate::components::{ComponentStorages, FamilyMask, GroupMask};
use crate::storage::{Component, ComponentStorage, Entity, EntityStorage};
use crate::utils::panic_missing_comp;
use std::any::TypeId;
use std::iter;

/// Manages components in component storages.
/// Implemented for `Component` tuples up to arity 16.
///
/// # Safety
/// All operations must preserve component grouping.
pub unsafe trait ComponentSet: Sized + Send + Sync + 'static {
    /// Result of the `remove` operation.
    type RemoveResult: Send + Sync + 'static;

    /// Adds the given `components` to `entity`.
    fn insert(storages: &mut ComponentStorages, entity: Entity, components: Self);

    /// Creates new entities with the components produced by the iterator.
    fn extend<'a, I>(
        entities: &'a mut EntityStorage,
        storages: &mut ComponentStorages,
        components_iter: I,
    ) -> &'a [Entity]
    where
        I: IntoIterator<Item = Self>;

    /// Removes a component set from `entity` and returns the components.
    #[must_use = "use `delete` to discard the components"]
    fn remove(storages: &mut ComponentStorages, entity: Entity) -> Self::RemoveResult;

    /// Deletes a component set from `entity`. This is faster than removing them.
    fn delete(storages: &mut ComponentStorages, entity: Entity);
}

unsafe impl ComponentSet for () {
    type RemoveResult = ();

    #[inline]
    fn insert(_storages: &mut ComponentStorages, _entity: Entity, _components: Self) {
        // Empty
    }

    fn extend<'a, I>(
        entities: &'a mut EntityStorage,
        _storages: &mut ComponentStorages,
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

    #[inline]
    fn remove(_storages: &mut ComponentStorages, _entity: Entity) -> Self::RemoveResult {
        ()
    }

    #[inline]
    fn delete(_storages: &mut ComponentStorages, _entity: Entity) {
        // Empty
    }
}

macro_rules! impl_component_set {
    ($(($comp:ident, $idx:tt)),+) => {
        unsafe impl<$($comp),+> ComponentSet for ($($comp,)+)
        where
            $($comp: Component,)+
        {
            type RemoveResult = ($(Option<$comp>,)+);

            #[allow(clippy::eval_order_dependence)]
            fn insert(
                storages: &mut ComponentStorages,
                entity: Entity,
                components: Self,
            ) {
                let mut family_mask = FamilyMask::default();

                $(
                    {
                        let (storage, mask) = get_as_ptr_with_family_mask::<$comp>(storages);
                        unsafe { (*storage).insert::<$comp>(entity, components.$idx); }
                        family_mask |= mask;
                    }
                )+

                unsafe {
                    storages.group_families(family_mask, iter::once(entity));
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
                    let storage_ptrs = (
                        $({
                            let (storage, mask) = get_as_ptr_with_family_mask::<$comp>(storages);
                            family_mask |= mask;
                            storage
                        },)+
                    );

                    components_iter.into_iter().for_each(|components| {
                        let entity = entities.create();

                        unsafe {
                            $(
                                (*storage_ptrs.$idx).insert::<$comp>(entity, components.$idx);
                            )+
                        }
                    });
                }

                unsafe {
                    let new_entities = entities.get_unchecked(initial_entity_count..);
                    storages.group_families(family_mask, new_entities.iter().copied());
                    new_entities
                }
            }

            #[allow(clippy::eval_order_dependence)]
            fn remove(storages: &mut ComponentStorages, entity: Entity) -> Self::RemoveResult {
                let mut family_mask = FamilyMask::default();
                let mut group_mask = GroupMask::default();

                let storage_ptrs = ($(
                    {
                        let (storage, family, group) = get_as_ptr_with_masks::<$comp>(storages);

                        family_mask |= family;
                        group_mask |= group;

                        storage
                    },
                )+);

                unsafe {
                    storages.ungroup_families(family_mask, group_mask, iter::once(entity));
                    ($((*storage_ptrs.$idx).remove::<$comp>(entity),)+)
                }
            }

            #[allow(clippy::eval_order_dependence)]
            fn delete(storages: &mut ComponentStorages, entity: Entity) {
                let mut family_mask = FamilyMask::default();
                let mut group_mask = GroupMask::default();

                let storage_ptrs = ($(
                    {
                        let (storage, family, group) = get_as_ptr_with_masks::<$comp>(storages);

                        family_mask |= family;
                        group_mask |= group;

                        storage
                    },
                )+);

                unsafe {
                    storages.ungroup_families(family_mask, group_mask, iter::once(entity));
                    $((*storage_ptrs.$idx).delete_typed::<$comp>(entity);)+
                }
            }
        }
    };
}

fn get_as_ptr_with_family_mask<T>(
    storages: &ComponentStorages,
) -> (*mut ComponentStorage, FamilyMask)
where
    T: Component,
{
    let (storage, family_mask) = storages
        .get_as_ptr_with_family_mask(&TypeId::of::<T>())
        .unwrap_or_else(|| panic_missing_comp::<T>());

    (storage.as_ptr(), family_mask)
}

fn get_as_ptr_with_masks<T>(
    storages: &ComponentStorages,
) -> (*mut ComponentStorage, FamilyMask, GroupMask)
where
    T: Component,
{
    let (storage, family_mask, group_mask) = storages
        .get_as_ptr_with_masks(&TypeId::of::<T>())
        .unwrap_or_else(|| panic_missing_comp::<T>());

    (storage.as_ptr(), family_mask, group_mask)
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
