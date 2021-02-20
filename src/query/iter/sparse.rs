use crate::data::{Entity, SparseArray};
use crate::query::ComponentView;
use paste::paste;

macro_rules! impl_sparse_iter {
    ($ident:ident, $($comp:ident),+) => {
        paste! {
            pub struct $ident<'a, $($comp),+>
            where
                $($comp: ComponentView<'a>,)+
            {
                dense: &'a [Entity],
                index: usize,
                $([<comp_ $comp:lower>]: (&'a SparseArray, $comp::Flags, $comp::Data),)+
            }

            impl<'a, $($comp),+> $ident<'a, $($comp),+>
            where
                $($comp: ComponentView<'a>,)+
            {
                pub fn new($([<comp_ $comp:lower>]: $comp),+) -> Self {
                    $(let [<comp_ $comp:lower>] = [<comp_ $comp:lower>].split();)+

                    Self {
                        dense: shortest_dense!($([<comp_ $comp:lower>].1),+),
                        index: 0,
                        $([<comp_ $comp:lower>]: strip_dense::<$comp>([<comp_ $comp:lower>]),)+
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
                        let entity = *self.dense.get(self.index)?;
                        self.index += 1;

                        let item = (|| unsafe {
                            Some((
                                $(
                                    $comp::get_item(
                                        self.[<comp_ $comp:lower>].1,
                                        self.[<comp_ $comp:lower>].2,
                                        self.[<comp_ $comp:lower>].0.get_index_entity(entity)?.index(),
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

macro_rules! shortest_dense {
    ($first:expr) => {
        $first
    };
    ($first:expr, $($others:expr),+) => {
        shortest_dense($first, shortest_dense!($($others),+))
    };
}

fn shortest_dense<'a>(a: &'a [Entity], b: &'a [Entity]) -> &'a [Entity] {
    if a.len() <= b.len() {
        a
    } else {
        b
    }
}

#[inline]
fn strip_dense<'a, C>(
    view: (&'a SparseArray, &'a [Entity], C::Flags, C::Data),
) -> (&'a SparseArray, C::Flags, C::Data)
where
    C: ComponentView<'a>,
{
    (view.0, view.2, view.3)
}

impl_sparse_iter!(SparseIter1, A);
impl_sparse_iter!(SparseIter2, A, B);
impl_sparse_iter!(SparseIter3, A, B, C);
impl_sparse_iter!(SparseIter4, A, B, C, D);
