use crate::data::{
    Component, ComponentFlags, ComponentRefMut, Entity, MappedAtomicRef, MappedAtomicRefMut,
    SparseArray, SparseSetRef, SparseSetRefMut,
};
use crate::query::SimpleIter1;
use crate::world::SubgroupInfo;
use std::ops::{Deref, DerefMut};

pub unsafe trait ComponentView<'a>
where
    Self: Sized,
{
    type Flags: 'a + Copy;
    type Data: 'a + Copy;
    type Item: 'a;

    fn subgroup_info(&self) -> Option<SubgroupInfo>;

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
    T: Send + Sync + 'static,
{
    sparse_set: MappedAtomicRef<'a, SparseSetRef<'a, T>>,
    subgroup_info: Option<SubgroupInfo<'a>>,
}

impl<'a, T> Comp<'a, T>
where
    T: Send + Sync + 'static,
{
    pub(crate) unsafe fn new(
        sparse_set: MappedAtomicRef<'a, SparseSetRef<'a, T>>,
        subgroup_info: Option<SubgroupInfo<'a>>,
    ) -> Self {
        Self {
            sparse_set,
            subgroup_info,
        }
    }

    pub fn iter(&'a self) -> SimpleIter1<'a, &'a Self>
    where
        T: Component,
    {
        SimpleIter1::new(self)
    }

    pub fn entities(&self) -> &[Entity] {
        self.sparse_set.entities()
    }
}

impl<T> AsRef<[T]> for Comp<'_, T>
where
    T: Send + Sync + 'static,
{
    fn as_ref(&self) -> &[T] {
        self.sparse_set.as_ref()
    }
}

impl<T> Deref for Comp<'_, T>
where
    T: Send + Sync + 'static,
{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.sparse_set.as_ref()
    }
}

unsafe impl<'a, T> ComponentView<'a> for &'a Comp<'a, T>
where
    T: Send + Sync + 'static,
{
    type Flags = *const ComponentFlags;
    type Data = *const T;
    type Item = &'a T;

    fn subgroup_info(&self) -> Option<SubgroupInfo> {
        self.subgroup_info
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
    T: Send + Sync + 'static,
{
    sparse_set: MappedAtomicRefMut<'a, SparseSetRefMut<'a, T>>,
    subgroup_info: Option<SubgroupInfo<'a>>,
}

impl<'a, T> CompMut<'a, T>
where
    T: Send + Sync + 'static,
{
    pub(crate) unsafe fn new(
        sparse_set: MappedAtomicRefMut<'a, SparseSetRefMut<'a, T>>,
        subgroup_info: Option<SubgroupInfo<'a>>,
    ) -> Self {
        Self {
            sparse_set,
            subgroup_info,
        }
    }

    pub fn iter(&'a self) -> SimpleIter1<'a, &'a Self>
    where
        T: Component,
    {
        SimpleIter1::new(self)
    }

    pub fn iter_mut(&mut self) -> SimpleIter1<&mut Self>
    where
        T: Component,
    {
        SimpleIter1::new(self)
    }

    pub fn entities(&self) -> &[Entity] {
        self.sparse_set.entities()
    }
}

impl<T> AsRef<[T]> for CompMut<'_, T>
where
    T: Send + Sync + 'static,
{
    fn as_ref(&self) -> &[T] {
        self.sparse_set.as_ref()
    }
}

impl<T> AsMut<[T]> for CompMut<'_, T>
where
    T: Send + Sync + 'static,
{
    fn as_mut(&mut self) -> &mut [T] {
        self.sparse_set.as_mut()
    }
}

impl<T> Deref for CompMut<'_, T>
where
    T: Send + Sync + 'static,
{
    type Target = [T];

    fn deref(&self) -> &Self::Target
    where
        T: Send + Sync + 'static,
    {
        self.sparse_set.as_ref()
    }
}

impl<T> DerefMut for CompMut<'_, T>
where
    T: Send + Sync + 'static,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.sparse_set.as_mut()
    }
}

unsafe impl<'a, T> ComponentView<'a> for &'a CompMut<'a, T>
where
    T: Component,
{
    type Flags = *const ComponentFlags;
    type Data = *const T;
    type Item = &'a T;

    fn subgroup_info(&self) -> Option<SubgroupInfo> {
        self.subgroup_info
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

    fn subgroup_info(&self) -> Option<SubgroupInfo> {
        self.subgroup_info
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

pub struct SparseSetRefMutBorrow<'a, T>(MappedAtomicRefMut<'a, SparseSetRefMut<'a, T>>)
where
    T: Component;

impl<'a, T> SparseSetRefMutBorrow<'a, T>
where
    T: Component,
{
    pub(crate) fn new(sparse_set: MappedAtomicRefMut<'a, SparseSetRefMut<'a, T>>) -> Self {
        Self(sparse_set)
    }
}

impl<'a, T> Deref for SparseSetRefMutBorrow<'a, T>
where
    T: Component,
{
    type Target = SparseSetRefMut<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> DerefMut for SparseSetRefMutBorrow<'a, T>
where
    T: Component,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
