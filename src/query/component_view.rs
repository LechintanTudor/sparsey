use crate::entity::{Comp, CompMut, Component, Entity, GroupInfo, SparseVec};

#[allow(clippy::len_without_is_empty)]
pub unsafe trait ComponentView {
    type Ptr: Copy;

    type Ref<'a>
    where
        Self: 'a;

    #[must_use]
    fn get<'a>(self, entity: Entity) -> Option<Self::Ref<'a>>
    where
        Self: 'a;

    #[must_use]
    fn contains(self, entity: Entity) -> bool;

    #[must_use]
    fn len(&self) -> usize;

    #[must_use]
    fn group_info(&self) -> Option<GroupInfo>;

    #[must_use]
    fn split<'a>(self) -> (&'a [Entity], &'a SparseVec, Self::Ptr)
    where
        Self: 'a;

    #[must_use]
    unsafe fn add_to_ptr(ptr: Self::Ptr, index: usize) -> Self::Ptr;

    #[must_use]
    unsafe fn get_from_ptr<'a>(ptr: Self::Ptr, index: usize) -> Self::Ref<'a>
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
}
