use crate::query::iter::dense::*;
use crate::query::iter::sparse::*;
use crate::query::ComponentView;
use paste::paste;
use std::ptr;

macro_rules! impl_iter {
    ($len:tt, $($comp:ident),+) => {
        paste! {
            pub enum [<Iter $len>]<'a, $($comp),+>
            where
                $($comp: ComponentView<'a>,)+
            {
                Sparse([<SparseIter $len>]<'a, $($comp),+>),
                Dense([<DenseIter $len>]<'a, $($comp),+>),
            }

            impl<'a, $($comp),+> [<Iter $len>]<'a, $($comp),+>
            where
                $($comp: ComponentView<'a>,)+
            {
                pub fn new($([<comp_ $comp:lower>]: $comp),+) -> Self {
                    let is_grouped = is_grouped(&[
                        $([<comp_ $comp:lower>].group_len_ref()),+
                    ]);

                    if is_grouped {
                        unsafe {
                            Self::Dense([<DenseIter $len>]::new_unchecked($([<comp_ $comp:lower>]),+))
                        }
                    } else {
                        Self::Sparse([<SparseIter $len>]::new($([<comp_ $comp:lower>]),+))
                    }
                }
            }

            impl<'a, $($comp),+> Iterator for [<Iter $len>]<'a, $($comp),+>
            where
                $($comp: ComponentView<'a>,)+
            {
                type Item = ($($comp::Item,)+);

                fn next(&mut self) -> Option<Self::Item> {
                    match self {
                        Self::Sparse(iter) => iter.next(),
                        Self::Dense(iter) => iter.next(),
                    }
                }
            }
        }
    }
}

fn is_grouped(group_len_refs: &[Option<&usize>]) -> bool {
    (|| -> Option<()> {
        match group_len_refs.split_first() {
            Some((&first, others)) => {
                let first = first?;

                for &other in others {
                    let other = other?;

                    if !ptr::eq(first, other) {
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

impl_iter!(1, A);
impl_iter!(2, A, B);
impl_iter!(3, A, B, C);
impl_iter!(4, A, B, C, D);
