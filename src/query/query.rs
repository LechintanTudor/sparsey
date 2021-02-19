use crate::data::Entity;
use crate::query::ComponentView;

pub unsafe trait Query<'a> {
    type Item: 'a;

    fn get(self, entity: Entity) -> Option<Self::Item>;
}

macro_rules! impl_query {
    ($(($comp:ident, $idx:tt)),+) => {
        unsafe impl<'a, $($comp),+> Query<'a> for ($($comp,)+)
        where
            $($comp: ComponentView<'a> + 'a,)+
        {
            type Item = ($($comp::Item,)+);

            fn get(self, entity: Entity) -> Option<Self::Item> {
                Some((
                    $(self.$idx.get(entity)?,)+
                ))
            }
        }
    };
}

impl_query!((A, 0));
impl_query!((A, 0), (B, 1));
impl_query!((A, 0), (B, 1), (C, 2));
impl_query!((A, 0), (B, 1), (C, 2), (D, 3));
