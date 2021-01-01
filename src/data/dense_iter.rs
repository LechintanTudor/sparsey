use crate::{data::IterableView, entity::Entity};
use paste::paste;

unsafe fn get<'a, V>((data, flags): (V::Data, V::Flags), index: usize) -> Option<V::Output>
where
    V: IterableView<'a>,
{
    V::get(data, flags, index)
}

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
                    let subgroup_len = first_of!($([<view_ $comp:lower>]),+)
                        .parent_group()
                        .unwrap()
                        .subgroup_len();

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
                            $(get::<$comp>(self.[<set_ $comp:lower>], index)?,)+
                        ))
                    }
                }
            }
        }
    };
}

#[rustfmt::skip] impl_dense_iter!(DenseIter1, A);
#[rustfmt::skip] impl_dense_iter!(DenseIter2, A, B);
#[rustfmt::skip] impl_dense_iter!(DenseIter3, A, B, C);
#[rustfmt::skip] impl_dense_iter!(DenseIter4, A, B, C, D);
#[rustfmt::skip] impl_dense_iter!(DenseIter5, A, B, C, D, E);
#[rustfmt::skip] impl_dense_iter!(DenseIter6, A, B, C, D, E, F);
#[rustfmt::skip] impl_dense_iter!(DenseIter7, A, B, C, D, E, F, G);
#[rustfmt::skip] impl_dense_iter!(DenseIter8, A, B, C, D, E, F, G, H);
#[rustfmt::skip] impl_dense_iter!(DenseIter9, A, B, C, D, E, F, G, H, I);
#[rustfmt::skip] impl_dense_iter!(DenseIter10, A, B, C, D, E, F, G, H, I, J);
#[rustfmt::skip] impl_dense_iter!(DenseIter11, A, B, C, D, E, F, G, H, I, J, K);
#[rustfmt::skip] impl_dense_iter!(DenseIter12, A, B, C, D, E, F, G, H, I, J, K, L);
