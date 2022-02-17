use crate::storage::{Component, Entity, IndexEntity, SparseArray};
use std::alloc::{alloc, dealloc, handle_alloc_error, realloc, Layout};
use std::ptr::NonNull;
use std::{mem, ptr, slice};

struct ComponentStorageFns {
    grow_amortized: unsafe fn(&mut ComponentStorage),
    swap_nonoverlapping: unsafe fn(&mut ComponentStorage, usize, usize),
    delete: unsafe fn(&mut ComponentStorage, entity: Entity),
    clear: unsafe fn(&mut ComponentStorage),
    drop: unsafe fn(&mut ComponentStorage),
}

/// Type-erased storage for components.
pub(crate) struct ComponentStorage {
    entities: NonNull<Entity>,
    len: usize,
    sparse: SparseArray,
    components: NonNull<u8>,
    cap: usize,
    fns: ComponentStorageFns,
}

unsafe impl Send for ComponentStorage {}
unsafe impl Sync for ComponentStorage {}

impl ComponentStorage {
    pub(crate) fn new<T>() -> Self
    where
        T: Component,
    {
        Self {
            entities: NonNull::dangling(),
            len: 0,
            sparse: SparseArray::default(),
            components: NonNull::<T>::dangling().cast(),
            cap: 0,
            fns: ComponentStorageFns {
                grow_amortized: Self::grow_amortized_typed::<T>,
                swap_nonoverlapping: Self::swap_nonoverlapping_typed::<T>,
                delete: Self::delete_typed::<T>,
                clear: Self::clear_typed::<T>,
                drop: Self::drop_typed::<T>,
            },
        }
    }

    pub(crate) unsafe fn insert<T>(&mut self, entity: Entity, component: T) -> Option<T>
    where
        T: Component,
    {
        let index_entity = self.sparse.get_mut_or_allocate_at(entity.sparse());

        match index_entity {
            Some(index_entity) => {
                let index = index_entity.dense();
                self.get_entity_ptr(index).write(entity);
                Some(self.get_component_ptr::<T>(index).replace(component))
            }
            None => {
                *index_entity = Some(IndexEntity::new(self.len as u32, entity.version()));

                if self.len == self.cap {
                    self.grow_amortized();
                }

                self.get_entity_ptr(self.len).write(entity);
                self.get_component_ptr::<T>(self.len).write(component);

                self.len += 1;
                None
            }
        }
    }

    pub(crate) unsafe fn remove<T>(&mut self, entity: Entity) -> Option<T>
    where
        T: Component,
    {
        let index = self.sparse.remove(entity)?;
        self.len -= 1;

        let last_entity = *self.get_entity_ptr(self.len);
        self.get_entity_ptr(index).write(last_entity);

        if index < self.len {
            let index_entity = IndexEntity::new(index as u32, last_entity.version());
            *self.sparse.get_unchecked_mut(last_entity.sparse()) = Some(index_entity);
        }

        let removed_ptr = self.get_component_ptr::<T>(index);
        let removed = removed_ptr.read();

        ptr::copy(self.get_component_ptr::<T>(self.len), removed_ptr, 1);
        Some(removed)
    }

    #[inline]
    pub(crate) fn delete(&mut self, entity: Entity) {
        unsafe { (self.fns.delete)(self, entity) }
    }

    pub(crate) unsafe fn delete_typed<T>(&mut self, entity: Entity)
    where
        T: Component,
    {
        let index = match self.sparse.remove(entity) {
            Some(index) => index,
            None => return,
        };

        self.len -= 1;

        let last_entity = *self.get_entity_ptr(self.len);
        self.get_entity_ptr(index).write(last_entity);

        if index < self.len {
            let index_entity = IndexEntity::new(index as u32, last_entity.version());
            *self.sparse.get_unchecked_mut(last_entity.sparse()) = Some(index_entity);
        }

        let dropped_ptr = self.get_component_ptr::<T>(index);
        dropped_ptr.drop_in_place();

        ptr::copy(self.get_component_ptr::<T>(self.len), dropped_ptr, 1);
    }

    #[inline]
    pub(crate) unsafe fn swap_nonoverlapping(&mut self, a: usize, b: usize) {
        (self.fns.swap_nonoverlapping)(self, a, b);
    }

    unsafe fn swap_nonoverlapping_typed<T>(&mut self, a: usize, b: usize)
    where
        T: Component,
    {
        debug_assert!(a < self.len);
        debug_assert!(b < self.len);

        let (sparse_a, sparse_b) = {
            let entity_a = &mut *self.get_entity_ptr(a);
            let entity_b = &mut *self.get_entity_ptr(b);
            mem::swap(entity_a, entity_b);

            (entity_a.sparse(), entity_b.sparse())
        };

        self.sparse.swap_nonoverlapping(sparse_a, sparse_b);

        let component_a = &mut *self.get_component_ptr::<T>(a);
        let component_b = &mut *self.get_component_ptr::<T>(a);
        mem::swap(component_a, component_b);
    }

    #[inline]
    pub(crate) unsafe fn get<T>(&self, entity: Entity) -> Option<&T>
    where
        T: Component,
    {
        let index = self.sparse.get(entity)?;
        Some(&*self.get_component_ptr(index))
    }

    #[inline]
    pub(crate) unsafe fn get_mut<T>(&self, entity: Entity) -> Option<&mut T>
    where
        T: Component,
    {
        let index = self.sparse.get(entity)?;
        Some(&mut *self.get_component_ptr(index))
    }

    #[inline]
    pub(crate) fn get_index(&self, entity: Entity) -> Option<usize> {
        self.sparse.get(entity)
    }

    #[inline]
    pub(crate) fn contains(&self, entity: Entity) -> bool {
        self.sparse.contains(entity)
    }

    #[inline]
    pub(crate) fn get_index_from_sparse(&self, sparse: usize) -> Option<usize> {
        self.sparse.get_from_sparse(sparse)
    }

    #[inline]
    pub(crate) fn contains_sparse(&self, sparse: usize) -> bool {
        self.sparse.contains_sparse(sparse)
    }

    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub(crate) fn entities(&self) -> &[Entity] {
        unsafe { slice::from_raw_parts(self.entities.as_ptr(), self.len) }
    }

    #[inline]
    pub(crate) unsafe fn components<T>(&self) -> &[T] {
        slice::from_raw_parts(self.components.cast::<T>().as_ptr(), self.len)
    }

    #[inline]
    pub(crate) unsafe fn components_mut<T>(&mut self) -> &mut [T] {
        slice::from_raw_parts_mut(self.components.cast::<T>().as_ptr(), self.len)
    }

    #[inline]
    pub(crate) unsafe fn split<T>(&self) -> (&[Entity], &SparseArray, &[T]) {
        (
            slice::from_raw_parts(self.entities.as_ptr(), self.len),
            &self.sparse,
            slice::from_raw_parts(self.components.cast::<T>().as_ptr(), self.len),
        )
    }

    #[inline]
    pub(crate) unsafe fn split_mut<T>(&mut self) -> (&[Entity], &SparseArray, &mut [T]) {
        (
            slice::from_raw_parts(self.entities.as_ptr(), self.len),
            &self.sparse,
            slice::from_raw_parts_mut(self.components.cast::<T>().as_ptr(), self.len),
        )
    }

    #[inline]
    pub(crate) fn clear(&mut self) {
        unsafe { (self.fns.clear)(self) }
    }

    fn clear_typed<T>(&mut self)
    where
        T: Component,
    {
        self.sparse.clear();

        let len = self.len;
        self.len = 0;

        for i in 0..len {
            unsafe {
                self.get_component_ptr::<T>(i).drop_in_place();
            }
        }
    }

    #[inline]
    unsafe fn get_entity_ptr(&self, index: usize) -> *mut Entity {
        self.entities.as_ptr().add(index)
    }

    #[inline]
    unsafe fn get_component_ptr<T>(&self, index: usize) -> *mut T
    where
        T: Component,
    {
        self.components.cast::<T>().as_ptr().add(index)
    }

    #[inline]
    fn grow_amortized(&mut self) {
        unsafe { (self.fns.grow_amortized)(self) }
    }

    unsafe fn grow_amortized_typed<T>(&mut self)
    where
        T: Component,
    {
        let (entities, components, cap) = if self.cap == 0 {
            let entities_layout = Layout::new::<Entity>();
            let entities = NonNull::new(alloc(entities_layout))
                .unwrap_or_else(|| handle_alloc_error(entities_layout));

            let components_layout = Layout::new::<T>();
            let components = if mem::size_of::<T>() != 0 {
                NonNull::new(alloc(components_layout))
                    .unwrap_or_else(|| handle_alloc_error(components_layout))
            } else {
                self.components
            };

            (entities, components, 1)
        } else {
            let cap = 2 * self.cap;

            let entities = {
                let old_layout = array_layout::<Entity>(self.cap);
                let layout = array_layout::<Entity>(cap);

                NonNull::new(realloc(self.entities.as_ptr().cast(), old_layout, layout.size()))
                    .unwrap_or_else(|| handle_alloc_error(layout))
            };

            let components = if mem::size_of::<T>() != 0 {
                let old_layout = array_layout::<T>(self.cap);
                let layout = array_layout::<T>(cap);

                NonNull::new(realloc(self.components.as_ptr(), old_layout, layout.size()))
                    .unwrap_or_else(|| handle_alloc_error(layout))
            } else {
                self.components
            };

            (entities, components, cap)
        };

        self.entities = entities.cast();
        self.components = components;
        self.cap = cap;
    }

    unsafe fn drop_typed<T>(&mut self)
    where
        T: Component,
    {
        if self.cap != 0 {
            self.clear_typed::<T>();

            dealloc(self.entities.as_ptr().cast(), array_layout::<Entity>(self.cap));

            if mem::size_of::<T>() != 0 {
                dealloc(self.components.as_ptr(), array_layout::<T>(self.cap));
            }
        }
    }
}

impl Drop for ComponentStorage {
    fn drop(&mut self) {
        unsafe { (self.fns.drop)(self) }
    }
}

#[inline]
fn array_layout<U>(n: usize) -> Layout {
    Layout::array::<U>(n).unwrap()
}
