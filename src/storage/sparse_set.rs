use crate::{
    data::view::StorageView,
    entity::{Entity, IndexEntity},
    storage::{SparseArray, SparseSetView, SparseSetViewMut},
};
use std::mem;

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

    pub fn as_slice(&self) -> &[T] {
        self.as_ref()
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.as_mut()
    }
}

impl<T> SparseSetView for SparseSet<T> {
    type Component = T;

    fn sparse(&self) -> &SparseArray {
        &self.sparse
    }

    fn dense(&self) -> &[Entity] {
        &self.dense
    }

    fn data(&self) -> &[Self::Component] {
        &self.data
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn contains(&self, entity: Entity) -> bool {
        self.sparse.contains(entity)
    }

    fn get(&self, entity: Entity) -> Option<&T> {
        let index = self.sparse.get(entity)?.index();
        unsafe { Some(self.data.get_unchecked(index)) }
    }

    fn split(&self) -> (&SparseArray, &[Entity], &[Self::Component]) {
        (&self.sparse, &self.dense, &self.data)
    }

    unsafe fn get_unchecked(&self, entity: Entity) -> &Self::Component {
        self.data.get_unchecked(entity.index())
    }
}

impl<T> SparseSetViewMut for SparseSet<T> {
    fn data_mut(&mut self) -> &mut [Self::Component] {
        &mut self.data
    }

    fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        let index = self.sparse.get(entity)?.index();
        unsafe { Some(self.data.get_unchecked_mut(index)) }
    }

    fn split_mut(&mut self) -> (&SparseArray, &[Entity], &mut [T]) {
        (&self.sparse, &self.dense, &mut self.data)
    }

    unsafe fn get_unchecked_mut(&mut self, entity: Entity) -> &mut Self::Component {
        self.data.get_unchecked_mut(entity.index())
    }
}

impl<T> AsRef<[T]> for SparseSet<T> {
    fn as_ref(&self) -> &[T] {
        &self.data
    }
}

impl<T> AsMut<[T]> for SparseSet<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}

impl<'a, T> StorageView<'a> for &'a SparseSet<T> {
    const STRICT: bool = true;
    type Output = &'a T;
    type Component = &'a T;
    type Data = *const T;

    unsafe fn split_for_iteration(self) -> (&'a SparseArray, &'a [Entity], Self::Data) {
        let (sparse, dense, data) = SparseSetView::split(self);
        (sparse, dense, data.as_ptr())
    }

    unsafe fn get_output(self, entity: Entity) -> Option<Self::Output> {
        SparseSetView::get(self, entity)
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
        let (sparse, dense, data) = SparseSetViewMut::split_mut(self);
        (sparse, dense, data.as_mut_ptr())
    }

    unsafe fn get_output(self, entity: Entity) -> Option<Self::Output> {
        SparseSetViewMut::get_mut(self, entity)
    }

    unsafe fn get_component(data: Self::Data, entity: IndexEntity) -> Self::Component {
        &mut *data.add(entity.index())
    }

    unsafe fn get_from_component(component: Option<Self::Component>) -> Option<Self::Output> {
        component
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
