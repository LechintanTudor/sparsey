use super::{dense_iter::*, iterable_view::*, sparse_iter::*};
use paste::paste;

fn is_grouped(groups: &[ParentGroup]) -> bool {
    let (first, other) = match groups.split_first() {
        Some(result) => result,
        None => return false,
    };

    for group in other.iter() {
        if group != first {
            return false;
        }
    }

    true
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
                        Some([$([<view_ $view:lower>].parent_group()?,)+])
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

#[rustfmt::skip] impl_iter!(Iter1, SparseIter1, DenseIter1, A);
#[rustfmt::skip] impl_iter!(Iter2, SparseIter2, DenseIter2, A, B);
#[rustfmt::skip] impl_iter!(Iter3, SparseIter3, DenseIter3, A, B, C);
#[rustfmt::skip] impl_iter!(Iter4, SparseIter4, DenseIter4, A, B, C, D);
#[rustfmt::skip] impl_iter!(Iter5, SparseIter5, DenseIter5, A, B, C, D, E);
#[rustfmt::skip] impl_iter!(Iter6, SparseIter6, DenseIter6, A, B, C, D, E, F);
#[rustfmt::skip] impl_iter!(Iter7, SparseIter7, DenseIter7, A, B, C, D, E, F, G);
#[rustfmt::skip] impl_iter!(Iter8, SparseIter8, DenseIter8, A, B, C, D, E, F, G, H);
#[rustfmt::skip] impl_iter!(Iter9, SparseIter9, DenseIter9, A, B, C, D, E, F, G, H, I);
#[rustfmt::skip] impl_iter!(Iter10, SparseIter10, DenseIter10, A, B, C, D, E, F, G, H, I, J);
#[rustfmt::skip] impl_iter!(Iter11, SparseIter11, DenseIter11, A, B, C, D, E, F, G, H, I, J, K);
#[rustfmt::skip] impl_iter!(Iter12, SparseIter12, DenseIter12, A, B, C, D, E, F, G, H, I, J, K, L);
