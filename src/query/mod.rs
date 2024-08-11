mod group_info;
mod iter;
mod query_elem;
mod world_query;

pub use self::group_info::*;
pub use self::iter::*;
pub use self::query_elem::*;
pub use self::world_query::*;

use crate::entity::{Entity, World};

pub trait Query {
    type View<'a>;
    type Item<'a>;
    type Ptr;

    #[must_use]
    fn borrow(world: &World) -> Self::View<'_>;

    #[must_use]
    fn borrow_with_group_info(world: &World) -> (Self::View<'_>, Option<QueryGroupInfo>);

    #[must_use]
    fn entities<'a>(view: &'a Self::View<'_>) -> Option<&'a [Entity]>;

    #[must_use]
    fn contains_all(view: &Self::View<'_>, entity: Entity) -> bool;

    #[must_use]
    fn contains_none(view: &Self::View<'_>, entity: Entity) -> bool;

    #[must_use]
    fn get<'a>(view: &'a mut Self::View<'_>, entity: Entity) -> Option<Self::Item<'a>>;

    #[must_use]
    fn get_ptr(view: &Self::View<'_>, entity: Entity) -> Option<Self::Ptr>;

    #[must_use]
    unsafe fn get_ptr_unchecked(view: &Self::View<'_>, entity: Entity, index: usize) -> Self::Ptr;

    #[must_use]
    unsafe fn deref_ptr<'a>(ptr: Self::Ptr) -> Self::Item<'a>;
}

#[allow(unused_variables)]
impl Query for () {
    type View<'a> = ();
    type Item<'a> = ();
    type Ptr = ();

    #[inline]
    fn borrow(world: &World) -> Self::View<'_> {}

    #[inline]
    fn borrow_with_group_info(world: &World) -> (Self::View<'_>, Option<QueryGroupInfo>) {
        ((), Some(QueryGroupInfo::Empty))
    }

    #[inline]
    fn entities<'a>(view: &'a Self::View<'_>) -> Option<&'a [Entity]> {
        None
    }

    #[inline]
    fn contains_all(view: &Self::View<'_>, entity: Entity) -> bool {
        true
    }

    #[inline]
    fn contains_none(view: &Self::View<'_>, entity: Entity) -> bool {
        true
    }

    #[inline]
    fn get<'a>(view: &'a mut Self::View<'_>, entity: Entity) -> Option<Self::Item<'a>> {
        Some(())
    }

    #[inline]
    fn get_ptr(view: &Self::View<'_>, entity: Entity) -> Option<Self::Ptr> {
        Some(())
    }

    #[inline]
    unsafe fn get_ptr_unchecked(view: &Self::View<'_>, entity: Entity, index: usize) -> Self::Ptr {}

    #[inline]
    unsafe fn deref_ptr<'a>(ptr: Self::Ptr) -> Self::Item<'a> {}
}

macro_rules! impl_query {
    ($(($Ty:ident, $idx:tt)),+) => {
        impl<$($Ty),+> Query for ($($Ty,)+)
        where
            $($Ty: QueryElem,)+
        {
            type View<'a> = ($($Ty::View<'a>,)+);
            type Item<'a> = ($($Ty::Item<'a>,)+);
            type Ptr = ($($Ty::Ptr,)+);

            fn borrow(world: &World) -> Self::View<'_> {
                ($($Ty::borrow(world),)+)
            }

            fn borrow_with_group_info(world: &World) -> (Self::View<'_>, Option<QueryGroupInfo>) {
                let view_and_group_info = ($($Ty::borrow_with_group_info(world),)+);

                let get_group_info = || -> Option<QueryGroupInfo> {
                    let mut group_info = QueryGroupInfo::Empty;

                    $(
                        if let Some(info) = &view_and_group_info.$idx.1 {
                            group_info = group_info.add_view(info)?;
                        }
                    )+

                    Some(group_info)
                };

                (
                    ($(view_and_group_info.$idx.0,)+),
                    get_group_info(),
                )
            }

            #[allow(unused_assignments)]
            fn entities<'a>(view: &'a Self::View<'_>) -> Option<&'a [Entity]> {
                let mut entities = Option::<&[Entity]>::None;
                let mut min_len = usize::MAX;

                $(
                    if let Some(view_entities) = $Ty::entities(&view.$idx) {
                        if view_entities.len() < min_len {
                            entities = Some(view_entities);
                            min_len = view_entities.len();
                        }
                    }
                )+

                entities
            }

            fn contains_all(view: &Self::View<'_>, entity: Entity) -> bool {
                $($Ty::contains(&view.$idx, entity))&&+
            }

            fn contains_none(view: &Self::View<'_>, entity: Entity) -> bool {
                $(!$Ty::contains(&view.$idx, entity))&&+
            }

            fn get<'a>(view: &'a mut Self::View<'_>, entity: Entity) -> Option<Self::Item<'a>> {
                Some(($($Ty::get(&mut view.$idx, entity)?,)+))
            }

            fn get_ptr(view: &Self::View<'_>, entity: Entity) -> Option<Self::Ptr> {
                Some(($($Ty::get_ptr(&view.$idx, entity)?,)+))
            }

            unsafe fn get_ptr_unchecked(view: &Self::View<'_>, entity: Entity, index: usize) -> Self::Ptr {
                ($($Ty::get_ptr_unchecked(&view.$idx, entity, index),)+)
            }

            unsafe fn deref_ptr<'a>(ptr: Self::Ptr) -> Self::Item<'a> {
                ($($Ty::deref_ptr(ptr.$idx),)+)
            }
        }
    };
}

impl_query!((A, 0));
impl_query!((A, 0), (B, 1));
impl_query!((A, 0), (B, 1), (C, 2));
