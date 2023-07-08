use crate::components::GroupInfo;
use crate::storage::{Component, ComponentStorage, Entity, SparseArray};
use atomic_refcell::{AtomicRef, AtomicRefMut};
use std::fmt;
use std::marker::PhantomData;

/// Shared view over all components of type `T` in a world.
pub struct Comp<'a, T> {
    storage: AtomicRef<'a, ComponentStorage>,
    group_info: Option<GroupInfo<'a>>,
    _phantom: PhantomData<&'a [T]>,
}

impl<'a, T> Comp<'a, T>
where
    T: Component,
{
    pub(crate) unsafe fn new(
        storage: AtomicRef<'a, ComponentStorage>,
        group_info: Option<GroupInfo<'a>>,
    ) -> Self {
        Self {
            storage,
            group_info,
            _phantom: PhantomData,
        }
    }
}

/// Exclusive view over all components of type `T` in a world.
pub struct CompMut<'a, T> {
    storage: AtomicRefMut<'a, ComponentStorage>,
    group_info: Option<GroupInfo<'a>>,
    _phantom: PhantomData<&'a mut [T]>,
}

impl<'a, T> CompMut<'a, T>
where
    T: Component,
{
    pub(crate) unsafe fn new(
        storage: AtomicRefMut<'a, ComponentStorage>,
        group_info: Option<GroupInfo<'a>>,
    ) -> Self {
        Self {
            storage,
            group_info,
            _phantom: PhantomData,
        }
    }

    /// Mutably returns the component mapped to `entity` if it exists.
    #[must_use]
    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        unsafe { self.storage.get_mut::<T>(entity) }
    }

    /// Returns all components in the view as a mutable slice.
    #[must_use]
    pub fn components_mut(&mut self) -> &mut [T] {
        unsafe { self.storage.components_mut::<T>() }
    }

    #[must_use]
    pub(crate) fn split_mut(&mut self) -> (&[Entity], &SparseArray, &mut [T]) {
        unsafe { self.storage.split_mut::<T>() }
    }
}

macro_rules! impl_comp_common {
    ($Comp:ident) => {
        impl<'a, T> $Comp<'a, T>
        where
            T: Component,
        {
            /// Returns the component mapped to `entity` if it exists.
            #[must_use]
            pub fn get(&self, entity: Entity) -> Option<&T> {
                unsafe { self.storage.get::<T>(entity) }
            }

            /// Returns `true` if the view contains `entity`.
            #[must_use]
            pub fn contains(&self, entity: Entity) -> bool {
                self.storage.contains(entity)
            }

            /// Returns the number of components in the view.
            #[must_use]
            pub fn len(&self) -> usize {
                self.storage.len()
            }

            /// Returns `true` if the view contains no components.
            #[must_use]
            pub fn is_empty(&self) -> bool {
                self.storage.is_empty()
            }

            /// Returns all components in the view as a slice.
            #[must_use]
            pub fn components(&self) -> &[T] {
                unsafe { self.storage.components::<T>() }
            }

            /// Returns all entities in the view as a slice.
            #[must_use]
            pub fn entities(&self) -> &[Entity] {
                self.storage.entities()
            }

            #[must_use]
            pub(crate) fn group_info(&self) -> Option<GroupInfo<'a>> {
                self.group_info.clone()
            }

            #[must_use]
            pub(crate) fn split(&self) -> (&[Entity], &SparseArray, &[T]) {
                unsafe { self.storage.split::<T>() }
            }
        }

        impl<T> fmt::Debug for $Comp<'_, T>
        where
            T: Component + fmt::Debug,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let entries = self.entities().iter().zip(self.components());
                f.debug_map().entries(entries).finish()
            }
        }
    };
}

impl_comp_common!(Comp);
impl_comp_common!(CompMut);
