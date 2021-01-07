use crate::storage::{
    ComponentFlags, Entity, IndexEntity, SparseArray, SparseSet, COMPONENT_FLAG_NONE,
};
use std::any::Any;
use std::{mem, ptr, slice};

pub trait AbstractStorage
where
    Self: 'static,
{
    fn clear_flags(&mut self);

    fn as_any(&self) -> &dyn Any;

    fn as_mut_any(&mut self) -> &mut dyn Any;

    fn as_storage_view_mut(&mut self) -> AbstractStorageViewMut;
}

pub struct AbstractStorageViewMut<'a> {
    sparse: &'a mut SparseArray,
    dense: &'a mut [Entity],
    data: *mut (),
    flags: *mut ComponentFlags,
    component_size: usize,
}

impl<'a> AbstractStorageViewMut<'a> {
    pub fn new<T>(set: &'a mut SparseSet<T>) -> Self {
        let (sparse, dense, data, flags) = unsafe { set.split_raw() };

        Self {
            sparse,
            dense,
            data: data.as_mut_ptr() as _,
            component_size: mem::size_of::<T>(),
            flags: flags.as_mut_ptr(),
        }
    }

    pub fn clear_flags(&mut self) {
        let flags = unsafe { slice::from_raw_parts_mut(self.flags, self.len()) };
        flags.iter_mut().for_each(|f| *f = COMPONENT_FLAG_NONE);
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

                ptr::swap_nonoverlapping(self.flags.add(a), self.flags.add(b), 1);
            }
        }
    }
}
