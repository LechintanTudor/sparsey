use crate::{Component, Entity, SparseArray, SparseSet};

fn shortest_dense<'a>(d1: &'a [Entity], d2: &'a [Entity]) -> &'a [Entity] {
    if d1.len() < d2.len() {
        d1
    } else {
        d2
    }
}

macro_rules! find_shortest_dense {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($y:expr),+) => {
        shortest_dense($x, find_shortest_dense!($($y),+))
    };
}

macro_rules! find_shortest_dense2 {
    ($($x:expr),+) => {
        find_shortest_dense!($($x.1),+)
    };
}

pub trait IterView<'a> {
    type Component: Component;
    type Output: 'a;

    fn from_option(option: Option<&'a Self::Component>) -> Option<Self::Output>;
}

impl<'a, T> IterView<'a> for &'a T
where
    T: Component,
{
    type Component = T;
    type Output = &'a T;

    fn from_option(option: Option<&'a Self::Component>) -> Option<Self::Output> {
        option
    }
}

impl<'a, T> IterView<'a> for Option<&'a T>
where
    T: Component,
{
    type Component = T;
    type Output = Option<&'a T>;

    fn from_option(option: Option<&'a Self::Component>) -> Option<Self::Output> {
        Some(option)
    }
}

pub struct Iterator2<'a, A, B>
where
    A: IterView<'a>,
    B: IterView<'a>,
{
    dense: &'a [Entity],
    c0: (&'a SparseArray, &'a [A::Component]),
    c1: (&'a SparseArray, &'a [B::Component]),
    current_index: usize,
}

impl<'a, A, B> Iterator2<'a, A, B>
where
    A: IterView<'a>,
    B: IterView<'a>,
{
    pub fn new(c0: &'a SparseSet<A::Component>, c1: &'a SparseSet<B::Component>) -> Self {
        let c0 = c0.split_for_iteration();
        let c1 = c1.split_for_iteration();

        let dense = find_shortest_dense2!(c0, c1);

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

            let current_item = (|| {
                Some((
                    A::from_option(self.c0.0.get_valid(entity).map(|e| &self.c0.1[e.index()]))?,
                    B::from_option(self.c1.0.get_valid(entity).map(|e| &self.c1.1[e.index()]))?,
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

        let mut iterator = Iterator2::<&u32, &i32>::new(&s1, &s2);
        assert!(matches!(iterator.next(), Some((&0, &0))));
        assert!(matches!(iterator.next(), Some((&1, &1))));
        assert!(matches!(iterator.next(), Some((&2, &2))));
    }
}
