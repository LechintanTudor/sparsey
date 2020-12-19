use crate::{
    data::view::StorageView,
    entity::{Entity, IndexEntity},
    storage::{AbstractStorage, AbstractStorageViewMut, SparseArray},
};
use std::{any::Any, mem, ptr};

pub struct SparseSet<T> {
    sparse: SparseArray,
    dense: Vec<Entity>,
    data: Vec<T>,
}

impl<T> Default for SparseSet<T> {
    fn default() -> Self {
        Self {
            sparse: Default::default(),
            dense: Default::default(),
            data: Default::default(),
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
            *sparse_entity = IndexEntity::new(sparse_entity.id(), entity.gen());
            Some(mem::replace(&mut self.data[sparse_entity.index()], value))
        } else {
            *sparse_entity = IndexEntity::new(self.dense.len() as u32, entity.gen());
            self.dense.push(entity);
            self.data.push(value);
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

        unsafe {
            *self.sparse.get_unchecked_mut(last_index) = sparse_entity;
            *self.sparse.get_unchecked_mut(entity.index()) = IndexEntity::INVALID;
        }

        Some(self.data.swap_remove(sparse_entity.index()))
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.sparse.contains(entity)
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        let index = self.sparse.get(entity)?.index();
        unsafe { Some(self.data.get_unchecked(index)) }
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        let index = self.sparse.get(entity)?.index();
        unsafe { Some(self.data.get_unchecked_mut(index)) }
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

    pub fn sparse(&self) -> &SparseArray {
        &self.sparse
    }

    pub fn dense(&self) -> &[Entity] {
        &self.dense
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn split(&self) -> (&SparseArray, &[Entity], &[T]) {
        (&self.sparse, &self.dense, &self.data)
    }

    pub fn split_mut(&mut self) -> (&SparseArray, &[Entity], &mut [T]) {
        (&self.sparse, &self.dense, &mut self.data)
    }

    pub unsafe fn split_raw(&mut self) -> (&mut SparseArray, &mut [Entity], &mut [T]) {
        (&mut self.sparse, &mut self.dense, &mut self.data)
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
        )
    }
}

impl<'a, T> StorageView<'a> for &'a SparseSet<T> {
    const STRICT: bool = true;
    type Output = &'a T;
    type Component = &'a T;
    type Data = *const T;

    unsafe fn split_for_iteration(self) -> (&'a SparseArray, &'a [Entity], Self::Data) {
        let (sparse, dense, data) = self.split();
        (sparse, dense, data.as_ptr())
    }

    unsafe fn get_output(self, entity: Entity) -> Option<Self::Output> {
        self.get(entity)
    }

    unsafe fn get_component(data: Self::Data, entity: IndexEntity) -> Self::Component {
        &*data.add(entity.index())
    }

    unsafe fn get_from_component(component: Option<Self::Component>) -> Option<Self::Output> {
        component
    }
}

impl<'a, T> StorageView<'a> for &'a mut SparseSet<T> {
    const STRICT: bool = true;
    type Output = &'a mut T;
    type Component = &'a mut T;
    type Data = *mut T;

    unsafe fn split_for_iteration(self) -> (&'a SparseArray, &'a [Entity], Self::Data) {
        let (sparse, dense, data) = self.split_mut();
        (sparse, dense, data.as_mut_ptr())
    }

    unsafe fn get_output(self, entity: Entity) -> Option<Self::Output> {
        self.get_mut(entity)
    }

    unsafe fn get_component(data: Self::Data, entity: IndexEntity) -> Self::Component {
        &mut *data.add(entity.index())
    }

    unsafe fn get_from_component(component: Option<Self::Component>) -> Option<Self::Output> {
        component
    }
}

impl<T> AbstractStorage for SparseSet<T>
where
    T: 'static,
{
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
