pub use self::dense::*;
pub use self::sparse::*;

use crate::query::{ComponentFilter, QueryElement, StateFilter};
use paste::paste;

mod dense;
mod sparse;

macro_rules! impl_iter {
    ($n:tt, $($elem:ident),+) => {
        paste! {
            pub enum [<Iter $n>]<'a, Include, Exclude, Filter, $($elem),+>
            where
                Include: ComponentFilter,
                Exclude: ComponentFilter,
                Filter: StateFilter,
                $($elem: QueryElement<'a>,)+
            {
                Sparse([<SparseIter $n>]<'a, Include, Exclude, Filter, $($elem),+>),
                Dense([<DenseIter $n>]<'a, Filter, $($elem),+>),
            }

            impl<'a, Include, Exclude, Filter, $($elem),+> Iterator for [<Iter $n>]<'a, Include, Exclude, Filter, $($elem),+>
            where
                Include: ComponentFilter,
                Exclude: ComponentFilter,
                Filter: StateFilter,
                $($elem: QueryElement<'a>,)+
            {
                type Item = ($($elem::Item,)+);

                fn next(&mut self) -> Option<Self::Item> {
                    match self {
                        Self::Sparse(iter) => iter.next(),
                        Self::Dense(iter) => iter.next(),
                    }
                }
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_iter!(2, A, B);
}
