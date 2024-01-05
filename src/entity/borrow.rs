use crate::entity::{ComponentSparseSet, Entity, EntityStorage};
use atomic_refcell::{AtomicRef, AtomicRefMut};
use std::fmt;
use std::marker::PhantomData;

pub struct Entities<'a> {
    entities: &'a EntityStorage,
}

impl<'a> Entities<'a> {
    #[inline]
    #[must_use]
    pub(crate) fn new(entities: &'a EntityStorage) -> Self {
        Self { entities }
    }

    #[inline]
    #[must_use]
    pub fn create_atomic(&self) -> Entity {
        self.entities.create_atomic()
    }

    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[Entity] {
        self.entities.entities()
    }
}

pub struct Comp<'a, T> {
    components: AtomicRef<'a, ComponentSparseSet>,
    _phantom: PhantomData<&'a [T]>,
}

impl<'a, T> Comp<'a, T> {
    #[inline]
    #[must_use]
    pub(crate) unsafe fn new(components: AtomicRef<'a, ComponentSparseSet>) -> Self {
        Self {
            components,
            _phantom: PhantomData,
        }
    }
}

pub struct CompMut<'a, T> {
    components: AtomicRefMut<'a, ComponentSparseSet>,
    _phantom: PhantomData<&'a mut [T]>,
}

impl<'a, T> CompMut<'a, T> {
    #[inline]
    #[must_use]
    pub(crate) unsafe fn new(components: AtomicRefMut<'a, ComponentSparseSet>) -> Self {
        Self {
            components,
            _phantom: PhantomData,
        }
    }
}

macro_rules! impl_comp_common {
    ($Comp:ident) => {
        impl<T> fmt::Debug for $Comp<'_, T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Debug::fmt(&self.components, f)
            }
        }
    };
}

impl_comp_common!(Comp);
impl_comp_common!(CompMut);
