use crate::entity::{Component, ComponentSparseSet, Entity, EntityStorage, GroupInfo, SparseVec};
use atomic_refcell::{AtomicRef, AtomicRefMut};
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

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
    group_info: Option<GroupInfo<'a>>,
    _phantom: PhantomData<&'a [T]>,
}

impl<'a, T> Comp<'a, T> {
    #[inline]
    #[must_use]
    pub(crate) unsafe fn new(
        components: AtomicRef<'a, ComponentSparseSet>,
        group_info: Option<GroupInfo<'a>>,
    ) -> Self {
        Self {
            components,
            group_info,
            _phantom: PhantomData,
        }
    }
}

pub struct CompMut<'a, T> {
    components: AtomicRefMut<'a, ComponentSparseSet>,
    group_info: Option<GroupInfo<'a>>,
    _phantom: PhantomData<&'a mut [T]>,
}

impl<'a, T> CompMut<'a, T>
where
    T: Component,
{
    #[inline]
    #[must_use]
    pub(crate) unsafe fn new(
        components: AtomicRefMut<'a, ComponentSparseSet>,
        group_info: Option<GroupInfo<'a>>,
    ) -> Self {
        Self {
            components,
            group_info,
            _phantom: PhantomData,
        }
    }

    #[must_use]
    pub fn get_mut(&self, entity: Entity) -> Option<&mut T> {
        unsafe { self.components.get_mut(entity) }
    }

    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { self.components.as_mut_slice() }
    }

    #[must_use]
    pub fn split_mut(&mut self) -> (&[Entity], &SparseVec, &mut [T]) {
        unsafe { self.components.split_mut() }
    }
}

impl<T> IndexMut<Entity> for CompMut<'_, T>
where
    T: Component,
{
    fn index_mut(&mut self, entity: Entity) -> &mut Self::Output {
        self.get_mut(entity).unwrap()
    }
}

macro_rules! impl_comp_common {
    ($Comp:ident) => {
        impl<T> $Comp<'_, T>
        where
            T: Component,
        {
            #[must_use]
            pub fn get(&self, entity: Entity) -> Option<&T> {
                unsafe { self.components.get(entity) }
            }

            #[must_use]
            pub fn contains(&self, entity: Entity) -> bool {
                self.components.contains(entity)
            }

            #[must_use]
            pub fn len(&self) -> usize {
                self.components.len()
            }

            #[must_use]
            pub fn is_empty(&self) -> bool {
                self.components.is_empty()
            }

            #[must_use]
            pub fn group_info(&self) -> Option<GroupInfo<'_>> {
                self.group_info
            }

            #[must_use]
            pub fn entities(&self) -> &[Entity] {
                self.components.entities()
            }

            #[must_use]
            pub fn as_slice(&self) -> &[T] {
                unsafe { self.components.as_slice() }
            }

            #[must_use]
            pub fn split(&self) -> (&[Entity], &SparseVec, &[T]) {
                unsafe { self.components.split() }
            }
        }

        impl<T> Index<Entity> for $Comp<'_, T>
        where
            T: Component,
        {
            type Output = T;

            fn index(&self, entity: Entity) -> &Self::Output {
                self.get(entity).unwrap()
            }
        }

        impl<T> fmt::Debug for $Comp<'_, T>
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

impl_comp_common!(Comp);
impl_comp_common!(CompMut);
