use crate::data::{
    Component, ComponentFlags, ComponentRefMut, Entity, MappedAtomicRef, MappedAtomicRefMut,
    SparseArray, SparseSetRef, SparseSetRefMut,
};

pub unsafe trait ComponentView<'a>
where
    Self: Sized,
{
    type Flags: 'a + Copy;
    type Data: 'a + Copy;
    type Item: 'a;

    fn group_len_ref(&self) -> Option<&usize>;

    fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Flags, Self::Data);

    fn get(self, entity: Entity) -> Option<Self::Item> {
        let (sparse, _, flags, data) = self.split();
        let index = sparse.get_index_entity(entity)?.index();

        unsafe { Self::get_item(flags, data, index) }
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags;

    unsafe fn get_item(flags: Self::Flags, data: Self::Data, index: usize) -> Option<Self::Item>;
}

pub struct Comp<'a, T>
where
    T: Component,
{
    sparse_set: MappedAtomicRef<'a, SparseSetRef<'a, T>>,
    group_len_ref: Option<&'a usize>,
}

impl<'a, T> Comp<'a, T>
where
    T: Component,
{
    pub(crate) unsafe fn new(
        sparse_set: MappedAtomicRef<'a, SparseSetRef<'a, T>>,
        group_len_ref: Option<&'a usize>,
    ) -> Self {
        Self {
            sparse_set,
            group_len_ref,
        }
    }
}

unsafe impl<'a, T> ComponentView<'a> for &'a Comp<'a, T>
where
    T: Component,
{
    type Flags = *const ComponentFlags;
    type Data = *const T;
    type Item = &'a T;

    fn group_len_ref(&self) -> Option<&usize> {
        self.group_len_ref
    }

    fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Flags, Self::Data) {
        let (sparse, dense, flags, data) = self.sparse_set.split();
        (sparse, dense, flags.as_ptr(), data.as_ptr())
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags {
        *flags.add(index)
    }

    unsafe fn get_item(_: Self::Flags, data: Self::Data, index: usize) -> Option<Self::Item> {
        Some(&*data.add(index))
    }
}

pub struct CompMut<'a, T>
where
    T: Component,
{
    sparse_set: MappedAtomicRefMut<'a, SparseSetRefMut<'a, T>>,
    group_len_ref: Option<&'a usize>,
}

impl<'a, T> CompMut<'a, T>
where
    T: Component,
{
    pub(crate) unsafe fn new(
        sparse_set: MappedAtomicRefMut<'a, SparseSetRefMut<'a, T>>,
        group_len_ref: Option<&'a usize>,
    ) -> Self {
        Self {
            sparse_set,
            group_len_ref,
        }
    }
}

unsafe impl<'a, T> ComponentView<'a> for &'a CompMut<'a, T>
where
    T: Component,
{
    type Flags = *const ComponentFlags;
    type Data = *const T;
    type Item = &'a T;

    fn group_len_ref(&self) -> Option<&usize> {
        self.group_len_ref
    }

    fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Flags, Self::Data) {
        let (sparse, dense, flags, data) = self.sparse_set.split();
        (sparse, dense, flags.as_ptr(), data.as_ptr())
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags {
        *flags.add(index)
    }

    unsafe fn get_item(_: Self::Flags, data: Self::Data, index: usize) -> Option<Self::Item> {
        Some(&*data.add(index))
    }
}

unsafe impl<'a: 'b, 'b, T> ComponentView<'b> for &'b mut CompMut<'a, T>
where
    T: Component,
{
    type Flags = *mut ComponentFlags;
    type Data = *mut T;
    type Item = ComponentRefMut<'b, T>;

    fn group_len_ref(&self) -> Option<&usize> {
        self.group_len_ref
    }

    fn split(self) -> (&'b SparseArray, &'b [Entity], Self::Flags, Self::Data) {
        let (sparse, dense, flags, data) = self.sparse_set.split_mut();
        (sparse, dense, flags.as_mut_ptr(), data.as_mut_ptr())
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags {
        *flags.add(index)
    }

    unsafe fn get_item(flags: Self::Flags, data: Self::Data, index: usize) -> Option<Self::Item> {
        Some(ComponentRefMut::new(
            &mut *data.add(index),
            &mut *flags.add(index),
        ))
    }
}
