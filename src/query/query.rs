use crate::data::Entity;
use crate::query::iter::*;
use crate::query::ComponentView;
use crate::world::SubgroupInfo;

pub unsafe trait Query<'a> {
    type Item: 'a;
    type Iterator: Iterator<Item = Self::Item> + 'a;

    fn get(self, entity: Entity) -> Option<Self::Item>;

    fn iter(self) -> Self::Iterator;

    fn is_grouped(&self) -> bool;
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

            fn is_grouped(&self) -> bool {
                is_grouped(&[
                    $(self.$idx.subgroup_info()),+
                ])
            }
        }
    };
}

fn is_grouped(group_len_refs: &[Option<SubgroupInfo>]) -> bool {
    (|| -> Option<()> {
        match group_len_refs.split_first() {
            Some((&first, others)) => {
                let first = first?;

                for &other in others {
                    let other = other?;

                    if !first.has_same_group(&other) {
                        return None;
                    }
                }

                Some(())
            }
            None => Some(()),
        }
    })()
    .is_some()
}

impl_query!(Iter2, (A, 0), (B, 1));
impl_query!(Iter3, (A, 0), (B, 1), (C, 2));
impl_query!(Iter4, (A, 0), (B, 1), (C, 2), (D, 3));
