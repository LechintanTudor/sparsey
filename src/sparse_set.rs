use crate::{Entity, SparseArray};
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
        let sparse_entity = self.sparse.get_mut_or_allocate(entity);

        if sparse_entity.is_valid() {
            Some(mem::replace(&mut self.data[sparse_entity.index()], value))
        } else {
            *sparse_entity = Entity::new(self.dense.len() as u32, sparse_entity.gen());
            self.dense.push(entity);
            self.data.push(value);
            None
        }
    }

    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        let sparse_entity = *self.sparse.get(entity)?;

        if sparse_entity.is_valid() {
            let last_dense = *self.dense.last()?;
            self.dense.swap_remove(sparse_entity.index());

            unsafe {
                *self.sparse.get_mut_unchecked(last_dense) = sparse_entity;
                *self.sparse.get_mut_unchecked(entity) = Entity::INVALID;
            }

            Some(self.data.swap_remove(sparse_entity.index()))
        } else {
            None
        }
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.sparse.contains(entity)
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        let sparse_entity = *self.sparse.get(entity)?;

        if sparse_entity.is_valid() {
            Some(&self.data[sparse_entity.index()])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        let sparse_entity = *self.sparse.get(entity)?;

        if sparse_entity.is_valid() {
            Some(&mut self.data[sparse_entity.index()])
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> &[T] {
        self.as_ref()
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.as_mut()
    }

    pub fn dense(&self) -> &[Entity] {
        &self.dense
    }

    pub fn split(&self) -> (&SparseArray, &[Entity], &[T]) {
        (&self.sparse, &self.dense, &self.data)
    }

    pub fn split_mut(&mut self) -> (&SparseArray, &[Entity], &mut [T]) {
        (&self.sparse, &self.dense, &mut self.data)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut set = SparseSet::<u32>::default();
        set.insert(Entity::new(100, 0), 10);
        set.insert(Entity::new(200, 0), 20);
        set.insert(Entity::new(300, 0), 30);

        assert_eq!(set.insert(Entity::new(100, 0), 11), Some(10));
        assert_eq!(set.insert(Entity::new(200, 0), 21), Some(20));
        assert_eq!(set.insert(Entity::new(300, 0), 31), Some(30));

        assert_eq!(set.get(Entity::new(100, 0)), Some(&11));
        assert_eq!(set.get(Entity::new(200, 0)), Some(&21));
        assert_eq!(set.get(Entity::new(300, 0)), Some(&31));
    }

    #[test]
    fn remove() {
        let mut set = SparseSet::<u32>::default();
        set.insert(Entity::new(0, 0), 10);
        set.insert(Entity::new(1, 0), 20);
        set.insert(Entity::new(2, 0), 30);

        assert_eq!(set.remove(Entity::new(0, 0)), Some(10));
        assert_eq!(set.remove(Entity::new(0, 0)), None);

        assert_eq!(set.remove(Entity::new(1, 0)), Some(20));
        assert_eq!(set.remove(Entity::new(1, 0)), None);

        assert_eq!(set.remove(Entity::new(2, 0)), Some(30));
        assert_eq!(set.remove(Entity::new(2, 0)), None);
    }
}
