pub use self::impls::*;

use crate::data::{Entity, SparseVec};
use crate::query::{ComponentView, EntityIterator};
use paste::paste;

macro_rules! impl_dense_iter {
    ($ident:ident, $($comp:ident),+) => {
        paste! {
            pub struct $ident<'a, $($comp),+>
            where
                $($comp: ComponentView<'a>,)+
            {
                dense: &'a [Entity],
                index: usize,
                $([<comp_ $comp:lower>]: ($comp::Flags, $comp::Data),)+
            }

            impl<'a, $($comp),+> $ident<'a, $($comp),+>
            where
                $($comp: ComponentView<'a>,)+
            {
                pub unsafe fn new_unchecked(group_len: usize, $([<comp_ $comp:lower>]: $comp,)+) -> Self {
                    new_dense_iter!(group_len, $(([<comp_ $comp:lower>], $comp))+)
                }
            }

            impl<'a, $($comp),+> Iterator for $ident<'a, $($comp),+>
            where
                $($comp: ComponentView<'a>,)+
            {
                type Item = ($($comp::Item,)+);

                fn next(&mut self) -> Option<Self::Item> {
                    loop {
                        if self.index >= self.dense.len() {
                            return None;
                        }

                        let index = self.index;
                        self.index += 1;

                        let item = (|| unsafe {
                            Some((
                                $(
                                    $comp::get_item(
                                        self.[<comp_ $comp:lower>].0,
                                        self.[<comp_ $comp:lower>].1,
                                        index,
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

            impl<'a, $($comp),+> EntityIterator for $ident<'a, $($comp),+>
            where
                $($comp: ComponentView<'a>,)+
            {
                fn current_entity(&self) -> Option<Entity> {
                    Some(*self.dense.get(self.index)?)
                }
            }
        }
    };
}

macro_rules! new_dense_iter {
    ($group_len:tt, ($first:ident, $first_comp:ident) $(($other:ident, $other_comp:ident))*) => {
        {
            let (_, dense, flags, data) = $first.split();

            Self {
                dense: &dense[..$group_len],
                index: 0,
                $first: (flags, data),
                $(
                    $other: strip_view::<$other_comp>($other.split()),
                )*
            }
        }
    };
}

#[inline]
fn strip_view<'a, V>(view: (&'a SparseVec, &'a [Entity], V::Flags, V::Data)) -> (V::Flags, V::Data)
where
    V: ComponentView<'a>,
{
    (view.2, view.3)
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_dense_iter!(DenseIter2,  A, B);
    impl_dense_iter!(DenseIter3,  A, B, C);
    impl_dense_iter!(DenseIter4,  A, B, C, D);
    impl_dense_iter!(DenseIter5,  A, B, C, D, E);
    impl_dense_iter!(DenseIter6,  A, B, C, D, E, F);
    impl_dense_iter!(DenseIter7,  A, B, C, D, E, F, G);
    impl_dense_iter!(DenseIter8,  A, B, C, D, E, F, G, H);
    impl_dense_iter!(DenseIter9,  A, B, C, D, E, F, G, H, I);
    impl_dense_iter!(DenseIter10, A, B, C, D, E, F, G, H, I, J);
    impl_dense_iter!(DenseIter11, A, B, C, D, E, F, G, H, I, J, K);
    impl_dense_iter!(DenseIter12, A, B, C, D, E, F, G, H, I, J, K, L);
    impl_dense_iter!(DenseIter13, A, B, C, D, E, F, G, H, I, J, K, L, M);
    impl_dense_iter!(DenseIter14, A, B, C, D, E, F, G, H, I, J, K, L, M, N);
    impl_dense_iter!(DenseIter15, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
    impl_dense_iter!(DenseIter16, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
}
