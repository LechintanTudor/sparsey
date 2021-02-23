use crate::query::iter::dense::*;
use crate::query::iter::sparse::*;
use crate::query::ComponentView;
use crate::world::get_subgroup_len;
use paste::paste;

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
                    let subgroup_len = (|| -> Option<usize> {
                        get_subgroup_len(&[
                            $([<comp_ $comp:lower>].subgroup_info()?),+
                        ])
                    })();

                    if let Some(subgroup_len) = subgroup_len {
                        unsafe {
                            Self::Dense([<DenseIter $len>]::new_unchecked(
                                subgroup_len,
                                $([<comp_ $comp:lower>]),+
                            ))
                        }
                    } else {
                        Self::Sparse([<SparseIter $len>]::new($([<comp_ $comp:lower>]),+))
                    }
                }

                pub fn is_grouped(&self) -> bool {
                    matches!(self, Self::Dense(_))
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

impl_iter!(2, A, B);
impl_iter!(3, A, B, C);
impl_iter!(4, A, B, C, D);
