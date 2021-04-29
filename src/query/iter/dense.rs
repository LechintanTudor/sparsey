pub use self::impls::*;

use crate::components::{ComponentInfo, Entity, Ticks};
use crate::query::{QueryElement, StateFilter};
use paste::paste;
use std::ops::Range;

type SparseSplitQueryElement<'a, S, T> = (S, *mut ComponentInfo, *mut T);

macro_rules! impl_dense_iter {
	($ident:ident, $($elem:ident),+) => {
		paste! {
            pub struct $ident<'a, Filter, $($elem),+>
            where
                Filter: StateFilter,
                $($elem: QueryElement<'a>,)+
            {
                entities: &'a [Entity],
                filter: Filter,
                $([<elem_ $elem:lower>]: SparseSplitQueryElement<'a, $elem::State, $elem::Component>,)+
                world_tick: Ticks,
                last_system_tick: Ticks,
                index: usize,
            }

            impl<'a, Filter, $($elem),+> $ident<'a, Filter, $($elem),+>
            where
                Filter: StateFilter,
                $($elem: QueryElement<'a>,)+
            {
                pub unsafe fn new(
                    filter: Filter,
                    group_bounds: Range<usize>,
                    $([<elem_ $elem:lower>]: $elem,)+
                ) -> Self {
                    new_dense_iter!(filter, group_bounds, $([<elem_ $elem:lower>]),+)
                }
            }

            impl<'a, Filter, $($elem),+> Iterator for $ident<'a, Filter, $($elem),+>
            where
                Filter: StateFilter,
                $($elem: QueryElement<'a>,)+
            {
                type Item = ($($elem::Item,)+);

                fn next(&mut self) -> Option<Self::Item> {
                    loop {
                        let index = self.index;
                        let entity = *self.entities.get(index)?;

                        self.index += 1;

                        if self.filter.matches(entity) {
                            let item = (|| unsafe {
                                Some(($(
                                    $elem::get_from_split(
                                        self.[<elem_ $elem:lower>].2,
                                        self.[<elem_ $elem:lower>].1,
                                        index,
                                        self.[<elem_ $elem:lower>].0,
                                        self.world_tick,
                                        self.last_system_tick,
                                    )?,
                                )+))
                            })();

                            if item.is_some() {
                                return item;
                            }
                        }
                    }
                }
            }
        }
	};
}

macro_rules! new_dense_iter {
    (
        $filter:expr,
        $group_bounds:expr,
        $first:ident
        $(, $other:ident)*
    ) => {
        {
            let world_tick = $first.world_tick();
            let last_system_tick = $first.last_system_tick();

            let $first = $first.split();
            $(let $other = $other.split();)*

            Self {
                filter: $filter,
                entities: &$first.2[$group_bounds],
                $first: ($first.0, $first.3, $first.4),
                $($other: ($other.0, $other.3, $other.4),)*
                world_tick,
                last_system_tick,
                index: 0,
            }
        }
    };
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
