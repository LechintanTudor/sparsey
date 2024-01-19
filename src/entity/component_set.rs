use crate::entity::{
    group, panic_missing_comp, ungroup, Component, Entity, EntityStorage, GroupMask,
};
use std::any::TypeId;

/// Handles insert and remove operations for components stored in an
/// [`EntitySotrage`](crate::entity::EntityStorage).
pub unsafe trait ComponentSet {
    /// The components returned by [`remove`](Self::remove).
    type Remove;

    /// Adds the given `components` to `entity`.
    fn insert(entities: &mut EntityStorage, entity: Entity, components: Self);

    /// Creates new entities from the components produced by the iterator.
    ///
    /// Returns the newly created entities as a slice.
    fn extend<TComponents>(entities: &mut EntityStorage, components: TComponents) -> &[Entity]
    where
        TComponents: IntoIterator<Item = Self>;

    /// Removes components from the given `entity`.
    ///
    /// Returns the components that were successfully removed.
    #[must_use = "Use `delete` to discard the components."]
    fn remove(entities: &mut EntityStorage, entity: Entity) -> Self::Remove;

    /// Removes components from the given `entity`.
    fn delete(entities: &mut EntityStorage, entity: Entity);
}

macro_rules! impl_component_set {
    ($(($Comp:ident, $idx:tt)),*) => {
        unsafe impl<$($Comp,)*> ComponentSet for ($($Comp,)*)
        where
            $($Comp: Component,)*
        {
            type Remove = ($(Option<$Comp>,)*);

            fn insert(entities: &mut EntityStorage, entity: Entity, components: Self) {
                let mut group_mask = GroupMask(0);

                $({
                    let metadata = entities
                        .components
                        .metadata
                        .get(&TypeId::of::<$Comp>())
                        .unwrap_or_else(|| panic_missing_comp::<$Comp>());

                    group_mask |= metadata.insert_mask;

                    unsafe {
                        entities
                            .components
                            .components
                            .get_unchecked_mut(metadata.storage_index)
                            .get_mut()
                            .insert(entity, components.$idx);
                    }
                })*

                if group_mask.0 != 0 {
                    unsafe {
                        group(
                            &mut entities.components.components,
                            &mut entities.components.groups,
                            group_mask,
                            entity,
                        );
                    }
                }
            }

            fn extend<TComponents>(entities: &mut EntityStorage, components: TComponents) -> &[Entity]
            where
                TComponents: IntoIterator<Item = Self>,
            {
                let mut group_mask = GroupMask(0);

                let sparse_sets = ($({
                    let metadata = entities
                        .components
                        .metadata
                        .get(&TypeId::of::<$Comp>())
                        .unwrap_or_else(|| panic_missing_comp::<$Comp>());

                    group_mask |= metadata.delete_mask;

                    unsafe {
                        entities
                            .components
                            .components
                            .get_unchecked(metadata.storage_index)
                            .as_ptr()
                    }
                },)*);

                let start_entity = entities.entities.len();
                let mut allocate_entity = || entities.create_empty_entity();

                components.into_iter().for_each(move |components| {
                    let entity = allocate_entity();

                    unsafe {$(
                        (*sparse_sets.$idx).insert(entity, components.$idx);
                    )*}
                });

                let new_entities = &entities.entities.as_slice()[start_entity..];

                if group_mask.0 != 0 {
                    for &entity in new_entities {
                        unsafe {
                            group(
                                &mut entities.components.components,
                                &mut entities.components.groups,
                                group_mask,
                                entity,
                            );
                        }
                    }
                }

                new_entities
            }

            fn remove(entities: &mut EntityStorage, entity: Entity) -> Self::Remove {
                let mut group_mask = GroupMask(0);

                let sparse_sets = ($({
                    let metadata = entities
                        .components
                        .metadata
                        .get(&TypeId::of::<$Comp>())
                        .unwrap_or_else(|| panic_missing_comp::<$Comp>());

                    group_mask |= metadata.delete_mask;

                    unsafe {
                        entities
                            .components
                            .components
                            .get_unchecked(metadata.storage_index)
                            .as_ptr()
                    }
                },)*);

                unsafe {
                    if group_mask.0 != 0 {
                        ungroup(
                            &mut entities.components.components,
                            &mut entities.components.groups,
                            group_mask,
                            entity,
                        );
                    }

                    ($(
                        (*sparse_sets.$idx).remove::<$Comp>(entity),
                    )*)
                }
            }

            fn delete(entities: &mut EntityStorage, entity: Entity) {
                let mut group_mask = GroupMask(0);

                let sparse_sets = ($({
                    let metadata = entities
                        .components
                        .metadata
                        .get(&TypeId::of::<$Comp>())
                        .unwrap_or_else(|| panic_missing_comp::<$Comp>());

                    group_mask |= metadata.delete_mask;

                    unsafe {
                        entities
                            .components
                            .components
                            .get_unchecked(metadata.storage_index)
                            .as_ptr()
                    }
                },)*);

                unsafe {
                    if group_mask.0 != 0 {
                        ungroup(
                            &mut entities.components.components,
                            &mut entities.components.groups,
                            group_mask,
                            entity,
                        );
                    }

                    $(
                        (*sparse_sets.$idx).delete::<$Comp>(entity);
                    )*
                }
            }
        }
    };
}

#[allow(unused_variables)]
unsafe impl ComponentSet for () {
    type Remove = ();

    #[inline(always)]
    fn insert(entities: &mut EntityStorage, entity: Entity, components: Self) {
        // Empty
    }

    fn extend<TComponents>(entities: &mut EntityStorage, components: TComponents) -> &[Entity]
    where
        TComponents: IntoIterator<Item = Self>,
    {
        let start_entity = entities.entities.len();

        components.into_iter().for_each(|()| {
            let _ = entities.create_empty_entity();
        });

        &entities.entities.as_slice()[start_entity..]
    }

    #[inline(always)]
    fn remove(entities: &mut EntityStorage, entity: Entity) -> Self::Remove {
        // Empty
    }

    #[inline(always)]
    fn delete(entities: &mut EntityStorage, entity: Entity) {
        // Empty
    }
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
