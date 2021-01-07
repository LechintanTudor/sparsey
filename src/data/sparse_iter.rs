pub use self::impls::*;

use crate::data::{self, IterableView};
use crate::storage::{Entity, SparseArray};
use paste::paste;

unsafe fn split<'a, V>(view: V) -> (&'a SparseArray, &'a [Entity], V::Data, V::Flags)
where
    V: IterableView<'a>,
{
    view.split()
}

fn shortest_dense<'a>(a: &'a [Entity], b: &'a [Entity]) -> &'a [Entity] {
    if a.len() <= b.len() {
        a
    } else {
        b
    }
}

fn strip_dense<'a, V>(
    split_view: (&'a SparseArray, &'a [Entity], V::Data, V::Flags),
) -> (&'a SparseArray, V::Data, V::Flags)
where
    V: IterableView<'a>,
{
    (split_view.0, split_view.2, split_view.3)
}

macro_rules! shortest_dense {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($y:expr),+) => {
        shortest_dense($x, shortest_dense!($($y),+))
    };
}

macro_rules! impl_sparse_iter {
    ($ident:ident, $($comp:ty),+) => {
        paste! {
            pub struct $ident<'a, $($comp,)+>
            where
                $($comp: IterableView<'a>,)+
            {
                dense: &'a [Entity],
                current_index: usize,
                $([<set_ $comp:lower>]: (&'a SparseArray, $comp::Data, $comp::Flags),)+
            }

            impl<'a, $($comp,)+> $ident<'a, $($comp,)+>
            where
                $($comp: IterableView<'a>,)+
            {
                pub fn new($([<set_ $comp:lower>]: $comp,)+) -> Self {
                    $(let [<set_ $comp:lower>] = unsafe { split([<set_ $comp:lower>]) };)+
                    let dense = shortest_dense!($([<set_ $comp:lower>].1),+);
                    $(let [<set_ $comp:lower>] = strip_dense::<$comp>([<set_ $comp:lower>]);)+

                    Self {
                        dense,
                        current_index: 0,
                        $([<set_ $comp:lower>],)+
                    }
                }

                pub fn current_entity(&self) -> Option<&Entity> {
                    self.dense.get(self.current_index)
                }
            }

            impl<'a, $($comp,)+> Iterator for $ident<'a, $($comp,)+>
            where
                $($comp: IterableView<'a>,)+
            {
                type Item = ($($comp::Output,)+);

                fn next(&mut self) -> Option<Self::Item> {
                    loop {
                        let entity = *self.current_entity()?;
                        self.current_index += 1;

                        let item = (|| unsafe {
                            Some((
                                $(
                                    data::get_output::<$comp>(
                                        self.[<set_ $comp:lower>].1,
                                        self.[<set_ $comp:lower>].2,
                                        self.[<set_ $comp:lower>].0.get(entity)?.index(),
                                    )?,
                                )+
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

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_sparse_iter!(SparseIter1, A);
    impl_sparse_iter!(SparseIter2, A, B);
    impl_sparse_iter!(SparseIter3, A, B, C);
    impl_sparse_iter!(SparseIter4, A, B, C, D);
    impl_sparse_iter!(SparseIter5, A, B, C, D, E);
    impl_sparse_iter!(SparseIter6, A, B, C, D, E, F);
    impl_sparse_iter!(SparseIter7, A, B, C, D, E, F, G);
    impl_sparse_iter!(SparseIter8, A, B, C, D, E, F, G, H);
    impl_sparse_iter!(SparseIter9, A, B, C, D, E, F, G, H, I);
    impl_sparse_iter!(SparseIter10, A, B, C, D, E, F, G, H, I, J);
    impl_sparse_iter!(SparseIter11, A, B, C, D, E, F, G, H, I, J, K);
    impl_sparse_iter!(SparseIter12, A, B, C, D, E, F, G, H, I, J, K, L);
}
