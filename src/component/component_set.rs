use crate::component::{group, panic_missing_comp, ungroup, Component, GroupMask};
use crate::entity::Entity;
use crate::World;
use core::any::TypeId;

/// Handles insert and remove operations for components stored in a [`World`].
///
/// # Safety
///
/// This trait is considered an implementation detail and cannot be safely
/// implemented outside the crate.
pub unsafe trait ComponentSet {
    /// The components returned by [`remove`](Self::remove) operations.
    type Remove;

    /// Adds the given `components` to `entity`.
    fn insert(world: &mut World, entity: Entity, components: Self);

    /// Creates new entities from the components produced by the iterator.
    ///
    /// Returns the newly created entities as a slice.
    fn extend<TComponents>(world: &mut World, components: TComponents) -> &[Entity]
    where
        TComponents: IntoIterator<Item = Self>;

    /// Removes components from the given `entity`.
    ///
    /// Returns the components that were successfully removed.
    #[must_use = "Use `delete` to discard the components."]
    fn remove(world: &mut World, entity: Entity) -> Self::Remove;

    /// Removes components from the given `entity`.
    fn delete(world: &mut World, entity: Entity);
}

macro_rules! impl_component_set {
    ($(($Comp:ident, $idx:tt)),*) => {
        unsafe impl<$($Comp,)*> ComponentSet for ($($Comp,)*)
        where
            $($Comp: Component,)*
        {
            type Remove = ($(Option<$Comp>,)*);

            fn insert(world: &mut World, entity: Entity, components: Self) {
                let mut group_mask = GroupMask::EMPTY;

                $({
                    let metadata = world
                        .components
                        .metadata
                        .get(&TypeId::of::<$Comp>())
                        .unwrap_or_else(|| panic_missing_comp::<$Comp>());

                    group_mask |= metadata.insert_mask;

                    unsafe {
                        world
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
                            &mut world.components.components,
                            &mut world.components.groups,
                            group_mask,
                            entity,
                        );
                    }
                }
            }

            fn extend<TComponents>(world: &mut World, components: TComponents) -> &[Entity]
            where
                TComponents: IntoIterator<Item = Self>,
            {
                let mut group_mask = GroupMask::EMPTY;

                let sparse_sets = ($({
                    let metadata = world
                        .components
                        .metadata
                        .get(&TypeId::of::<$Comp>())
                        .unwrap_or_else(|| panic_missing_comp::<$Comp>());

                    group_mask |= metadata.insert_mask;

                    unsafe {
                        world
                            .components
                            .components
                            .get_unchecked(metadata.storage_index)
                            .as_ptr()
                    }
                },)*);

                let start_entity = world.entities.len();

                components.into_iter().for_each(|components| {
                    let entity = world.entities.create();

                    unsafe {$(
                        (*sparse_sets.$idx).insert(entity, components.$idx);
                    )*}
                });

                let new_entities = unsafe {
                    world.entities.as_slice().get_unchecked(start_entity..)
                };

                if group_mask.0 != 0 {
                    for &entity in new_entities {
                        unsafe {
                            group(
                                &mut world.components.components,
                                &mut world.components.groups,
                                group_mask,
                                entity,
                            );
                        }
                    }
                }

                new_entities
            }

            fn remove(world: &mut World, entity: Entity) -> Self::Remove {
                let mut group_mask = GroupMask::EMPTY;

                let sparse_sets = ($({
                    let metadata = world
                        .components
                        .metadata
                        .get(&TypeId::of::<$Comp>())
                        .unwrap_or_else(|| panic_missing_comp::<$Comp>());

                    group_mask |= metadata.delete_mask;

                    unsafe {
                        world
                            .components
                            .components
                            .get_unchecked(metadata.storage_index)
                            .as_ptr()
                    }
                },)*);

                unsafe {
                    if group_mask != GroupMask::EMPTY {
                        ungroup(
                            &mut world.components.components,
                            &mut world.components.groups,
                            group_mask,
                            entity,
                        );
                    }

                    ($(
                        (*sparse_sets.$idx).remove::<$Comp>(entity),
                    )*)
                }
            }

            fn delete(world: &mut World, entity: Entity) {
                let mut group_mask = GroupMask::EMPTY;

                let sparse_sets = ($({
                    let metadata = world
                        .components
                        .metadata
                        .get(&TypeId::of::<$Comp>())
                        .unwrap_or_else(|| panic_missing_comp::<$Comp>());

                    group_mask |= metadata.delete_mask;

                    unsafe {
                        world
                            .components
                            .components
                            .get_unchecked(metadata.storage_index)
                            .as_ptr()
                    }
                },)*);

                unsafe {
                    if group_mask != GroupMask::EMPTY {
                        ungroup(
                            &mut world.components.components,
                            &mut world.components.groups,
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

unsafe impl ComponentSet for () {
    type Remove = ();

    #[inline(always)]
    fn insert(_world: &mut World, _entity: Entity, _components: Self) {
        // Empty
    }

    fn extend<TComponents>(world: &mut World, components: TComponents) -> &[Entity]
    where
        TComponents: IntoIterator<Item = Self>,
    {
        let start_entity = world.entities.len();

        components.into_iter().for_each(|()| {
            let _ = world.entities.create();
        });

        unsafe { world.entities.as_slice().get_unchecked(start_entity..) }
    }

    #[inline(always)]
    fn remove(_world: &mut World, _entity: Entity) -> Self::Remove {
        // Empty
    }

    #[inline(always)]
    fn delete(_world: &mut World, _entity: Entity) {
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
