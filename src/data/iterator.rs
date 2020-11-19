use crate::{data::view::*, entity::Entity, storage::SparseArray};
use paste::paste;

fn split<'a, V>(view: V) -> (&'a SparseArray, Option<&'a [Entity]>, V::Data)
where
    V: StorageView<'a>,
{
    let (sparse, dense, data) = unsafe { view.split() };

    if V::STRICT {
        (sparse, Some(dense), data)
    } else {
        (sparse, None, data)
    }
}

fn shortest_dense<'a>(d1: Option<&'a [Entity]>, d2: Option<&'a [Entity]>) -> Option<&'a [Entity]> {
    match (d1, d2) {
        (Some(a), Some(b)) => {
            if a.len() <= b.len() {
                d1
            } else {
                d2
            }
        }
        (Some(_), None) => d1,
        (None, Some(_)) => d2,
        (None, None) => None,
    }
}

fn strip_dense<'a, V>(
    split_view: (&'a SparseArray, Option<&'a [Entity]>, V::Data),
) -> (&'a SparseArray, V::Data)
where
    V: StorageView<'a>,
{
    (split_view.0, split_view.2)
}

unsafe fn get_output<'a, V>(
    (sparse, data): (&'a SparseArray, V::Data),
    entity: Entity,
) -> Option<V::Output>
where
    V: StorageView<'a>,
{
    V::get_from_component(sparse.get_valid(entity).map(|&e| V::get_component(data, e)))
}

macro_rules! find_shortest_dense {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($y:expr),+) => {
        shortest_dense($x, find_shortest_dense!($($y),+))
    };
}

macro_rules! impl_iterator {
    ($ident:ident, $($comp:ty),+) => {
        paste! {
            pub struct $ident<'a, $($comp,)+>
            where
                $($comp: StorageView<'a>,)+
            {
                dense: &'a [Entity],
                index: usize,
                $([<set_ $comp:lower>]: (&'a SparseArray, $comp::Data),)+
            }

            impl<'a, $($comp,)+> $ident<'a, $($comp,)+>
            where
                $($comp: StorageView<'a>,)+
            {
                pub fn new($([<set_ $comp:lower>]: $comp,)+) -> Self {
                    $(let [<set_ $comp:lower>] = split([<set_ $comp:lower>]);)+
                    let dense = find_shortest_dense!($([<set_ $comp:lower>].1),+).expect("At least one view must be strict");
                    $(let [<set_ $comp:lower>] = strip_dense::<$comp>([<set_ $comp:lower>]);)+

                    Self {
                        dense,
                        index: 0,
                        $([<set_ $comp:lower>],)+
                    }
                }
            }

            impl<'a, $($comp,)+> Iterator for $ident<'a, $($comp,)+>
            where
                $($comp: StorageView<'a>,)+
            {
                type Item = ($($comp::Output,)+);

                fn next(&mut self) -> Option<Self::Item> {
                    loop {
                        let entity = *self.dense.get(self.index)?;
                        self.index += 1;

                        let item = (|| unsafe {
                            Some((
                                $(get_output::<$comp>(self.[<set_ $comp:lower>], entity)?,)+
                            ))
                        })();

                        if item.is_some() {
                            return item;
                        }
                    }
                }
            }
        }
    };
}

impl_iterator!(SparseIter1, A);
impl_iterator!(SparseIter2, A, B);
impl_iterator!(SparseIter3, A, B, C);
impl_iterator!(SparseIter4, A, B, C, D);
impl_iterator!(SparseIter5, A, B, C, D, E);
impl_iterator!(SparseIter6, A, B, C, D, E, F);
impl_iterator!(SparseIter7, A, B, C, D, E, F, G);
impl_iterator!(SparseIter8, A, B, C, D, E, F, G, H);
