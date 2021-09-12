use crate::storage::Entity;
use crate::utils::{ChangeTicks, EntityIterator};

/// Iterator over the components in a `ComponentView`.
#[derive(Clone, Copy)]
pub struct ComponentIter<'a, T> {
    entities: &'a [Entity],
    components: *const T,
    index: usize,
}

impl<'a, T> ComponentIter<'a, T> {
    pub(crate) unsafe fn new(entities: &'a [Entity], components: &'a [T]) -> Self {
        Self {
            entities,
            components: components.as_ptr(),
            index: 0,
        }
    }
}

impl<'a, T> Iterator for ComponentIter<'a, T>
where
    T: 'a,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.entities.len() {
            return None;
        }

        let index = self.index;
        self.index += 1;

        unsafe { Some(&*self.components.add(index)) }
    }
}

unsafe impl<'a, T> EntityIterator for ComponentIter<'a, T>
where
    T: 'a,
{
    fn next_with_entity(&mut self) -> Option<(Entity, Self::Item)> {
        if self.index >= self.entities.len() {
            return None;
        }

        let index = self.index;
        self.index += 1;

        unsafe {
            Some((
                *self.entities.get_unchecked(index),
                &*self.components.add(index),
            ))
        }
    }
}

/// Iterator over the components and `ChangeTicks` in a `ComponentView`.
#[derive(Clone, Copy)]
pub struct ComponentWithTicksIter<'a, T> {
    entities: &'a [Entity],
    components: *const T,
    ticks: *const ChangeTicks,
    index: usize,
}

impl<'a, T> ComponentWithTicksIter<'a, T> {
    pub(crate) unsafe fn new(
        entities: &'a [Entity],
        components: &'a [T],
        ticks: &'a [ChangeTicks],
    ) -> Self {
        Self {
            entities,
            components: components.as_ptr(),
            ticks: ticks.as_ptr(),
            index: 0,
        }
    }
}

impl<'a, T> Iterator for ComponentWithTicksIter<'a, T>
where
    T: 'a,
{
    type Item = (&'a T, &'a ChangeTicks);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.entities.len() {
            return None;
        }

        let index = self.index;
        self.index += 1;

        unsafe { Some((&*self.components.add(index), &*self.ticks.add(index))) }
    }
}

unsafe impl<'a, T> EntityIterator for ComponentWithTicksIter<'a, T>
where
    T: 'a,
{
    fn next_with_entity(&mut self) -> Option<(Entity, Self::Item)> {
        if self.index >= self.entities.len() {
            return None;
        }

        let index = self.index;
        self.index += 1;

        unsafe {
            Some((
                *self.entities.get_unchecked(index),
                (&*self.components.add(index), &*self.ticks.add(index)),
            ))
        }
    }
}
