use crate::storage::{
    AbstractSparseSet, AbstractSparseSetView, AbstractSparseSetViewMut, Entity, IndexEntity,
    SparseArray,
};
use bitflags::bitflags;
use std::ops::{Deref, DerefMut};
use std::{mem, ptr};

bitflags! {
    pub struct ComponentFlags: u8 {
        const ADDED   = 0b00000001;
        const CHANGED = 0b00000010;
    }
}

pub struct ComponentRefMut<'a, T>
where
    T: 'static,
{
    data: &'a mut T,
    flags: &'a mut ComponentFlags,
}

impl<'a, T> ComponentRefMut<'a, T>
where
    T: 'static,
{
    pub fn new(data: &'a mut T, flags: &'a mut ComponentFlags) -> Self {
        Self { data, flags }
    }
}

impl<T> Deref for ComponentRefMut<'_, T>
where
    T: 'static,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for ComponentRefMut<'_, T>
where
    T: 'static,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.flags.insert(ComponentFlags::CHANGED);
        self.data
    }
}

pub struct SparseSet<T> {
    sparse: SparseArray,
    dense: Vec<Entity>,
    data: Vec<T>,
    flags: Vec<ComponentFlags>,
}

impl<T> Default for SparseSet<T> {
    fn default() -> Self {
        Self {
            sparse: Default::default(),
            dense: Default::default(),
            data: Default::default(),
            flags: Default::default(),
        }
    }
}

impl<T> SparseSet<T> {
    pub fn insert(&mut self, entity: Entity, value: T) -> Option<T> {
        let index_entity = self.sparse.get_mut_or_allocate(entity.index());

        match index_entity {
            Some(e) => {
                if e.id() == entity.id() {
                    self.flags[e.index()].insert(ComponentFlags::CHANGED);
                } else {
                    self.flags[e.index()].insert(ComponentFlags::ADDED);
                }

                *e = IndexEntity::new(e.id(), entity.gen());
                Some(mem::replace(&mut self.data[e.index()], value))
            }
            None => {
                *index_entity = Some(IndexEntity::new(self.dense.len() as u32, entity.gen()));
                self.dense.push(entity);
                self.data.push(value);
                self.flags.push(ComponentFlags::ADDED);
                None
            }
        }
    }

    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        let index_entity = self.sparse.get_index_entity(entity)?;

        let last_index = self.dense.last()?.index();
        self.dense.swap_remove(index_entity.index());
        self.flags.swap_remove(index_entity.index());

        unsafe {
            *self.sparse.get_unchecked_mut(last_index) = Some(index_entity);
            *self.sparse.get_unchecked_mut(entity.index()) = None;
        }

        Some(self.data.swap_remove(index_entity.index()))
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.sparse.contains(entity)
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        let index = self.sparse.get_index_entity(entity)?.index();
        unsafe { Some(self.data.get_unchecked(index)) }
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<ComponentRefMut<T>> {
        let index = self.sparse.get_index_entity(entity)?.index();
        unsafe {
            Some(ComponentRefMut::new(
                self.data.get_unchecked_mut(index),
                self.flags.get_unchecked_mut(index),
            ))
        }
    }

    pub fn swap(&mut self, a: usize, b: usize) -> bool {
        if a < self.len() && b < self.len() && a != b {
            unsafe {
                self.swap_unchecked(a, b);
            }
            true
        } else {
            false
        }
    }

    pub fn split(&self) -> (&SparseArray, &[Entity], &[T], &[ComponentFlags]) {
        (&self.sparse, &self.dense, &self.data, &self.flags)
    }

    pub fn split_mut(&mut self) -> (&SparseArray, &[Entity], &mut [T], &mut [ComponentFlags]) {
        (&self.sparse, &self.dense, &mut self.data, &mut self.flags)
    }

    pub unsafe fn split_raw(
        &mut self,
    ) -> (
        &mut SparseArray,
        &mut [Entity],
        &mut [T],
        &mut [ComponentFlags],
    ) {
        (
            &mut self.sparse,
            &mut self.dense,
            &mut self.data,
            &mut self.flags,
        )
    }

    pub unsafe fn swap_unchecked(&mut self, a: usize, b: usize) {
        let index_a = self.dense.get_unchecked(a).index();
        let index_b = self.dense.get_unchecked(b).index();
        self.sparse.swap_unchecked(index_a, index_b);

        ptr::swap_nonoverlapping(
            self.dense.as_mut_ptr().add(a),
            self.dense.as_mut_ptr().add(b),
            1,
        );

        ptr::swap_nonoverlapping(
            self.data.as_mut_ptr().add(a),
            self.data.as_mut_ptr().add(b),
            1,
        );

        ptr::swap_nonoverlapping(
            self.flags.as_mut_ptr().add(a),
            self.flags.as_mut_ptr().add(b),
            1,
        );
    }
}

impl<T> AbstractSparseSet for SparseSet<T>
where
    T: Send + Sync + 'static,
{
    fn delete(&mut self, entity: Entity) {
        self.remove(entity);
    }

    fn maintain(&mut self) {
        self.flags
            .iter_mut()
            .for_each(|f| *f = ComponentFlags::empty());
    }

    fn as_abstract_view(&self) -> AbstractSparseSetView {
        AbstractSparseSetView::new(self)
    }

    fn as_abstract_view_mut(&mut self) -> AbstractSparseSetViewMut {
        AbstractSparseSetViewMut::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut set = SparseSet::<u32>::default();
        set.insert(Entity::with_id(1), 10);
        set.insert(Entity::with_id(2), 20);
        set.insert(Entity::with_id(3), 30);

        assert_eq!(set.insert(Entity::with_id(1), 11), Some(10));
        assert_eq!(set.insert(Entity::with_id(2), 21), Some(20));
        assert_eq!(set.insert(Entity::with_id(3), 31), Some(30));

        assert_eq!(set.get(Entity::with_id(1)), Some(&11));
        assert_eq!(set.get(Entity::with_id(2)), Some(&21));
        assert_eq!(set.get(Entity::with_id(3)), Some(&31));
    }

    #[test]
    fn remove() {
        let mut set = SparseSet::<u32>::default();
        set.insert(Entity::with_id(0), 10);
        set.insert(Entity::with_id(1), 20);
        set.insert(Entity::with_id(2), 30);

        assert_eq!(set.remove(Entity::with_id(0)), Some(10));
        assert_eq!(set.remove(Entity::with_id(0)), None);

        assert_eq!(set.remove(Entity::with_id(1)), Some(20));
        assert_eq!(set.remove(Entity::with_id(1)), None);

        assert_eq!(set.remove(Entity::with_id(2)), Some(30));
        assert_eq!(set.remove(Entity::with_id(2)), None);
    }
}
