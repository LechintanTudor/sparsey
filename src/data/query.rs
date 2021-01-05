use crate::data::iter::*;
use crate::data::IterableView;
use paste::paste;

pub trait Query<'a> {
    type Iterator: Iterator + 'a;

    fn join(self) -> Self::Iterator;
}

macro_rules! impl_query {
    ($iter:ty, $($view:ty),+) => {
        paste! {
            impl<'a, $($view,)+> Query<'a> for ($($view,)+)
            where
                $($view: IterableView<'a> + 'a,)+
            {
                type Iterator = $iter<'a, $($view,)+>;

                fn join(self) -> Self::Iterator {
                    let ($([<view_ $view:lower>],)+) = self;
                    Self::Iterator::new($([<view_ $view:lower>],)+)
                }
            }
        }
    };
}

#[rustfmt::skip] impl_query!(Iter1, A);
#[rustfmt::skip] impl_query!(Iter2, A, B);
#[rustfmt::skip] impl_query!(Iter3, A, B, C);
#[rustfmt::skip] impl_query!(Iter4, A, B, C, D);
#[rustfmt::skip] impl_query!(Iter5, A, B, C, D, E);
#[rustfmt::skip] impl_query!(Iter6, A, B, C, D, E, F);
#[rustfmt::skip] impl_query!(Iter7, A, B, C, D, E, F, G);
#[rustfmt::skip] impl_query!(Iter8, A, B, C, D, E, F, G, H);
#[rustfmt::skip] impl_query!(Iter9, A, B, C, D, E, F, G, H, I);
#[rustfmt::skip] impl_query!(Iter10, A, B, C, D, E, F, G, H, I, J);
#[rustfmt::skip] impl_query!(Iter11, A, B, C, D, E, F, G, H, I, J, K);
#[rustfmt::skip] impl_query!(Iter12, A, B, C, D, E, F, G, H, I, J, K, L);
