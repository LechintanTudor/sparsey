use std::alloc::Layout;
use std::any;
use std::any::TypeId;
use std::mem;
use std::ptr;

pub struct TypeInfo {
    id: TypeId,
    name: &'static str,
    layout: Layout,
    needs_drop: bool,
    drop_one_fn: unsafe fn(*mut u8),
    drop_slice_fn: unsafe fn(*mut u8, usize),
}

impl TypeInfo {
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

    pub fn id(&self) -> TypeId {
        self.id
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn layout(&self) -> Layout {
        self.layout
    }

    pub fn size(&self) -> usize {
        self.layout.size()
    }

    pub fn align(&self) -> usize {
        self.layout.align()
    }

    pub fn needs_drop(&self) -> bool {
        self.needs_drop
    }

    pub unsafe fn drop_one(&self, ptr: *mut u8) {
        (self.drop_one_fn)(ptr);
    }

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
