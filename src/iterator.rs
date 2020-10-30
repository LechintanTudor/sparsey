use crate::{Entity, SparseArray, SparseSet};

fn shortest_dense<'a>(d1: &'a [Entity], d2: &'a [Entity]) -> &'a [Entity] {
    if d1.len() < d2.len() {
        d1
    } else {
        d2
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
        find_shortest_dense_inner!($($x.1),+)
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
    type SparseSet: SparseSetLike<'a>;
    type Output: 'a;

    fn fetch(value: Option<<Self::SparseSet as SparseSetLike<'a>>::Ref>) -> Option<Self::Output>;
}

impl<'a, T> IterView<'a> for &'a T {
    type SparseSet = &'a SparseSet<T>;
    type Output = Self;

    fn fetch(value: Option<<Self::SparseSet as SparseSetLike<'a>>::Ref>) -> Option<Self::Output> {
        value
    }
}

impl<'a, T> IterView<'a> for &'a mut T {
    type SparseSet = &'a mut SparseSet<T>;
    type Output = Self;

    fn fetch(value: Option<<Self::SparseSet as SparseSetLike<'a>>::Ref>) -> Option<Self::Output> {
        value
    }
}

impl<'a, T> IterView<'a> for Option<&'a T> {
    type SparseSet = &'a SparseSet<T>;
    type Output = Self;

    fn fetch(value: Option<<Self::SparseSet as SparseSetLike<'a>>::Ref>) -> Option<Self::Output> {
        Some(value)
    }
}

impl<'a, T> IterView<'a> for Option<&'a mut T> {
    type SparseSet = &'a mut SparseSet<T>;
    type Output = Self;

    fn fetch(value: Option<<Self::SparseSet as SparseSetLike<'a>>::Ref>) -> Option<Self::Output> {
        Some(value)
    }
}

unsafe fn fetch<'a, T>(
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

pub struct Iterator2<'a, A, B>
where
    A: IterView<'a>,
    B: IterView<'a>,
{
    dense: &'a [Entity],
    c0: (&'a SparseArray, <A::SparseSet as SparseSetLike<'a>>::Slice),
    c1: (&'a SparseArray, <B::SparseSet as SparseSetLike<'a>>::Slice),
    current_index: usize,
}

impl<'a, A, B> Iterator2<'a, A, B>
where
    A: IterView<'a>,
    B: IterView<'a>,
{
    pub fn new(c0: A::SparseSet, c1: B::SparseSet) -> Self {
        let c0 = <A::SparseSet as SparseSetLike<'a>>::split(c0);
        let c1 = <B::SparseSet as SparseSetLike<'a>>::split(c1);

        let dense = find_shortest_dense!(c0, c1);

        Self {
            dense,
            c0: (c0.0, c0.2),
            c1: (c1.0, c1.2),
            current_index: 0,
        }
    }
}

impl<'a, A, B> Iterator for Iterator2<'a, A, B>
where
    A: IterView<'a>,
    B: IterView<'a>,
{
    type Item = (A::Output, B::Output);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entity = *self.dense.get(self.current_index)?;
            self.current_index += 1;

            let current_item = (|| unsafe {
                Some((
                    fetch::<A>(self.c0.0, self.c0.1, entity)?,
                    fetch::<B>(self.c1.0, self.c1.1, entity)?,
                ))
            })();

            if current_item.is_some() {
                return current_item;
            }
        }
    }
}

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
