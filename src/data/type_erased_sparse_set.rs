use crate::data::{Component, ComponentFlags, Entity, SparseArray, TypeErasedVec};

pub struct TypeErasedSparseSet {
    sparse: SparseArray,
    dense: Vec<Entity>,
    flags: Vec<ComponentFlags>,
    data: Box<dyn TypeErasedVec>,
}

impl TypeErasedSparseSet {
    pub fn new<T>() -> Self
    where
        T: Component,
    {
        Self {
            sparse: Default::default(),
            dense: Default::default(),
            flags: Default::default(),
            data: Box::new(Vec::<T>::new()),
        }
    }

    pub fn clear(&mut self) {
        self.sparse.clear();
        self.dense.clear();
        self.flags.clear();
        self.data.clear();
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        let sparse_index_a = self.dense[a].index();
        let sparse_index_b = self.dense[b].index();

        unsafe {
            self.sparse.swap_unchecked(sparse_index_a, sparse_index_b);
        }

        self.dense.swap(a, b);
        self.flags.swap(a, b);
        self.data.swap(a, b);
    }

    pub fn len(&self) -> usize {
        self.dense.len()
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.sparse.contains(entity)
    }
}
