use crate::data::TypeInfo;
use std::alloc::{alloc, dealloc, realloc, Layout};
use std::any::TypeId;
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::ptr::NonNull;
use std::slice;

pub struct TypeErasedVec {
    type_info: TypeInfo,
    ptr: NonNull<u8>,
    cap: usize,
    len: usize,
}

unsafe impl Send for TypeErasedVec {}
unsafe impl Sync for TypeErasedVec {}

impl TypeErasedVec {
    pub fn new<T>() -> Self
    where
        T: Send + Sync + 'static,
    {
        let ptr = unsafe { NonNull::new_unchecked(NonNull::<T>::dangling().as_ptr() as _) };
        let cap = if mem::size_of::<T>() == 0 { !0 } else { 0 };

        Self {
            type_info: TypeInfo::new::<T>(),
            ptr,
            cap,
            len: 0,
        }
    }

    pub fn type_info(&self) -> &TypeInfo {
        &self.type_info
    }

    pub fn clear(&mut self) {
        if self.type_info.needs_drop() {
            unsafe {
                self.type_info.drop_slice(self.ptr.as_ptr(), self.len);
            }
        }

        self.len = 0;
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        assert!(a < self.len && b < self.len, "Index out of range");

        if a != b {
            unsafe {
                ptr::swap_nonoverlapping(
                    self.ptr.as_ptr().add(a * self.type_info.size()),
                    self.ptr.as_ptr().add(b * self.type_info.size()),
                    self.type_info.size(),
                );
            }
        }
    }

    pub fn swap_delete(&mut self, index: usize) {
        self.swap(index, self.len - 1);
        self.len -= 1;

        if self.type_info.needs_drop() {
            unsafe {
                self.type_info
                    .drop_one(self.ptr.as_ptr().add(self.len * self.type_info.size()));
            }
        }
    }

    pub fn as_ref<T>(&self) -> VecRef<T>
    where
        T: Send + Sync + 'static,
    {
        assert!(self.type_info.id() == TypeId::of::<T>());

        VecRef {
            vec: self,
            _phantom: PhantomData,
        }
    }

    pub fn as_mut<T>(&mut self) -> VecRefMut<T>
    where
        T: Send + Sync + 'static,
    {
        assert!(self.type_info.id() == TypeId::of::<T>());

        VecRefMut {
            vec: self,
            _phantom: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }
}

impl Drop for TypeErasedVec {
    fn drop(&mut self) {
        if self.cap != 0 {
            if self.type_info.needs_drop() {
                unsafe {
                    self.type_info.drop_slice(self.ptr.as_ptr(), self.len);
                }
            }

            if self.type_info.size() != 0 {
                unsafe {
                    dealloc(self.ptr.as_ptr(), self.type_info.layout());
                }
            }
        }
    }
}

pub struct VecRef<'a, T>
where
    T: Send + Sync + 'static,
{
    vec: &'a TypeErasedVec,
    _phantom: PhantomData<T>,
}

unsafe impl<T> Send for VecRef<'_, T> where T: Send + Sync + 'static {}
unsafe impl<T> Sync for VecRef<'_, T> where T: Send + Sync + 'static {}

impl<T> VecRef<'_, T>
where
    T: Send + Sync + 'static,
{
    fn as_ptr(&self) -> *const T {
        self.vec.ptr.as_ptr() as _
    }
}

impl<T> AsRef<[T]> for VecRef<'_, T>
where
    T: Send + Sync + 'static,
{
    fn as_ref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.vec.len) }
    }
}

impl<T> Deref for VecRef<'_, T>
where
    T: Send + Sync + 'static,
{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

pub struct VecRefMut<'a, T>
where
    T: Send + Sync + 'static,
{
    vec: &'a mut TypeErasedVec,
    _phantom: PhantomData<T>,
}

unsafe impl<T> Send for VecRefMut<'_, T> where T: Send + Sync + 'static {}
unsafe impl<T> Sync for VecRefMut<'_, T> where T: Send + Sync + 'static {}

impl<T> VecRefMut<'_, T>
where
    T: Send + Sync + 'static,
{
    pub fn push(&mut self, elem: T) {
        if self.vec.len == self.vec.cap {
            self.grow();
        }

        unsafe {
            ptr::write(self.as_mut_ptr().add(self.vec.len), elem);
        }

        self.vec.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.vec.len == 0 {
            None
        } else {
            self.vec.len -= 1;

            unsafe { Some(ptr::read(self.as_ptr() as _)) }
        }
    }

    pub fn swap_remove(&mut self, index: usize) -> T {
        assert!(index < self.vec.len, "Index out of range");
        self.vec.len -= 1;

        unsafe {
            let last = ptr::read(self.as_ptr().add(self.vec.len));
            let hole = self.as_mut_ptr().add(index);
            ptr::replace(hole, last)
        }
    }

    fn grow(&mut self) {
        assert!(mem::size_of::<T>() != 0, "Vec is overfull");

        unsafe {
            let (new_cap, ptr) = if self.vec.cap == 0 {
                (1, alloc(Layout::new::<T>()))
            } else {
                let old_num_bytes = self.vec.cap * mem::size_of::<T>();
                assert!(
                    old_num_bytes <= (isize::MAX as usize) / 2,
                    "Capacity overflow",
                );

                let new_cap = self.vec.cap * 2;
                let new_num_bytes = old_num_bytes * 2;

                let ptr = realloc(self.as_mut_ptr() as _, Layout::new::<T>(), new_num_bytes);
                (new_cap, ptr)
            };

            if ptr.is_null() {
                panic!("Out of memory");
            }

            self.vec.ptr = NonNull::new_unchecked(ptr);
            self.vec.cap = new_cap;
        }
    }

    fn as_ptr(&self) -> *const T {
        self.vec.ptr.as_ptr() as _
    }

    fn as_mut_ptr(&mut self) -> *mut T {
        self.vec.ptr.as_ptr() as _
    }
}

impl<T> AsRef<[T]> for VecRefMut<'_, T>
where
    T: Send + Sync + 'static,
{
    fn as_ref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.vec.len) }
    }
}

impl<T> AsMut<[T]> for VecRefMut<'_, T>
where
    T: Send + Sync + 'static,
{
    fn as_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.vec.len) }
    }
}

impl<T> Deref for VecRefMut<'_, T>
where
    T: Send + Sync + 'static,
{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for VecRefMut<'_, T>
where
    T: Send + Sync + 'static,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
