pub use self::impls::*;

use crate::data::Entity;
use crate::query::IterableView;
use paste::paste;

macro_rules! first_of {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($y:expr),+) => {
        $x
    };
}

macro_rules! impl_dense_iter {
    ($ident:ident, $($comp:ty),+) => {
        paste! {
            pub struct $ident<'a, $($comp,)+>
            where
                $($comp: IterableView<'a>,)+
            {
                dense: &'a [Entity],
                current_index: usize,
                $([<set_ $comp:lower>]: ($comp::Data, $comp::Flags),)+
            }

            impl<'a, $($comp,)+> $ident<'a, $($comp,)+>
            where
                $($comp: IterableView<'a>,)+
            {
                #[allow(unused_variables)]
                pub unsafe fn new_unchecked($([<view_ $comp:lower>]: $comp),+) -> Self {
                    let subgroup_len = *first_of!($([<view_ $comp:lower>]),+)
                        .group_len()
                        .unwrap();

                    $(
                        let (
                            _,
                            [<dense_ $comp:lower>],
                            [<data_ $comp:lower>],
                            [<flags_ $comp:lower>],
                        ) = [<view_ $comp:lower>].split();
                    )+

                    let dense = &first_of!($([<dense_ $comp:lower>]),+)[..subgroup_len];

                    Self {
                        dense,
                        current_index: 0,
                        $([<set_ $comp:lower>]: ([<data_ $comp:lower>], [<flags_ $comp:lower>]),)+
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
                    if self.current_index >= self.dense.len() {
                        return None;
                    }

                    let index = self.current_index;
                    self.current_index += 1;

                    unsafe {
                        Some((
                            $(
                                <$comp>::get(
                                    self.[<set_ $comp:lower>].0,
                                    self.[<set_ $comp:lower>].1,
                                    index,
                                )?,
                            )+
                        ))
                    }
                }
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_dense_iter!(DenseIter1, A);
    impl_dense_iter!(DenseIter2, A, B);
    impl_dense_iter!(DenseIter3, A, B, C);
    impl_dense_iter!(DenseIter4, A, B, C, D);
    impl_dense_iter!(DenseIter5, A, B, C, D, E);
    impl_dense_iter!(DenseIter6, A, B, C, D, E, F);
    impl_dense_iter!(DenseIter7, A, B, C, D, E, F, G);
    impl_dense_iter!(DenseIter8, A, B, C, D, E, F, G, H);
    impl_dense_iter!(DenseIter9, A, B, C, D, E, F, G, H, I);
    impl_dense_iter!(DenseIter10, A, B, C, D, E, F, G, H, I, J);
    impl_dense_iter!(DenseIter11, A, B, C, D, E, F, G, H, I, J, K);
    impl_dense_iter!(DenseIter12, A, B, C, D, E, F, G, H, I, J, K, L);
}
