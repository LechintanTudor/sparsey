use crate::component::Component;
use crate::entity::{DenseEntity, Entity, SparseVec};
use alloc::{alloc, Layout, LayoutError};
use core::ptr::NonNull;
use core::{fmt, mem, slice};

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

                // Replace existing entity and component.
                *self.entities.add(index).as_mut() = entity;
                Some(self.components.cast::<T>().add(index).replace(component))
            }
            None => {
                *dense_entity = Some(DenseEntity {
                    index: self.len as u32,
                    version: entity.version,
                });

                if self.len == self.cap {
                    self.grow();
                }

                // Write entity and component to uninitialized memory.
                self.entities.add(self.len).write(entity);
                self.components.cast::<T>().add(self.len).write(component);

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

        let last_entity = *self.entities.add(self.len).as_ref();
        *self.entities.add(index).as_mut() = last_entity;

        if index < self.len {
            *self.sparse.get_unchecked_mut(last_entity.sparse()) = Some(DenseEntity {
                index: index as u32,
                version: last_entity.version,
            });
        }

        let removed_ptr = self.components.cast::<T>().add(index);
        let last_ptr = self.components.cast::<T>().add(self.len);

        // Replace removed component with last component.
        let component = removed_ptr.read();
        last_ptr.copy_to(removed_ptr, 1);
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

        let last_entity = *self.entities.add(self.len).as_ref();
        *self.entities.add(index).as_mut() = last_entity;

        if index < self.len {
            *self.sparse.get_unchecked_mut(last_entity.sparse()) = Some(DenseEntity {
                index: index as u32,
                version: last_entity.version,
            });
        }

        let dropped_ptr = self.components.cast::<T>().add(index);
        dropped_ptr.drop_in_place();

        let last_ptr = self.components.cast::<T>().add(self.len);
        last_ptr.copy_to(dropped_ptr, 1);
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
        Some(self.components.cast::<T>().add(dense).as_ref())
    }

    #[inline]
    #[must_use]
    pub unsafe fn get_mut<T>(&self, entity: Entity) -> Option<&mut T>
    where
        T: Component,
    {
        let dense = self.sparse.get(entity)?.dense();
        Some(self.components.cast::<T>().add(dense).as_mut())
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
    pub unsafe fn as_non_null_ptr<T>(&self) -> NonNull<T>
    where
        T: Component,
    {
        self.components.cast::<T>()
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

    #[cold]
    #[inline(never)]
    unsafe fn grow_typed<T>(&mut self)
    where
        T: Component,
    {
        // Allocate new storage for entities and components.
        let (new_entities, new_components, new_cap) = {
            let new_cap = match self.cap {
                0 => 4,
                cap => {
                    let new_cap = cap.saturating_add(cap);
                    assert_ne!(new_cap, self.cap, "Cannot grow sparse set");
                    new_cap
                }
            };

            let (new_layout, new_components_offset) = Self::compute_layout::<T>(new_cap);

            let Some(new_data) = NonNull::new(alloc::alloc(new_layout)) else {
                alloc::handle_alloc_error(new_layout);
            };

            (
                new_data.cast::<Entity>(),
                new_data.byte_add(new_components_offset).cast::<T>(),
                new_cap,
            )
        };

        // Copy old entities and components to new location.
        self.entities.copy_to_nonoverlapping(new_entities, self.len);

        self.components
            .cast::<T>()
            .copy_to_nonoverlapping(new_components, self.len);

        // Deallocate old storage, if any.
        if self.cap != 0 {
            let (layout, _) = Self::compute_layout::<T>(self.cap);
            alloc::dealloc(self.entities.cast().as_ptr(), layout);
        }

        // Update pointers and capacity.
        self.entities = new_entities;
        self.components = new_components.cast();
        self.cap = new_cap;
    }

    unsafe fn swap_typed<T>(&mut self, dense_a: usize, dense_b: usize)
    where
        T: Component,
    {
        debug_assert!(dense_a < self.len);
        debug_assert!(dense_b < self.len);
        debug_assert_ne!(dense_a, dense_b);

        // Swap entities.
        let entity_a = self.entities.add(dense_a).as_mut();
        let entity_b = self.entities.add(dense_b).as_mut();
        self.sparse.swap(entity_a.sparse(), entity_b.sparse());
        mem::swap(entity_a, entity_b);

        // Swap components.
        let component_a = self.components.cast::<T>().add(dense_a).as_mut();
        let component_b = self.components.cast::<T>().add(dense_b).as_mut();
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
                    self.components.cast::<T>().add(i).drop_in_place();
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
                    self.components.cast::<T>().add(i).drop_in_place();
                }
            }
        }

        if self.cap != 0 {
            let (layout, _) = Self::compute_layout::<T>(self.cap);
            alloc::dealloc(self.entities.cast::<u8>().as_ptr(), layout);
        }
    }

    fn compute_layout<T>(cap: usize) -> (Layout, usize) {
        fn compute_layout_impl<T>(cap: usize) -> Result<(Layout, usize), LayoutError> {
            let entities_layout = Layout::array::<Entity>(cap)?;
            let components_layout = Layout::array::<T>(cap)?;
            entities_layout.extend(components_layout)
        }

        match compute_layout_impl::<T>(cap) {
            Ok(result) => result,
            Err(e) => panic!("Cannot compute sparse set data layout: {e}"),
        }
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

#[derive(Clone, Copy)]
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
