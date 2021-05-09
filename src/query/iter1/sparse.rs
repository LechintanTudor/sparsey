pub use self::impls::*;

use crate::components::{ComponentInfo, Entity, SparseArray, Ticks};
use crate::query::{ComponentFilter, QueryElement, StateFilter};
use paste::paste;

type SparseSplitQueryElement<'a, S, T> = (S, &'a SparseArray, *mut ComponentInfo, *mut T);

macro_rules! impl_sparse_iter {
	($ident:ident, $($elem:ident),+) => {
		paste! {
            pub struct $ident<'a, Include, Exclude, Filter, $($elem),+>
            where
                Include: ComponentFilter,
                Exclude: ComponentFilter,
                Filter: StateFilter,
                $($elem: QueryElement<'a>,)+
            {
                entities: &'a [Entity],
                include: Include,
                exclude: Exclude,
                filter: Filter,
                $([<elem_ $elem:lower>]: SparseSplitQueryElement<'a, $elem::State, $elem::Component>,)+
                world_tick: Ticks,
                last_system_tick: Ticks,
                index: usize,
            }

            impl<'a, Include, Exclude, Filter, $($elem),+> $ident<'a, Include, Exclude, Filter, $($elem),+>
            where
                Include: ComponentFilter,
                Exclude: ComponentFilter,
                Filter: StateFilter,
                $($elem: QueryElement<'a>,)+
            {
                pub fn new(
                    include: Include,
                    exclude: Exclude,
                    filter: Filter,
                    $([<elem_ $elem:lower>]: $elem,)+
                ) -> Self {
                    new_sparse_iter!(include, exclude, filter, $([<elem_ $elem:lower>]),+)
                }
            }

            impl<'a, Include, Exclude, Filter, $($elem),+> Iterator for $ident<'a, Include, Exclude, Filter, $($elem),+>
            where
                Include: ComponentFilter,
                Exclude: ComponentFilter,
                Filter: StateFilter,
                $($elem: QueryElement<'a>,)+
            {
                type Item = ($($elem::Item,)+);

                fn next(&mut self) -> Option<Self::Item> {
                    loop {
                        let entity = *self.entities.get(self.index)?;
                        self.index += 1;

                        if self.include.includes_all(entity)
                            && self.exclude.excludes_all(entity)
                            && self.filter.matches(entity)
                        {
                            let item = (|| unsafe {
                                Some(($(
                                    $elem::get_from_split(
                                        self.[<elem_ $elem:lower>].3,
                                        self.[<elem_ $elem:lower>].2,
                                        self.[<elem_ $elem:lower>].1.get_index(entity)? as usize,
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

macro_rules! new_sparse_iter {
    (
        $include:expr,
        $exclude:expr,
        $filter:expr,
        $first:ident
        $(, $other:ident)*
    ) => {
        {
            let world_tick = $first.world_tick();
            let last_system_tick = $first.last_system_tick();

            let $first = $first.split();
            $(let $other = $other.split();)*

            Self {
                include: $include,
                exclude: $exclude,
                filter: $filter,
                entities: shortest_entity_slice!($first.2 $(, $other.2)+),
                $first: ($first.0, $first.1, $first.3, $first.4),
                $($other: ($other.0, $other.1, $other.3, $other.4),)*
                world_tick,
                last_system_tick,
                index: 0,
            }
        }
    };
}

macro_rules! shortest_entity_slice {
    ($first:expr) => {
        $first
    };
    ($first:expr, $($other:expr),+) => {
        shortest_entity_slice($first, shortest_entity_slice!($($other),+))
    };
}

fn shortest_entity_slice<'a>(e1: &'a [Entity], e2: &'a [Entity]) -> &'a [Entity] {
	if e1.len() <= e2.len() {
		e1
	} else {
		e2
	}
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_sparse_iter!(SparseIter2,  A, B);
    impl_sparse_iter!(SparseIter3,  A, B, C);
    impl_sparse_iter!(SparseIter4,  A, B, C, D);
    impl_sparse_iter!(SparseIter5,  A, B, C, D, E);
    impl_sparse_iter!(SparseIter6,  A, B, C, D, E, F);
    impl_sparse_iter!(SparseIter7,  A, B, C, D, E, F, G);
    impl_sparse_iter!(SparseIter8,  A, B, C, D, E, F, G, H);
    impl_sparse_iter!(SparseIter9,  A, B, C, D, E, F, G, H, I);
    impl_sparse_iter!(SparseIter10, A, B, C, D, E, F, G, H, I, J);
    impl_sparse_iter!(SparseIter11, A, B, C, D, E, F, G, H, I, J, K);
    impl_sparse_iter!(SparseIter12, A, B, C, D, E, F, G, H, I, J, K, L);
    impl_sparse_iter!(SparseIter13, A, B, C, D, E, F, G, H, I, J, K, L, M);
    impl_sparse_iter!(SparseIter14, A, B, C, D, E, F, G, H, I, J, K, L, M, N);
    impl_sparse_iter!(SparseIter15, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
    impl_sparse_iter!(SparseIter16, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
}
