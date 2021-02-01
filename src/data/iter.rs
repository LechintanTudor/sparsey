pub use self::impls::*;

use crate::data::dense_iter::*;
use crate::data::sparse_iter::*;
use crate::data::IterableView;
use crate::registry::Group;
use paste::paste;

fn is_grouped(groups: &[Group]) -> bool {
    groups.windows(2).all(|w| w[0] == w[1])
}

macro_rules! impl_iter {
    (
        $iter:ident,
        $sparse_iter:ident,
        $dense_iter:ident,
        $($view:ident),+
    ) => {
        paste! {
            pub enum $iter<'a, $($view,)+>
            where
                $($view: IterableView<'a>,)+
            {
                Sparse($sparse_iter<'a, $($view,)+>),
                Dense($dense_iter<'a, $($view,)+>),
            }

            impl<'a, $($view,)+> $iter<'a, $($view,)+>
            where
                $($view: IterableView<'a>,)+
            {
                pub fn new($([<view_ $view:lower>]: $view,)+) -> Self {
                    let groups = (|| -> Option<_> {
                        unsafe { Some([$([<view_ $view:lower>].group()?,)+]) }
                    })();

                    let is_grouped = match groups {
                        Some(groups) => is_grouped(&groups),
                        None => false,
                    };

                    if is_grouped {
                        unsafe {
                            Self::Dense($dense_iter::new_unchecked($([<view_ $view:lower>],)+))
                        }
                    } else {
                        Self::Sparse($sparse_iter::new($([<view_ $view:lower>],)+))
                    }
                }
            }

            impl<'a, $($view,)+> Iterator for $iter<'a, $($view,)+>
            where
                $($view: IterableView<'a>,)+
            {
                type Item = ($($view::Output,)+);

                fn next(&mut self) -> Option<Self::Item> {
                    match self {
                        Self::Sparse(ref mut sparse) => sparse.next(),
                        Self::Dense(ref mut dense) => dense.next(),
                    }
                }
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_iter!(Iter1, SparseIter1, DenseIter1, A);
    impl_iter!(Iter2, SparseIter2, DenseIter2, A, B);
    impl_iter!(Iter3, SparseIter3, DenseIter3, A, B, C);
    impl_iter!(Iter4, SparseIter4, DenseIter4, A, B, C, D);
    impl_iter!(Iter5, SparseIter5, DenseIter5, A, B, C, D, E);
    impl_iter!(Iter6, SparseIter6, DenseIter6, A, B, C, D, E, F);
    impl_iter!(Iter7, SparseIter7, DenseIter7, A, B, C, D, E, F, G);
    impl_iter!(Iter8, SparseIter8, DenseIter8, A, B, C, D, E, F, G, H);
    impl_iter!(Iter9, SparseIter9, DenseIter9, A, B, C, D, E, F, G, H, I);
    impl_iter!(Iter10, SparseIter10, DenseIter10, A, B, C, D, E, F, G, H, I, J);
    impl_iter!(Iter11, SparseIter11, DenseIter11, A, B, C, D, E, F, G, H, I, J, K);
    impl_iter!(Iter12, SparseIter12, DenseIter12, A, B, C, D, E, F, G, H, I, J, K, L);
}
