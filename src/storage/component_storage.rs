use crate::storage::{Entity, IndexEntity, SparseArray, SparseArrayView};
use crate::utils::ChangeTicks;
use std::alloc::{alloc, dealloc, handle_alloc_error, realloc, Layout};
use std::ptr::NonNull;
use std::{mem, ptr, slice};

pub struct ComponentStorage {
    layout: Layout,
    sparse: SparseArray,
    entities: NonNull<Entity>,
    components: NonNull<u8>,
    ticks: NonNull<ChangeTicks>,
    cap: usize,
    len: usize,
    swap_space: NonNull<u8>,
    drop: unsafe fn(*mut u8),
    needs_drop: bool,
}

impl ComponentStorage {
    pub(crate) fn new<T>() -> Self
    where
        T: 'static,
    {
        unsafe {
            if mem::needs_drop::<T>() {
                let drop = |ptr: *mut u8| ptr::drop_in_place(ptr as *mut T);
                Self::from_layout_drop(Layout::new::<T>(), Some(drop))
            } else {
                Self::from_layout_drop(Layout::new::<T>(), None)
            }
        }
    }

    pub(crate) unsafe fn from_layout_drop(
        layout: Layout,
        drop: Option<unsafe fn(*mut u8)>,
    ) -> Self {
        let swap_space = if layout.size() != 0 {
            NonNull::new(alloc(layout)).unwrap_or_else(|| handle_alloc_error(layout))
        } else {
            NonNull::new_unchecked(layout.align() as _)
        };

        Self {
            layout,
            sparse: SparseArray::default(),
            entities: NonNull::dangling(),
            components: NonNull::new_unchecked(layout.align() as _),
            ticks: NonNull::dangling(),
            cap: 0,
            len: 0,
            swap_space,
            drop: drop.unwrap_or(|_| ()),
            needs_drop: drop.is_some(),
        }
    }

    #[must_use]
    pub(crate) unsafe fn insert_and_forget_prev(
        &mut self,
        entity: Entity,
        component: *const u8,
        ticks: ChangeTicks,
    ) -> Option<NonNull<u8>> {
        debug_assert!(!component.is_null());

        let index_entity = self.sparse.get_mut_or_allocate_at(entity.index());

        match index_entity {
            Some(index_entity) => {
                let index = index_entity.index();
                *self.entities.as_ptr().add(index) = entity;

                let size = self.layout.size();
                let to_remove = self.components.as_ptr().add(index * size);
                let swap_space = self.swap_space.as_ptr();

                ptr::copy_nonoverlapping(to_remove, swap_space, size);
                ptr::copy_nonoverlapping(component, to_remove, size);

                *self.ticks.as_ptr().add(index) = ticks;

                Some(self.swap_space)
            }
            None => {
                *index_entity = Some(IndexEntity::new(self.len as u32, entity.version()));

                if self.len == self.cap {
                    self.grow_amortized();
                }

                let size = self.layout.size();
                let slot = self.components.as_ptr().add(self.len * size);

                *self.entities.as_ptr().add(self.len) = entity;
                ptr::copy_nonoverlapping(component, slot, size);
                *self.ticks.as_ptr().add(self.len) = ticks;

                self.len += 1;
                None
            }
        }
    }

    #[allow(dead_code)]
    pub(crate) unsafe fn insert_and_drop_prev(
        &mut self,
        entity: Entity,
        component: *const u8,
        ticks: ChangeTicks,
    ) {
        debug_assert!(!component.is_null());

        let index_entity = self.sparse.get_mut_or_allocate_at(entity.index());

        match index_entity {
            Some(index_entity) => {
                let index = index_entity.index();
                *self.entities.as_ptr().add(index) = entity;

                let size = self.layout.size();
                let to_remove = self.components.as_ptr().add(index * size);

                (self.drop)(to_remove);
                ptr::copy_nonoverlapping(component, to_remove, size);

                *self.ticks.as_ptr().add(index) = ticks;
            }
            None => {
                *index_entity = Some(IndexEntity::new(self.len as u32, entity.version()));

                if self.len == self.cap {
                    self.grow_amortized();
                }

                *self.entities.as_ptr().add(self.len) = entity;

                let size = self.layout.size();
                let slot = self.components.as_ptr().add(self.len * size);
                ptr::copy_nonoverlapping(component, slot, size);

                *self.ticks.as_ptr().add(self.len) = ticks;

                self.len += 1;
            }
        }
    }

    #[must_use]
    pub(crate) fn remove_and_forget(&mut self, entity: Entity) -> Option<NonNull<u8>> {
        let index = self.sparse.remove(entity)?;

        self.len -= 1;

        unsafe {
            let last_entity = *self.entities.as_ptr().add(self.len);
            *self.entities.as_ptr().add(index) = last_entity;

            if index < self.len {
                let index_entity = IndexEntity::new(index as u32, last_entity.version());
                *self.sparse.get_unchecked_mut(last_entity.index()) = Some(index_entity);
            }

            let size = self.layout.size();
            let to_remove = self.components.as_ptr().add(index * size);
            let last = self.components.as_ptr().add(self.len * size);
            let swap_space = self.swap_space.as_ptr();

            ptr::copy_nonoverlapping(to_remove, swap_space, size);
            ptr::copy(last, to_remove, size);

            *self.ticks.as_ptr().add(index) = *self.ticks.as_ptr().add(self.len);
        }

        Some(self.swap_space)
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
            let to_remove = self.components.as_ptr().add(index * size);
            let last = self.components.as_ptr().add(self.len * size);

            (self.drop)(to_remove);
            ptr::copy(last, to_remove, size);

            *self.ticks.as_ptr().add(index) = *self.ticks.as_ptr().add(self.len);
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
        let component_a = self.components.as_ptr().add(a * size);
        let component_b = self.components.as_ptr().add(b * size);
        let swap_space = self.swap_space.as_ptr();

        ptr::copy_nonoverlapping(component_a, swap_space, size);
        ptr::copy(component_b, component_a, size);
        ptr::copy_nonoverlapping(swap_space, component_b, size);

        ptr::swap(self.ticks.as_ptr().add(a), self.ticks.as_ptr().add(b));
    }

    pub fn get(&self, entity: Entity) -> Option<NonNull<u8>> {
        let index = self.sparse.get_index(entity)?;

        unsafe {
            let component = self.components.as_ptr().add(index * self.layout.size());
            Some(NonNull::new_unchecked(component))
        }
    }

    pub fn get_ticks(&self, entity: Entity) -> Option<&ChangeTicks> {
        let index = self.sparse.get_index(entity)?;
        unsafe { Some(&*self.ticks.as_ptr().add(index)) }
    }

    pub fn get_with_ticks(&self, entity: Entity) -> Option<(NonNull<u8>, &ChangeTicks)> {
        let index = self.sparse.get_index(entity)?;

        unsafe {
            let component = self.components.as_ptr().add(index * self.layout.size());
            let ticks = self.ticks.as_ptr().add(index);
            Some((NonNull::new_unchecked(component), &*ticks))
        }
    }

    pub(crate) fn get_with_ticks_mut(
        &mut self,
        entity: Entity,
    ) -> Option<(NonNull<u8>, &mut ChangeTicks)> {
        let index = self.sparse.get_index(entity)?;

        unsafe {
            let component = self.components.as_ptr().add(index * self.layout.size());
            let ticks = self.ticks.as_ptr().add(index);
            Some((NonNull::new_unchecked(component), &mut *ticks))
        }
    }

    #[inline]
    pub(crate) fn get_index(&self, entity: Entity) -> Option<usize> {
        self.sparse.get_index(entity)
    }

    #[inline]
    pub fn contains(&self, entity: Entity) -> bool {
        self.sparse.contains(entity)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    pub fn entities(&self) -> &[Entity] {
        unsafe { slice::from_raw_parts(self.entities.as_ptr(), self.len) }
    }

    pub fn components(&self) -> *const u8 {
        self.components.as_ptr()
    }

    pub fn ticks(&self) -> &[ChangeTicks] {
        unsafe { slice::from_raw_parts(self.ticks.as_ptr(), self.len) }
    }

    pub(crate) fn split_for_iteration(
        &self,
    ) -> (SparseArrayView, &[Entity], *const u8, *const ChangeTicks) {
        (
            self.sparse.as_view(),
            unsafe { slice::from_raw_parts(self.entities.as_ptr(), self.len) },
            self.components.as_ptr(),
            self.ticks.as_ptr(),
        )
    }

    pub(crate) fn split_for_iteration_mut(
        &mut self,
    ) -> (SparseArrayView, &[Entity], *mut u8, *mut ChangeTicks) {
        (
            self.sparse.as_view(),
            unsafe { slice::from_raw_parts(self.entities.as_ptr(), self.len) },
            self.components.as_ptr(),
            self.ticks.as_ptr(),
        )
    }

    pub(crate) fn clear(&mut self) {
        if self.needs_drop {
            let len = self.len;
            self.len = 0;

            for i in 0..len {
                unsafe {
                    (self.drop)(self.components.as_ptr().add(i * self.layout.size()));
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
                    self.components
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

                    NonNull::new(realloc(self.components.as_ptr(), old_layout, layout.size()))
                        .unwrap_or_else(|| handle_alloc_error(layout))
                } else {
                    self.components
                };

                let ticks = {
                    let old_layout = array_layout::<ChangeTicks>(self.cap);
                    let layout = array_layout::<ChangeTicks>(cap);

                    NonNull::new(realloc(
                        self.ticks.as_ptr().cast(),
                        old_layout,
                        layout.size(),
                    ))
                    .unwrap_or_else(|| handle_alloc_error(layout))
                };

                (entities, components, ticks, cap)
            };

            self.entities = entities.cast();
            self.components = components;
            self.ticks = ticks.cast();
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
                        self.components.as_ptr(),
                        repeat_layout(&self.layout, self.cap),
                    );
                }

                dealloc(
                    self.ticks.as_ptr().cast(),
                    array_layout::<ChangeTicks>(self.cap),
                );
            }
        }
    }
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
