use crate::component::{Component, ComponentSparseSet};
use crate::entity::{Entity, SparseVec};
use atomic_refcell::{AtomicRef, AtomicRefMut};
use core::fmt;
use core::marker::PhantomData;
use core::ops::{Index, IndexMut};
use core::ptr::NonNull;

/// Shared view over all components of type `T` in a [`World`](crate::world::World).
pub struct View<'a, T> {
    components: AtomicRef<'a, ComponentSparseSet>,
    _phantom: PhantomData<&'a [T]>,
}

impl<'a, T> View<'a, T> {
    #[inline]
    #[must_use]
    pub(crate) fn new(components: AtomicRef<'a, ComponentSparseSet>) -> Self {
        Self {
            components,
            _phantom: PhantomData,
        }
    }
}

/// Exclusive view over all components of type `T` in a [`World`](crate::world::World).
pub struct ViewMut<'a, T> {
    components: AtomicRefMut<'a, ComponentSparseSet>,
    _phantom: PhantomData<&'a mut [T]>,
}

impl<'a, T> ViewMut<'a, T>
where
    T: Component,
{
    #[inline]
    #[must_use]
    pub(crate) fn new(components: AtomicRefMut<'a, ComponentSparseSet>) -> Self {
        Self {
            components,
            _phantom: PhantomData,
        }
    }

    /// Returns a mutable reference to the component mapped to `entity`, if it exists.
    #[must_use]
    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        unsafe { self.components.get_mut::<T>(entity) }
    }

    /// Returns a mutable slice of all components in the view.
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { self.components.as_mut_slice::<T>() }
    }
}

impl<T> IndexMut<Entity> for ViewMut<'_, T>
where
    T: Component,
{
    fn index_mut(&mut self, entity: Entity) -> &mut Self::Output {
        self.get_mut(entity).unwrap()
    }
}

macro_rules! impl_view_common {
    ($View:ident) => {
        impl<'a, T> $View<'a, T>
        where
            T: Component,
        {
            /// Returns a reference to the component mapped to `entity` if it exists.
            #[must_use]
            pub fn get(&self, entity: Entity) -> Option<&T> {
                unsafe { self.components.get::<T>(entity) }
            }

            /// Returns whether `entity` is present in the view.
            #[must_use]
            pub fn contains(&self, entity: Entity) -> bool {
                self.components.contains(entity)
            }

            /// Returns the number of entities in the view.
            #[must_use]
            pub fn len(&self) -> usize {
                self.components.len()
            }

            /// Returns whether the view is empty.
            #[must_use]
            pub fn is_empty(&self) -> bool {
                self.components.is_empty()
            }

            /// Returns a slice of all entities in the view.
            #[must_use]
            pub fn entities(&self) -> &[Entity] {
                self.components.entities()
            }

            /// Returns a slice of all components in the view.
            #[must_use]
            pub fn as_slice(&self) -> &[T] {
                unsafe { self.components.as_slice::<T>() }
            }

            #[must_use]
            pub(crate) fn sparse(&self) -> &SparseVec {
                self.components.sparse()
            }

            #[must_use]
            pub(crate) fn as_non_null_ptr(&self) -> NonNull<T> {
                unsafe { self.components.as_non_null_ptr::<T>() }
            }
        }

        impl<T> Index<Entity> for $View<'_, T>
        where
            T: Component,
        {
            type Output = T;

            fn index(&self, entity: Entity) -> &Self::Output {
                self.get(entity).unwrap()
            }
        }

        impl<T> fmt::Debug for $View<'_, T>
        where
            T: Component + fmt::Debug,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let entries = self.entities().iter().zip(self.as_slice());
                f.debug_map().entries(entries).finish()
            }
        }
    };
}

impl_view_common!(View);
impl_view_common!(ViewMut);
