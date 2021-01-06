use crate::entity::{Entity, IndexEntity};
use crate::storage::{AbstractStorage, AbstractStorageViewMut, SparseArray};
use std::any::Any;
use std::ops::{Deref, DerefMut};
use std::{mem, ptr};

pub const COMPONENT_FLAG_NONE: ComponentFlags = 0;
pub const COMPONENT_FLAG_CHANGED: ComponentFlags = 1;
pub const COMPONENT_FLAG_ADDED: ComponentFlags = 1 << 1;

pub type ComponentFlags = u8;

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

    pub fn flags(component_ref: &ComponentRefMut<'a, T>) -> ComponentFlags {
        *component_ref.flags
    }

    pub fn into_component(component_ref: ComponentRefMut<'a, T>) -> &'a mut T {
        component_ref.data
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
        *self.flags |= COMPONENT_FLAG_CHANGED;
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
        if !entity.is_valid() {
            return None;
        }

        let sparse_entity = self.sparse.get_mut_or_allocate(entity.index());

        if sparse_entity.is_valid() {
            if sparse_entity.id() == entity.id() {
                self.flags[sparse_entity.index()] |= COMPONENT_FLAG_CHANGED;
            } else {
                self.flags[sparse_entity.index()] = COMPONENT_FLAG_ADDED;
            }

            *sparse_entity = IndexEntity::new(sparse_entity.id(), entity.gen());
            Some(mem::replace(&mut self.data[sparse_entity.index()], value))
        } else {
            *sparse_entity = IndexEntity::new(self.dense.len() as u32, entity.gen());
            self.dense.push(entity);
            self.data.push(value);
            self.flags.push(COMPONENT_FLAG_ADDED);
            None
        }
    }

    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        let sparse_entity = *self.sparse.get(entity)?;

        if !sparse_entity.is_valid() {
            return None;
        }

        let last_index = self.dense.last()?.index();
        self.dense.swap_remove(sparse_entity.index());
        self.flags.swap_remove(sparse_entity.index());

        unsafe {
            *self.sparse.get_unchecked_mut(last_index) = sparse_entity;
            *self.sparse.get_unchecked_mut(entity.index()) = IndexEntity::INVALID;
        }

        Some(self.data.swap_remove(sparse_entity.index()))
    }

    pub fn clear_flags(&mut self) {
        self.flags.iter_mut().for_each(|f| *f = COMPONENT_FLAG_NONE);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.sparse.contains(entity)
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        let index = self.sparse.get(entity)?.index();
        unsafe { Some(self.data.get_unchecked(index)) }
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<ComponentRefMut<T>> {
        let index = self.sparse.get(entity)?.index();
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

impl<T> AbstractStorage for SparseSet<T>
where
    T: 'static,
{
    fn clear_flags(&mut self) {
        self.flags.iter_mut().for_each(|f| *f = 0);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn as_storage_view_mut(&mut self) -> AbstractStorageViewMut {
        AbstractStorageViewMut::new(self)
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
