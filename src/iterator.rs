use crate::{Entity, SparseArray, SparseSet};
use paste::paste;

fn shortest_dense<'a>(d1: Option<&'a [Entity]>, d2: Option<&'a [Entity]>) -> Option<&'a [Entity]> {
    match d1 {
        Some(d1) => match d2 {
            Some(d2) => if d1.len() < d2.len() {
                Some(d1)
            } else {
                Some(d2)
            }
            None => Some(d1)
        } 
        None => d2
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

pub trait SparseSetLike<'a> {
    type Ref: 'a;
    type Slice: 'a + Copy;

    fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Slice);

    unsafe fn fetch(values: Self::Slice, entity: Entity) -> Self::Ref;
}

impl<'a, T> SparseSetLike<'a> for &'a SparseSet<T> {
    type Ref = &'a T;
    type Slice = *const T;

    fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Slice) {
        let (sparse, dense, data) = self.split();
        (sparse, dense, data.as_ptr())
    }

    unsafe fn fetch(values: Self::Slice, entity: Entity) -> Self::Ref {
        &*values.add(entity.index())
    }
}

impl<'a, T> SparseSetLike<'a> for &'a mut SparseSet<T> {
    type Ref = &'a mut T;
    type Slice = *mut T;

    fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Slice) {
        let (sparse, dense, data) = self.split_mut();
        (sparse, dense, data.as_mut_ptr())
    }

    unsafe fn fetch(values: Self::Slice, entity: Entity) -> Self::Ref {
        &mut *values.add(entity.index())
    }
}

pub trait IterView<'a> {
    const STRICT: bool;
    type SparseSet: SparseSetLike<'a>;
    type Output: 'a;

    fn fetch(value: Option<<Self::SparseSet as SparseSetLike<'a>>::Ref>) -> Option<Self::Output>;
}

impl<'a, T> IterView<'a> for &'a T {
    const STRICT: bool = true;
    type SparseSet = &'a SparseSet<T>;
    type Output = Self;

    fn fetch(value: Option<<Self::SparseSet as SparseSetLike<'a>>::Ref>) -> Option<Self::Output> {
        value
    }
}

impl<'a, T> IterView<'a> for &'a mut T {
    const STRICT: bool = true;
    type SparseSet = &'a mut SparseSet<T>;
    type Output = Self;

    fn fetch(value: Option<<Self::SparseSet as SparseSetLike<'a>>::Ref>) -> Option<Self::Output> {
        value
    }
}

impl<'a, T> IterView<'a> for Option<&'a T> {
    const STRICT: bool = false;
    type SparseSet = &'a SparseSet<T>;
    type Output = Self;

    fn fetch(value: Option<<Self::SparseSet as SparseSetLike<'a>>::Ref>) -> Option<Self::Output> {
        Some(value)
    }
}

impl<'a, T> IterView<'a> for Option<&'a mut T> {
    const STRICT: bool = false;
    type SparseSet = &'a mut SparseSet<T>;
    type Output = Self;

    fn fetch(value: Option<<Self::SparseSet as SparseSetLike<'a>>::Ref>) -> Option<Self::Output> {
        Some(value)
    }
}

pub unsafe fn fetch<'a, T>(
    sparse: &SparseArray,
    values: <T::SparseSet as SparseSetLike<'a>>::Slice,
    entity: Entity,
) -> Option<T::Output>
where
    T: IterView<'a>,
{
    T::fetch(
        sparse
            .get_valid(entity)
            .map(|&e| <T::SparseSet as SparseSetLike<'a>>::fetch(values, e)),
    )
}

#[allow(unused_macros)]
macro_rules! impl_iter {
    ($ident:ident, $($comp:ident),+) => {
        paste! {
            pub struct $ident<'a, $($comp),+>
            where
                $($comp: $crate::IterView<'a>,)+
            {
                dense: &'a [Entity],
                index: usize,
                $([<set_ $comp:lower>]: (&'a SparseArray, <$comp::SparseSet as $crate::SparseSetLike<'a>>::Slice),)+
            }

            impl<'a, $($comp),+> $ident<'a, $($comp),+> 
            where
                $($comp: $crate::IterView<'a>,)+
            {
                pub fn new($([<set_ $comp:lower>]: $comp::SparseSet),+) -> Self {
                    $(
                        let [<set_ $comp:lower>] = <$comp::SparseSet as $crate::SparseSetLike<'a>>::split([<set_ $comp:lower>]);
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
                $($comp: $crate::IterView<'a>,)+
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator2() {
        let mut s1 = SparseSet::<u32>::default();
        s1.insert(Entity::new(0, 0), 0);
        s1.insert(Entity::new(1, 0), 1);
        s1.insert(Entity::new(2, 0), 2);

        let mut s2 = SparseSet::<i32>::default();
        s2.insert(Entity::new(3, 0), 3);
        s2.insert(Entity::new(2, 0), 2);
        s2.insert(Entity::new(1, 0), 1);
        s2.insert(Entity::new(0, 0), 0);

        let mut iterator = Iterator2::<&u32, Option<&mut i32>>::new(&s1, &mut s2);
        assert!(matches!(iterator.next(), Some((&0, Some(&mut 0)))));
        assert!(matches!(iterator.next(), Some((&1, Some(&mut 1)))));
        assert!(matches!(iterator.next(), Some((&2, Some(&mut 2)))));
    }
}
