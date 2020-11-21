use crate::{
    data::{iterator::*, view::*},
    entity::Entity,
};

pub trait Query<'a> {
    type Item: 'a;
    type Iterator: Iterator<Item = Self::Item>;

    fn get(self, entity: Entity) -> Option<Self::Item>;

    fn iter(self) -> Self::Iterator;
}

macro_rules! impl_query {
    ($iter:ident, $(($ty:ident, $idx:tt)),+) => {
        impl<'a, $($ty,)+> Query<'a> for ($($ty,)+)
        where
            $($ty: StorageView<'a>,)+
        {
            type Item = ($($ty::Output,)+);
            type Iterator = $iter<'a, $($ty,)+>;

            fn get(self, entity: Entity) -> Option<Self::Item> {
                unsafe {
                    Some((
                        $(self.$idx.get_output(entity)?,)+
                    ))
                }
            }

            fn iter(self) -> Self::Iterator {
                Self::Iterator::new(
                    $(self.$idx,)+
                )
            }
        }
    };
}

impl_query!(SparseIter1, (A, 0));
impl_query!(SparseIter2, (A, 0), (B, 1));
impl_query!(SparseIter3, (A, 0), (B, 1), (C, 2));
impl_query!(SparseIter4, (A, 0), (B, 1), (C, 2), (D, 3));
impl_query!(SparseIter5, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
impl_query!(SparseIter6, (A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
impl_query!(
    SparseIter7,
    (A, 0),
    (B, 1),
    (C, 2),
    (D, 3),
    (E, 4),
    (F, 5),
    (G, 6)
);
impl_query!(
    SparseIter8,
    (A, 0),
    (B, 1),
    (C, 2),
    (D, 3),
    (E, 4),
    (F, 5),
    (G, 6),
    (H, 7)
);
