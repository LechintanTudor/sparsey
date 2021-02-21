use crate::data::Entity;
use crate::query::iter::*;
use crate::query::ComponentView;

pub unsafe trait Query<'a> {
    type Item: 'a;
    type Iterator: Iterator<Item = Self::Item> + 'a;

    fn get(self, entity: Entity) -> Option<Self::Item>;

    fn iter(self) -> Self::Iterator;
}

macro_rules! impl_query {
    ($iter:ident, $(($comp:ident, $idx:tt)),+) => {
        unsafe impl<'a, $($comp),+> Query<'a> for ($($comp,)+)
        where
            $($comp: ComponentView<'a> + 'a,)+
        {
            type Item = ($($comp::Item,)+);
            type Iterator = $iter<'a, $($comp),+>;

            fn get(self, entity: Entity) -> Option<Self::Item> {
                Some((
                    $(self.$idx.get(entity)?,)+
                ))
            }

            fn iter(self) -> Self::Iterator {
                $iter::new($(self.$idx),+)
            }
        }
    };
}

impl_query!(Iter1, (A, 0));
impl_query!(Iter2, (A, 0), (B, 1));
impl_query!(Iter3, (A, 0), (B, 1), (C, 2));
impl_query!(Iter4, (A, 0), (B, 1), (C, 2), (D, 3));
