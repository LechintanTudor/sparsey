use crate::entity::{Component, DenseEntity, Entity, SparseVec};
use std::alloc::{alloc, dealloc, handle_alloc_error, Layout, LayoutError};
use std::ptr::NonNull;
use std::{fmt, mem, ptr, slice};

pub(crate) struct ComponentSparseSet {
    sparse: SparseVec,
    entities: NonNull<Entity>,
    components: NonNull<u8>,
    len: usize,
    cap: usize,
    vtable: ComponentSparseSetVtable,
}

impl ComponentSparseSet {
    #[must_use]
    pub const fn new<T>() -> Self
    where
        T: Component,
    {
        Self {
            sparse: SparseVec::new(),
            entities: NonNull::dangling(),
            components: NonNull::<T>::dangling().cast(),
            len: 0,
            cap: 0,
            vtable: ComponentSparseSetVtable::new::<T>(),
        }
    }

    pub unsafe fn insert<T>(&mut self, entity: Entity, component: T) -> Option<T>
    where
        T: Component,
    {
        let dense_entity = self.sparse.get_mut_or_allocate_at(entity.sparse());

        match dense_entity {
            Some(dense_entity) => {
                let index = dense_entity.dense();

                self.entities.as_ptr().add(index).write(entity);

                Some(
                    self.components
                        .as_ptr()
                        .cast::<T>()
                        .add(index)
                        .replace(component),
                )
            }
            None => {
                *dense_entity = Some(DenseEntity {
                    index: self.len as u32,
                    version: entity.version,
                });

                if self.len == self.cap {
                    self.grow();
                }

                self.entities.as_ptr().add(self.len).write(entity);

                self.components
                    .cast::<T>()
                    .as_ptr()
                    .add(self.len)
                    .write(component);

                self.len += 1;
                None
            }
        }
    }

    pub unsafe fn remove<T>(&mut self, entity: Entity) -> Option<T>
    where
        T: Component,
    {
        let index = self.sparse.remove(entity)?.dense();
        self.len -= 1;

        let last_entity = *self.entities.as_ptr().add(self.len);
        *self.entities.as_ptr().add(index) = last_entity;

        if index < self.len {
            *self.sparse.get_unchecked_mut(last_entity.sparse()) = Some(DenseEntity {
                index: index as u32,
                version: last_entity.version,
            });
        }

        let removed_ptr = self.components.cast::<T>().as_ptr().add(index);
        let last_ptr = self.components.cast::<T>().as_ptr().add(self.len);

        let component = removed_ptr.read();
        ptr::copy(last_ptr, removed_ptr, 1);
        Some(component)
    }

    pub unsafe fn delete<T>(&mut self, entity: Entity)
    where
        T: Component,
    {
        let index = match self.sparse.remove(entity) {
            Some(dense_entity) => dense_entity.dense(),
            None => return,
        };

        self.len -= 1;

        let last_entity = *self.entities.as_ptr().add(self.len);
        *self.entities.as_ptr().add(index) = last_entity;

        if index < self.len {
            *self.sparse.get_unchecked_mut(last_entity.sparse()) = Some(DenseEntity {
                index: index as u32,
                version: last_entity.version,
            });
        }

        let dropped_ptr = self.components.cast::<T>().as_ptr().add(index);
        dropped_ptr.drop_in_place();

        let last_ptr = self.components.cast::<T>().as_ptr().add(self.len);
        ptr::copy(last_ptr, dropped_ptr, 1);
    }

    #[inline]
    pub fn delete_dyn(&mut self, entity: Entity) {
        unsafe {
            (self.vtable.delete)(self, entity);
        }
    }

    #[inline]
    #[must_use]
    pub unsafe fn get<T>(&self, entity: Entity) -> Option<&T>
    where
        T: Component,
    {
        let dense = self.sparse.get(entity)?.dense();
        Some(&*self.components.cast::<T>().as_ptr().add(dense))
    }

    #[inline]
    #[must_use]
    pub unsafe fn get_mut<T>(&self, entity: Entity) -> Option<&mut T>
    where
        T: Component,
    {
        let dense = self.sparse.get(entity)?.dense();
        Some(&mut *self.components.cast::<T>().as_ptr().add(dense))
    }

    #[inline]
    #[must_use]
    pub fn contains(&self, entity: Entity) -> bool {
        self.sparse.contains(entity)
    }

    #[inline]
    #[must_use]
    pub fn sparse(&self) -> &SparseVec {
        &self.sparse
    }

    #[inline]
    #[must_use]
    pub fn entities(&self) -> &[Entity] {
        unsafe { slice::from_raw_parts(self.entities.as_ptr(), self.len) }
    }

    #[inline]
    #[must_use]
    pub unsafe fn as_slice<T>(&self) -> &[T]
    where
        T: Component,
    {
        slice::from_raw_parts(self.components.cast::<T>().as_ptr(), self.len)
    }

    #[inline]
    #[must_use]
    pub unsafe fn as_mut_slice<T>(&mut self) -> &mut [T]
    where
        T: Component,
    {
        slice::from_raw_parts_mut(self.components.cast::<T>().as_ptr(), self.len)
    }

    #[inline]
    #[must_use]
    pub unsafe fn split<T>(&self) -> (&[Entity], &SparseVec, &[T])
    where
        T: Component,
    {
        (
            slice::from_raw_parts(self.entities.as_ptr(), self.len),
            &self.sparse,
            slice::from_raw_parts(self.components.cast::<T>().as_ptr(), self.len),
        )
    }

    #[inline]
    #[must_use]
    pub unsafe fn split_mut<T>(&mut self) -> (&[Entity], &SparseVec, &mut [T])
    where
        T: Component,
    {
        (
            slice::from_raw_parts(self.entities.as_ptr(), self.len),
            &self.sparse,
            slice::from_raw_parts_mut(self.components.cast::<T>().as_ptr(), self.len),
        )
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub unsafe fn swap(&mut self, a: usize, b: usize) {
        unsafe {
            (self.vtable.swap)(self, a, b);
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        unsafe {
            (self.vtable.clear)(self);
        }
    }

    #[inline]
    fn grow(&mut self) {
        unsafe {
            (self.vtable.grow)(self);
        }
    }

    unsafe fn grow_typed<T>(&mut self)
    where
        T: Component,
    {
        let (new_entities, new_components, new_cap) = {
            let new_cap = match self.cap {
                0 => 4,
                cap => cap.saturating_add(cap),
            };

            assert_ne!(
                new_cap, self.cap,
                "Cannot allocate space for more components",
            );

            let (new_layout, new_components_offset) =
                Self::compute_layout::<T>(new_cap).expect("Failed to compute new component layout");

            let new_data = alloc(new_layout);

            if new_data.is_null() {
                handle_alloc_error(new_layout);
            }

            #[allow(clippy::cast_ptr_alignment)]
            (
                NonNull::new_unchecked(new_data.cast::<Entity>()),
                NonNull::new_unchecked(new_data.byte_add(new_components_offset)),
                new_cap,
            )
        };

        ptr::copy_nonoverlapping(self.entities.as_ptr(), new_entities.as_ptr(), self.len);

        ptr::copy_nonoverlapping::<T>(
            self.components.cast().as_ptr(),
            new_components.cast().as_ptr(),
            self.len,
        );

        if self.cap != 0 {
            let (layout, _) = Self::compute_layout::<T>(self.cap).unwrap();
            dealloc(self.entities.cast().as_ptr(), layout);
        }

        self.entities = new_entities;
        self.components = new_components;
        self.cap = new_cap;
    }

    unsafe fn swap_typed<T>(&mut self, a: usize, b: usize)
    where
        T: Component,
    {
        debug_assert!(a < self.len);
        debug_assert!(b < self.len);
        debug_assert_ne!(a, b);

        let (sparse_a, sparse_b) = {
            let entity_a = &mut *self.entities.as_ptr().add(a);
            let entity_b = &mut *self.entities.as_ptr().add(b);
            mem::swap(entity_a, entity_b);

            (entity_a.sparse(), entity_b.sparse())
        };

        self.sparse.swap(sparse_a, sparse_b);

        let component_a = &mut *self.components.cast::<T>().as_ptr().add(a);
        let component_b = &mut *self.components.cast::<T>().as_ptr().add(b);
        mem::swap(component_a, component_b);
    }

    unsafe fn clear_typed<T>(&mut self)
    where
        T: Component,
    {
        self.sparse.clear();

        if mem::needs_drop::<T>() {
            for i in 0..self.len {
                unsafe {
                    self.components.cast::<T>().as_ptr().add(i).drop_in_place();
                }
            }
        }

        self.len = 0;
    }

    unsafe fn drop_typed<T>(&mut self)
    where
        T: Component,
    {
        if mem::needs_drop::<T>() {
            for i in 0..self.len {
                unsafe {
                    self.components.cast::<T>().as_ptr().add(i).drop_in_place();
                }
            }
        }

        if self.cap != 0 {
            let (layout, _) = Self::compute_layout::<T>(self.cap).unwrap();
            dealloc(self.entities.cast().as_ptr(), layout);
        }
    }

    fn compute_layout<T>(cap: usize) -> Result<(Layout, usize), LayoutError> {
        let entities_layout = Layout::array::<Entity>(cap)?;
        let components_layout = Layout::array::<T>(cap)?;
        entities_layout.extend(components_layout)
    }
}

unsafe impl Send for ComponentSparseSet {
    // Empty
}

unsafe impl Sync for ComponentSparseSet {
    // Empty
}

impl Drop for ComponentSparseSet {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            (self.vtable.drop)(self);
        }
    }
}

impl fmt::Debug for ComponentSparseSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(ComponentSparseSet))
            .field("entities", &self.entities())
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Copy, Debug)]
struct ComponentSparseSetVtable {
    grow: unsafe fn(&mut ComponentSparseSet),
    swap: unsafe fn(&mut ComponentSparseSet, usize, usize),
    delete: unsafe fn(&mut ComponentSparseSet, Entity),
    clear: unsafe fn(&mut ComponentSparseSet),
    drop: unsafe fn(&mut ComponentSparseSet),
}

impl ComponentSparseSetVtable {
    const fn new<T>() -> Self
    where
        T: Component,
    {
        Self {
            grow: ComponentSparseSet::grow_typed::<T>,
            swap: ComponentSparseSet::swap_typed::<T>,
            delete: ComponentSparseSet::delete::<T>,
            clear: ComponentSparseSet::clear_typed::<T>,
            drop: ComponentSparseSet::drop_typed::<T>,
        }
    }
}
