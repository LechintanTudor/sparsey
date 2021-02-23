use crate::data::Entity;
use crate::query::ComponentView;
use paste::paste;

macro_rules! impl_dense_iter {
    ($ident:ident, $($comp:ident),+) => {
        paste! {
            pub struct $ident<'a, $($comp),+>
            where
                $($comp: ComponentView<'a>,)+
            {
                dense: &'a [Entity],
                index: usize,
                $([<comp_ $comp:lower>]: ($comp::Flags, $comp::Data),)+
            }

            impl<'a, $($comp),+> $ident<'a, $($comp),+>
            where
                $($comp: ComponentView<'a>,)+
            {
                pub unsafe fn new_unchecked(subgroup_len: usize, $([<comp_ $comp:lower>]: $comp,)+) -> Self {
                    $(let [<comp_ $comp:lower>] = [<comp_ $comp:lower>].split();)+

                    Self {
                        dense: &first_of!($([<comp_ $comp:lower>].1),+)[..subgroup_len],
                        index: 0,
                        $([<comp_ $comp:lower>]: ([<comp_ $comp:lower>].2, [<comp_ $comp:lower>].3),)+
                    }
                }
            }

            impl<'a, $($comp),+> Iterator for $ident<'a, $($comp),+>
            where
                $($comp: ComponentView<'a>,)+
            {
                type Item = ($($comp::Item,)+);

                fn next(&mut self) -> Option<Self::Item> {
                    loop {
                        if self.index >= self.dense.len() {
                            return None;
                        }

                        let index = self.index;
                        self.index += 1;

                        let item = (|| unsafe {
                            Some((
                                $(
                                    $comp::get_item(
                                        self.[<comp_ $comp:lower>].0,
                                        self.[<comp_ $comp:lower>].1,
                                        index,
                                    )?,
                                )+
                            ))
                        })();

                        if item.is_some() {
                            return item;
                        }
                    }
                }
            }
        }
    };
}

macro_rules! first_of {
    ($first:expr) => {
        $first
    };
    ($first:expr, $($others:expr),+) => {
        $first
    };
}

impl_dense_iter!(DenseIter2, A, B);
impl_dense_iter!(DenseIter3, A, B, C);
impl_dense_iter!(DenseIter4, A, B, C, D);
