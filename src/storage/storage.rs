use crate::{
    entity::{Entity, IndexEntity},
    storage::{SparseArray, SparseSet},
};
use std::{any::Any, mem, ptr};

pub trait AbstractStorage
where
    Self: 'static,
{
    fn as_any(&self) -> &dyn Any;

    fn as_mut_any(&mut self) -> &mut dyn Any;

    fn as_raw_storage_view(&mut self) -> RawStorageView;
}

pub struct RawStorageView<'a> {
    sparse: &'a mut SparseArray,
    dense: &'a mut [Entity],
    data: *mut (),
    component_size: usize,
}

impl<'a> RawStorageView<'a> {
    pub fn new<T>(set: &'a mut SparseSet<T>) -> Self {
        let (sparse, dense, data) = unsafe { set.split_raw() };

        Self {
            sparse,
            dense,
            data: data.as_mut_ptr() as _,
            component_size: mem::size_of::<T>(),
        }
    }

    pub fn len(&self) -> usize {
        self.dense.len()
    }

    pub fn get_index_entity(&self, entity: Entity) -> Option<&IndexEntity> {
        self.sparse.get(entity)
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        if a < self.len() && b < self.len() && a != b {
            unsafe {
                let index_a = self.dense.get_unchecked(a).index();
                let index_b = self.dense.get_unchecked(b).index();
                self.sparse.swap_unchecked(index_a, index_b);

                ptr::swap_nonoverlapping(
                    self.dense.as_mut_ptr().add(a),
                    self.dense.as_mut_ptr().add(b),
                    1,
                );

                ptr::swap_nonoverlapping(
                    self.data.add(a * self.component_size),
                    self.data.add(b * self.component_size),
                    self.component_size,
                );
            }
        }
    }
}
