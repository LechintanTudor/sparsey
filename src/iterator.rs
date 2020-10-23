use crate::{Component, Entity, SparseArray, SparseSet};

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

impl<'a, A, B> Iterator for Iterator2<'a, A, B>
where
    A: IterView<'a>,
    B: IterView<'a>,
{
    type Item = (A::Output, B::Output);

    // TODO: Check for INVALID_INDEX
    fn next(&mut self) -> Option<Self::Item> {
        let entity = *self.dense.get(self.current_index)?;
        self.current_index += 1;

        Some((
            A::from_option(self.c0.0.get(entity).map(|e| &self.c0.1[e.index()]))?,
            B::from_option(self.c1.0.get(entity).map(|e| &self.c1.1[e.index()]))?,
        ))
    }
}
