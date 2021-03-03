pub use self::impls::*;

use crate::data::Entity;
use crate::query::iter::*;
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

            impl<'a, $($comp),+> EntityIterator for [<Iter $len>]<'a, $($comp),+>
            where
                $($comp: ComponentView<'a>,)+
            {
                fn current_entity(&self) -> Option<Entity> {
                    match self {
                        Self::Sparse(iter) => iter.current_entity(),
                        Self::Dense(iter) => iter.current_entity(),
                    }
                }
            }
        }
    }
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_iter!(2,  A, B);
    impl_iter!(3,  A, B, C);
    impl_iter!(4,  A, B, C, D);
    impl_iter!(5,  A, B, C, D, E);
    impl_iter!(6,  A, B, C, D, E, F);
    impl_iter!(7,  A, B, C, D, E, F, G);
    impl_iter!(8,  A, B, C, D, E, F, G, H);
    impl_iter!(9,  A, B, C, D, E, F, G, H, I);
    impl_iter!(10, A, B, C, D, E, F, G, H, I, J);
    impl_iter!(11, A, B, C, D, E, F, G, H, I, J, K);
    impl_iter!(12, A, B, C, D, E, F, G, H, I, J, K, L);
    impl_iter!(13, A, B, C, D, E, F, G, H, I, J, K, L, M);
    impl_iter!(14, A, B, C, D, E, F, G, H, I, J, K, L, M, N);
    impl_iter!(15, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
    impl_iter!(16, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
}
