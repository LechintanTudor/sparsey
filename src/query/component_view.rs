use crate::entity::{Comp, CompMut, Component, Entity, GroupInfo, SparseVec};
use std::ops::Range;

/// Represents a view over components of a given type.
#[allow(clippy::len_without_is_empty)]
pub unsafe trait ComponentView {
    /// Pointer to the underlying component type.
    type Ptr: Copy;

    /// Reference to the underlying component type.
    type Ref<'a>
    where
        Self: 'a;

    /// Slice of elements of the underlying component type.
    type Slice<'a>
    where
        Self: 'a;

    /// Returns a reference to the component mapped to `entity`, if any.
    #[must_use]
    fn get<'a>(self, entity: Entity) -> Option<Self::Ref<'a>>
    where
        Self: 'a;

    /// Returns whether `entity` is present in the view.
    #[must_use]
    fn contains(self, entity: Entity) -> bool;

    /// Returns the number of components in the view.
    #[must_use]
    fn len(&self) -> usize;

    /// Returns the group info associated with the view, if any.
    #[must_use]
    fn group_info(&self) -> Option<GroupInfo>;

    /// Splits the view into its entities, sparse vec and pointer to the components.
    #[must_use]
    fn split<'a>(self) -> (&'a [Entity], &'a SparseVec, Self::Ptr)
    where
        Self: 'a;

    /// Offsets the given pointer by `index`.
    #[must_use]
    unsafe fn add_to_ptr(ptr: Self::Ptr, index: usize) -> Self::Ptr;

    /// Returns a reference to the component at the given `index`.
    #[must_use]
    unsafe fn get_from_ptr<'a>(ptr: Self::Ptr, index: usize) -> Self::Ref<'a>
    where
        Self: 'a;

    /// Returns the slice of entities at the given `range`.
    #[must_use]
    unsafe fn get_entities_unchecked<'a>(self, range: Range<usize>) -> &'a [Entity]
    where
        Self: 'a;

    /// Returns the slice of components at the given `range`.
    #[must_use]
    unsafe fn get_components_unchecked<'a>(self, range: Range<usize>) -> Self::Slice<'a>
    where
        Self: 'a;

    /// Returns the slices of entities and components at the given `range`.
    #[must_use]
    unsafe fn get_data_unchecked<'a>(self, range: Range<usize>) -> (&'a [Entity], Self::Slice<'a>)
    where
        Self: 'a;
}

macro_rules! impl_comp_common {
    ($Comp:ident) => {
        unsafe impl<T> ComponentView for &'_ $Comp<'_, T>
        where
            T: Component,
        {
            type Ptr = *const T;

            type Ref<'a> = &'a T where Self: 'a;

            type Slice<'a> = &'a [T] where Self: 'a;

            fn get<'a>(self, entity: Entity) -> Option<Self::Ref<'a>>
            where
                Self: 'a,
            {
                $Comp::get(self, entity)
            }

            fn contains(self, entity: Entity) -> bool {
                $Comp::contains(self, entity)
            }

            fn len(&self) -> usize {
                $Comp::len(self)
            }

            fn group_info(&self) -> Option<GroupInfo> {
                $Comp::group_info(self)
            }

            fn split<'a>(self) -> (&'a [Entity], &'a SparseVec, Self::Ptr)
            where
                Self: 'a,
            {
                let (entities, sparse, components) = $Comp::split(self);
                (entities, sparse, components.as_ptr())
            }

            unsafe fn add_to_ptr(ptr: Self::Ptr, index: usize) -> Self::Ptr {
                ptr.add(index)
            }

            unsafe fn get_from_ptr<'a>(ptr: Self::Ptr, index: usize) -> Self::Ref<'a>
            where
                Self: 'a,
            {
                &*ptr.add(index)
            }

            unsafe fn get_entities_unchecked<'a>(self, range: Range<usize>) -> &'a [Entity]
            where
                Self: 'a,
            {
                $Comp::entities(self).get_unchecked(range)
            }

            unsafe fn get_components_unchecked<'a>(self, range: Range<usize>) -> Self::Slice<'a>
            where
                Self: 'a,
            {
                $Comp::as_slice(self).get_unchecked(range)
            }

            unsafe fn get_data_unchecked<'a>(
                self,
                range: Range<usize>,
            ) -> (&'a [Entity], Self::Slice<'a>)
            where
                Self: 'a,
            {
                let (entities, _, components) = $Comp::split(self);

                (
                    entities.get_unchecked(range.clone()),
                    components.get_unchecked(range),
                )
            }
        }
    };
}

impl_comp_common!(Comp);
impl_comp_common!(CompMut);

unsafe impl<T> ComponentView for &'_ mut CompMut<'_, T>
where
    T: Component,
{
    type Ptr = *mut T;

    type Ref<'a> = &'a mut T where Self: 'a;

    type Slice<'a> = &'a mut [T] where Self: 'a;

    fn get<'a>(self, entity: Entity) -> Option<Self::Ref<'a>>
    where
        Self: 'a,
    {
        CompMut::get_mut(self, entity)
    }

    fn contains(self, entity: Entity) -> bool {
        CompMut::contains(self, entity)
    }

    fn len(&self) -> usize {
        CompMut::len(self)
    }

    fn group_info(&self) -> Option<GroupInfo> {
        CompMut::group_info(self)
    }

    fn split<'a>(self) -> (&'a [Entity], &'a SparseVec, Self::Ptr)
    where
        Self: 'a,
    {
        let (entities, sparse, components) = CompMut::split_mut(self);
        (entities, sparse, components.as_mut_ptr())
    }

    unsafe fn add_to_ptr(ptr: Self::Ptr, index: usize) -> Self::Ptr {
        ptr.add(index)
    }

    unsafe fn get_from_ptr<'a>(ptr: Self::Ptr, index: usize) -> Self::Ref<'a>
    where
        Self: 'a,
    {
        &mut *ptr.add(index)
    }

    unsafe fn get_entities_unchecked<'a>(self, range: Range<usize>) -> &'a [Entity]
    where
        Self: 'a,
    {
        CompMut::entities(self).get_unchecked(range)
    }

    unsafe fn get_components_unchecked<'a>(self, range: Range<usize>) -> Self::Slice<'a>
    where
        Self: 'a,
    {
        CompMut::as_mut_slice(self).get_unchecked_mut(range)
    }

    unsafe fn get_data_unchecked<'a>(self, range: Range<usize>) -> (&'a [Entity], Self::Slice<'a>)
    where
        Self: 'a,
    {
        let (entities, _, components) = CompMut::split_mut(self);

        (
            entities.get_unchecked(range.clone()),
            components.get_unchecked_mut(range),
        )
    }
}
