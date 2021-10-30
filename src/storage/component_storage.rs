use crate::storage::{Entity, EntitySparseArray, IndexEntity};
use crate::utils::ChangeTicks;
use std::alloc::{alloc, dealloc, handle_alloc_error, realloc, Layout};
use std::ptr::NonNull;
use std::{mem, ptr, slice};

pub struct ComponentStorageData {
    pub(crate) components: NonNull<u8>,
    pub(crate) ticks: NonNull<ChangeTicks>,
}

impl ComponentStorageData {
    fn from_layout(layout: &Layout) -> Self {
        let components = unsafe { NonNull::new_unchecked(layout.align() as _) };

        Self {
            components,
            ticks: NonNull::dangling(),
        }
    }

    #[inline]
    pub(crate) unsafe fn get_unchecked<T>(&self, index: usize) -> &T {
        &*self.components.cast::<T>().as_ptr().add(index)
    }

    #[inline]
    pub(crate) unsafe fn get_ticks_unchecked(&self, index: usize) -> &ChangeTicks {
        &*self.ticks.as_ptr().add(index)
    }

    #[inline]
    pub(crate) unsafe fn get_with_ticks_unchecked<T>(&self, index: usize) -> (&T, &ChangeTicks) {
        (
            &*self.components.cast::<T>().as_ptr().add(index),
            &*self.ticks.as_ptr().add(index),
        )
    }

    #[inline]
    pub(crate) unsafe fn get_with_ticks_unchecked_mut<T>(
        &mut self,
        index: usize,
    ) -> (&mut T, &mut ChangeTicks) {
        (
            &mut *self.components.cast::<T>().as_ptr().add(index),
            &mut *self.ticks.as_ptr().add(index),
        )
    }
}

/// Type-erased storage for `Component`s.
pub struct ComponentStorage {
    entities: NonNull<Entity>,
    len: usize,
    sparse: EntitySparseArray,
    data: ComponentStorageData,
    layout: Layout,
    swap_space: NonNull<u8>,
    cap: usize,
    drop: unsafe fn(*mut u8),
    needs_drop: bool,
}

impl ComponentStorage {
    pub(crate) fn new<T>() -> Self
    where
        T: 'static,
    {
        let layout = Layout::new::<T>();

        let swap_space = unsafe {
            if layout.size() != 0 {
                NonNull::new(alloc(layout)).unwrap_or_else(|| handle_alloc_error(layout))
            } else {
                NonNull::new_unchecked(layout.align() as _)
            }
        };

        Self {
            entities: NonNull::dangling(),
            len: 0,
            sparse: EntitySparseArray::default(),
            data: ComponentStorageData::from_layout(&layout),
            cap: 0,
            layout,
            swap_space,
            drop: drop_in_place::<T>,
            needs_drop: mem::needs_drop::<T>(),
        }
    }

    pub(crate) unsafe fn insert<T>(
        &mut self,
        entity: Entity,
        component: T,
        ticks: ChangeTicks,
    ) -> Option<T>
    where
        T: 'static,
    {
        let index_entity = self.sparse.get_mut_or_allocate_at(entity.index());

        match index_entity {
            Some(index_entity) => {
                let index = index_entity.index();
                *self.entities.as_ptr().add(index) = entity;
                *self.data.ticks.as_ptr().add(index) = ticks;
                Some(
                    self.data
                        .components
                        .cast::<T>()
                        .as_ptr()
                        .add(index)
                        .replace(component),
                )
            }
            None => {
                *index_entity = Some(IndexEntity::new(self.len as u32, entity.version()));

                if self.len == self.cap {
                    self.grow_amortized();
                }

                *self.entities.as_ptr().add(self.len) = entity;
                *self.data.components.cast::<T>().as_ptr().add(self.len) = component;
                *self.data.ticks.as_ptr().add(self.len) = ticks;

                self.len += 1;
                None
            }
        }
    }

    pub(crate) unsafe fn remove<T>(&mut self, entity: Entity) -> Option<T>
    where
        T: 'static,
    {
        let index = self.sparse.remove(entity)?;

        self.len -= 1;

        let last_entity = *self.entities.as_ptr().add(self.len);
        *self.entities.as_ptr().add(index) = last_entity;

        if index < self.len {
            let index_entity = IndexEntity::new(index as u32, last_entity.version());
            *self.sparse.get_unchecked_mut(last_entity.index()) = Some(index_entity);
        }

        *self.data.ticks.as_ptr().add(index) = *self.data.ticks.as_ptr().add(self.len);

        let components = self.data.components.cast::<T>().as_ptr();
        let removed = components.add(index).read();
        ptr::copy(components.add(self.len), components.add(index), 1);
        Some(removed)
    }

    pub(crate) fn remove_and_drop(&mut self, entity: Entity) {
        let index = match self.sparse.remove(entity) {
            Some(index) => index,
            None => return,
        };

        self.len -= 1;

        unsafe {
            let last_entity = *self.entities.as_ptr().add(self.len);
            *self.entities.as_ptr().add(index) = last_entity;

            if index < self.len {
                let index_entity = IndexEntity::new(index as u32, last_entity.version());
                *self.sparse.get_unchecked_mut(last_entity.index()) = Some(index_entity);
            }

            let size = self.layout.size();
            let to_remove = self.data.components.as_ptr().add(index * size);
            let last = self.data.components.as_ptr().add(self.len * size);

            (self.drop)(to_remove);
            ptr::copy(last, to_remove, size);

            *self.data.ticks.as_ptr().add(index) = *self.data.ticks.as_ptr().add(self.len);
        }
    }

    pub(crate) unsafe fn swap_unchecked(&mut self, a: usize, b: usize) {
        debug_assert!(a < self.len);
        debug_assert!(b < self.len);

        let entity_a = self.entities.as_ptr().add(a);
        let entity_b = self.entities.as_ptr().add(b);
        ptr::swap(entity_a, entity_b);

        let sparse_a = (*entity_a).index();
        let sparse_b = (*entity_b).index();
        self.sparse.swap_unchecked(sparse_a, sparse_b);

        let size = self.layout.size();
        let component_a = self.data.components.as_ptr().add(a * size);
        let component_b = self.data.components.as_ptr().add(b * size);
        let swap_space = self.swap_space.as_ptr();

        ptr::copy_nonoverlapping(component_a, swap_space, size);
        ptr::copy(component_b, component_a, size);
        ptr::copy_nonoverlapping(swap_space, component_b, size);

        ptr::swap(
            self.data.ticks.as_ptr().add(a),
            self.data.ticks.as_ptr().add(b),
        );
    }

    pub(crate) unsafe fn get<T>(&self, entity: Entity) -> Option<&T> {
        let index = self.sparse.get_index(entity)?;
        Some(self.data.get_unchecked::<T>(index))
    }

    pub(crate) fn get_ticks(&self, entity: Entity) -> Option<&ChangeTicks> {
        let index = self.sparse.get_index(entity)?;
        unsafe { Some(self.data.get_ticks_unchecked(index)) }
    }

    pub(crate) unsafe fn get_with_ticks<T>(&self, entity: Entity) -> Option<(&T, &ChangeTicks)> {
        let index = self.sparse.get_index(entity)?;
        Some(self.data.get_with_ticks_unchecked(index))
    }

    pub(crate) unsafe fn get_with_ticks_mut<T>(
        &mut self,
        entity: Entity,
    ) -> Option<(&mut T, &mut ChangeTicks)> {
        let index = self.sparse.get_index(entity)?;
        Some(self.data.get_with_ticks_unchecked_mut::<T>(index))
    }

    pub(crate) fn get_index(&self, entity: Entity) -> Option<usize> {
        self.sparse.get_index(entity)
    }

    pub(crate) fn contains(&self, entity: Entity) -> bool {
        self.sparse.contains(entity)
    }

    pub(crate) fn len(&self) -> usize {
        self.len
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub(crate) fn capacity(&self) -> usize {
        self.cap
    }

    pub(crate) fn entities(&self) -> &[Entity] {
        unsafe { slice::from_raw_parts(self.entities.as_ptr(), self.len) }
    }

    pub(crate) unsafe fn components<T>(&self) -> &[T]
    where
        T: 'static,
    {
        slice::from_raw_parts(self.data.components.cast::<T>().as_ptr(), self.len)
    }

    pub(crate) fn ticks(&self) -> &[ChangeTicks] {
        unsafe { slice::from_raw_parts(self.data.ticks.as_ptr(), self.len) }
    }

    pub(crate) fn split_for_iteration<T>(
        &self,
    ) -> (&EntitySparseArray, &[Entity], *const T, *const ChangeTicks)
    where
        T: 'static,
    {
        (
            &self.sparse,
            unsafe { slice::from_raw_parts(self.entities.as_ptr(), self.len) },
            self.data.components.cast::<T>().as_ptr(),
            self.data.ticks.as_ptr(),
        )
    }

    pub(crate) fn split_for_iteration_mut<T>(
        &mut self,
    ) -> (&EntitySparseArray, &[Entity], *mut T, *mut ChangeTicks)
    where
        T: 'static,
    {
        (
            &self.sparse,
            unsafe { slice::from_raw_parts(self.entities.as_ptr(), self.len) },
            self.data.components.cast::<T>().as_ptr(),
            self.data.ticks.as_ptr(),
        )
    }

    pub(crate) fn split(&self) -> (&[Entity], &EntitySparseArray, &ComponentStorageData) {
        (
            unsafe { slice::from_raw_parts(self.entities.as_ptr(), self.len) },
            &self.sparse,
            &self.data,
        )
    }

    pub(crate) unsafe fn split_mut(
        &mut self,
    ) -> (&[Entity], &EntitySparseArray, &mut ComponentStorageData) {
        (
            slice::from_raw_parts(self.entities.as_ptr(), self.len),
            &self.sparse,
            &mut self.data,
        )
    }

    pub(crate) fn clear(&mut self) {
        if self.needs_drop {
            let len = self.len;
            self.len = 0;

            for i in 0..len {
                unsafe {
                    (self.drop)(self.data.components.as_ptr().add(i * self.layout.size()));
                }
            }
        } else {
            self.len = 0;
        }
    }

    fn grow_amortized(&mut self) {
        unsafe {
            let (entities, components, ticks, cap) = if self.cap == 0 {
                let entities = NonNull::new(alloc(Layout::new::<Entity>()))
                    .unwrap_or_else(|| handle_alloc_error(Layout::new::<Entity>()));

                let components = if self.layout.size() != 0 {
                    NonNull::new(alloc(self.layout))
                        .unwrap_or_else(|| handle_alloc_error(self.layout))
                } else {
                    self.data.components
                };

                let ticks = NonNull::new(alloc(Layout::new::<ChangeTicks>()))
                    .unwrap_or_else(|| handle_alloc_error(Layout::new::<ChangeTicks>()));

                (entities, components, ticks, 1)
            } else {
                let cap = 2 * self.cap;

                let entities = {
                    let old_layout = array_layout::<Entity>(self.cap);
                    let layout = array_layout::<Entity>(cap);

                    NonNull::new(realloc(
                        self.entities.as_ptr().cast(),
                        old_layout,
                        layout.size(),
                    ))
                    .unwrap_or_else(|| handle_alloc_error(layout))
                };

                let components = if self.layout.size() != 0 {
                    let old_layout = repeat_layout(&self.layout, self.cap);
                    let layout = repeat_layout(&self.layout, cap);

                    NonNull::new(realloc(
                        self.data.components.as_ptr(),
                        old_layout,
                        layout.size(),
                    ))
                    .unwrap_or_else(|| handle_alloc_error(layout))
                } else {
                    self.data.components
                };

                let ticks = {
                    let old_layout = array_layout::<ChangeTicks>(self.cap);
                    let layout = array_layout::<ChangeTicks>(cap);

                    NonNull::new(realloc(
                        self.data.ticks.as_ptr().cast(),
                        old_layout,
                        layout.size(),
                    ))
                    .unwrap_or_else(|| handle_alloc_error(layout))
                };

                (entities, components, ticks, cap)
            };

            self.entities = entities.cast();
            self.data.components = components;
            self.data.ticks = ticks.cast();
            self.cap = cap;
        }
    }
}

impl Drop for ComponentStorage {
    fn drop(&mut self) {
        if self.layout.size() != 0 {
            unsafe {
                dealloc(self.swap_space.as_ptr(), self.layout);
            }
        }

        if self.cap != 0 {
            self.clear();

            unsafe {
                dealloc(
                    self.entities.as_ptr().cast(),
                    array_layout::<Entity>(self.cap),
                );

                if self.layout.size() != 0 {
                    dealloc(
                        self.data.components.as_ptr(),
                        repeat_layout(&self.layout, self.cap),
                    );
                }

                dealloc(
                    self.data.ticks.as_ptr().cast(),
                    array_layout::<ChangeTicks>(self.cap),
                );
            }
        }
    }
}

unsafe fn drop_in_place<T>(ptr: *mut u8) {
    ptr::drop_in_place(ptr.cast::<T>())
}

fn array_layout<T>(n: usize) -> Layout {
    Layout::array::<T>(n).expect("Layout size overflow")
}

// From https://doc.rust-lang.org/src/core/alloc/layout.rs.html
fn repeat_layout(layout: &Layout, n: usize) -> Layout {
    // This cannot overflow. Quoting from the invariant of Layout:
    // > `size`, when rounded up to the nearest multiple of `align`,
    // > must not overflow (i.e., the rounded value must be less than
    // > `usize::MAX`)
    let padded_size = layout.size() + padding_needed_for(layout, layout.align());
    let alloc_size = padded_size.checked_mul(n).expect("Layout size overflow");

    // SAFETY: self.align is already known to be valid and alloc_size has been
    // padded already.
    unsafe { Layout::from_size_align_unchecked(alloc_size, layout.align()) }
}

// From https://doc.rust-lang.org/src/core/alloc/layout.rs.html
fn padding_needed_for(layout: &Layout, align: usize) -> usize {
    let len = layout.size();

    // Rounded up value is:
    //   len_rounded_up = (len + align - 1) & !(align - 1);
    // and then we return the padding difference: `len_rounded_up - len`.
    //
    // We use modular arithmetic throughout:
    //
    // 1. align is guaranteed to be > 0, so align - 1 is always
    //    valid.
    //
    // 2. `len + align - 1` can overflow by at most `align - 1`,
    //    so the &-mask with `!(align - 1)` will ensure that in the
    //    case of overflow, `len_rounded_up` will itself be 0.
    //    Thus the returned padding, when added to `len`, yields 0,
    //    which trivially satisfies the alignment `align`.
    //
    // (Of course, attempts to allocate blocks of memory whose
    // size and padding overflow in the above manner should cause
    // the allocator to yield an error anyway.)

    let len_rounded_up = len.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1);
    len_rounded_up.wrapping_sub(len)
}
