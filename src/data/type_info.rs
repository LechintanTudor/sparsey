use std::alloc::Layout;
use std::any::TypeId;
use std::{any, mem, ptr};

/// Holds various type information, like the `TypeId`,
/// the `Layout` and destructor functions for a single
/// element or a slice of elements of type `T`.
pub struct TypeInfo {
    id: TypeId,
    name: &'static str,
    layout: Layout,
    needs_drop: bool,
    drop_one_fn: unsafe fn(*mut u8),
    drop_slice_fn: unsafe fn(*mut u8, usize),
}

impl TypeInfo {
    /// Create a new `TypeInfo` for type `T`.
    pub fn new<T>() -> Self
    where
        T: 'static,
    {
        Self {
            id: TypeId::of::<T>(),
            name: any::type_name::<T>(),
            layout: Layout::new::<T>(),
            needs_drop: mem::needs_drop::<T>(),
            drop_one_fn: drop_one::<T>,
            drop_slice_fn: drop_slice::<T>,
        }
    }

    /// Get the if of the type.
    pub fn id(&self) -> TypeId {
        self.id
    }

    /// Get the name of the type.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Get the `Layout` of the type.
    pub fn layout(&self) -> Layout {
        self.layout
    }

    /// Get the size in bytes of the type.
    pub fn size(&self) -> usize {
        self.layout.size()
    }

    /// Get the align in bytes of the type.
    pub fn align(&self) -> usize {
        self.layout.align()
    }

    /// Get whether or not the type has a destructor.
    pub fn needs_drop(&self) -> bool {
        self.needs_drop
    }

    /// Drop the value pointed to by `ptr`.
    pub unsafe fn drop_one(&self, ptr: *mut u8) {
        (self.drop_one_fn)(ptr);
    }

    /// Drop `count` values starting from `ptr`.
    pub unsafe fn drop_slice(&self, ptr: *mut u8, count: usize) {
        (self.drop_slice_fn)(ptr, count);
    }
}

unsafe fn drop_one<T>(ptr: *mut u8) {
    ptr::drop_in_place(ptr as *mut T);
}

unsafe fn drop_slice<T>(ptr: *mut u8, count: usize) {
    let mut ptr = ptr as *mut T;

    for _ in 0..count {
        ptr::drop_in_place(ptr);
        ptr = ptr.offset(1);
    }
}
