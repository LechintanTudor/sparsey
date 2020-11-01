use crate::{Entity, SparseArray};
use paste::paste;

fn shortest_dense<'a>(d1: Option<&'a [Entity]>, d2: Option<&'a [Entity]>) -> Option<&'a [Entity]> {
    match d1 {
        Some(d1) => match d2 {
            Some(d2) => {
                if d1.len() <= d2.len() {
                    Some(d1)
                } else {
                    Some(d2)
                }
            }
            None => Some(d1),
        },
        None => d2,
    }
}

macro_rules! find_shortest_dense_inner {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($y:expr),+) => {
        shortest_dense($x, find_shortest_dense_inner!($($y),+))
    };
}

macro_rules! find_shortest_dense {
    ($($x:expr),+) => {
        find_shortest_dense_inner!($(
            if $x.1 {
                Some($x.0.1)
            } else {
                None
            }
        ),+)
    };
}

macro_rules! impl_iter {
    ($ident:ident, $($comp:ident),+) => {
        paste! {
            pub struct $ident<'a, $($comp),+>
            where
                $($comp: $crate::iterator::View<'a>,)+
            {
                dense: &'a [Entity],
                index: usize,
                $([<set_ $comp:lower>]: (&'a SparseArray, <$comp::SparseSet as $crate::iterator::SparseSetLike<'a>>::Slice),)+
            }

            impl<'a, $($comp),+> $ident<'a, $($comp),+>
            where
                $($comp: $crate::iterator::View<'a>,)+
            {
                pub fn new($([<set_ $comp:lower>]: $comp::SparseSet),+) -> Self {
                    $(
                        let [<set_ $comp:lower>] = <$comp::SparseSet as $crate::iterator::SparseSetLike<'a>>::split([<set_ $comp:lower>]);
                    )+

                    let dense = find_shortest_dense!($((
                        [<set_ $comp:lower>],
                        $comp::STRICT,
                    )),+).expect("Iterators must have at least one strict view");

                    Self {
                        dense,
                        index: 0,
                        $(
                            [<set_ $comp:lower>]: ([<set_ $comp:lower>].0, [<set_ $comp:lower>].2),
                        )+
                    }
                }
            }

            impl<'a, $($comp),+> Iterator for $ident<'a, $($comp),+>
            where
                $($comp: $crate::iterator::View<'a>,)+
            {
                type Item = ($($comp::Output,)+);

                fn next(&mut self) -> Option<Self::Item> {
                    loop {
                        let entity = *self.dense.get(self.index)?;
                        self.index += 1;

                        let current_item = (|| unsafe {
                            Some((
                                $(
                                    $crate::fetch::<$comp>(self.[<set_ $comp:lower>].0, self.[<set_ $comp:lower>].1, entity)?,
                                )+
                            ))
                        })();

                        if current_item.is_some() {
                            return current_item;
                        }
                    }
                }
            }
        }
    };
}

impl_iter!(Iterator1, A);
impl_iter!(Iterator2, A, B);
impl_iter!(Iterator3, A, B, C);
impl_iter!(Iterator4, A, B, C, D);
impl_iter!(Iterator5, A, B, C, D, E);
impl_iter!(Iterator6, A, B, C, D, E, F);
impl_iter!(Iterator7, A, B, C, D, E, F, G);
impl_iter!(Iterator8, A, B, C, D, E, F, G, H);
