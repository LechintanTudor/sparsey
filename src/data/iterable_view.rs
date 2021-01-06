use crate::entity::Entity;
use crate::storage::*;
use std::marker::PhantomData;
use std::ops::Not;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ParentGroup {
    group_id: usize,
    subgroup_index: usize,
    subgroup_len: usize,
}

impl ParentGroup {
    pub fn new(group_id: usize, subgroup_index: usize, subgroup_len: usize) -> Self {
        Self {
            group_id,
            subgroup_index,
            subgroup_len,
        }
    }

    pub fn group_id(&self) -> usize {
        self.group_id
    }

    pub fn subgroup_index(&self) -> usize {
        self.subgroup_index
    }

    pub fn subgroup_len(&self) -> usize {
        self.subgroup_len
    }
}

pub trait IterableView<'a> {
    type Data: 'a + Copy;
    type Flags: 'a + Copy;
    type Output: 'a;

    fn parent_group(&self) -> Option<ParentGroup>;

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags);

    unsafe fn matches_flags(flags: Self::Flags, index: usize) -> bool;

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags;

    unsafe fn get(data: Self::Data, flags: Self::Flags, index: usize) -> Self::Output;
}

pub unsafe trait UnfilteredIterableView<'a>
where
    Self: IterableView<'a>,
{
}

pub struct Added<'a, V>
where
    V: UnfilteredIterableView<'a>,
{
    view: V,
    phantom: PhantomData<&'a ()>,
}

impl<'a, V> IterableView<'a> for Added<'a, V>
where
    V: UnfilteredIterableView<'a>,
{
    type Data = V::Data;
    type Flags = V::Flags;
    type Output = V::Output;

    fn parent_group(&self) -> Option<ParentGroup> {
        V::parent_group(&self.view)
    }

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags) {
        V::split(self.view)
    }

    unsafe fn matches_flags(flags: Self::Flags, index: usize) -> bool {
        (V::get_flags(flags, index) & COMPONENT_FLAG_ADDED) != 0
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags {
        V::get_flags(flags, index)
    }

    unsafe fn get(data: Self::Data, flags: Self::Flags, index: usize) -> Self::Output {
        V::get(data, flags, index)
    }
}

impl<'a, V> Not for Added<'a, V>
where
    V: UnfilteredIterableView<'a>,
{
    type Output = NotAdded<'a, V>;

    fn not(self) -> Self::Output {
        NotAdded {
            view: self.view,
            phantom: self.phantom,
        }
    }
}

pub struct NotAdded<'a, V>
where
    V: UnfilteredIterableView<'a>,
{
    view: V,
    phantom: PhantomData<&'a ()>,
}

impl<'a, V> IterableView<'a> for NotAdded<'a, V>
where
    V: UnfilteredIterableView<'a>,
{
    type Data = V::Data;
    type Flags = V::Flags;
    type Output = V::Output;

    fn parent_group(&self) -> Option<ParentGroup> {
        V::parent_group(&self.view)
    }

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags) {
        V::split(self.view)
    }

    unsafe fn matches_flags(flags: Self::Flags, index: usize) -> bool {
        (V::get_flags(flags, index) & COMPONENT_FLAG_ADDED) == 0
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags {
        V::get_flags(flags, index)
    }

    unsafe fn get(data: Self::Data, flags: Self::Flags, index: usize) -> Self::Output {
        V::get(data, flags, index)
    }
}

pub struct Changed<'a, V>
where
    V: UnfilteredIterableView<'a>,
{
    view: V,
    phantom: PhantomData<&'a ()>,
}

impl<'a, V> IterableView<'a> for Changed<'a, V>
where
    V: UnfilteredIterableView<'a>,
{
    type Data = V::Data;
    type Flags = V::Flags;
    type Output = V::Output;

    fn parent_group(&self) -> Option<ParentGroup> {
        V::parent_group(&self.view)
    }

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags) {
        V::split(self.view)
    }

    unsafe fn matches_flags(flags: Self::Flags, index: usize) -> bool {
        (V::get_flags(flags, index) & COMPONENT_FLAG_CHANGED) != 0
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags {
        V::get_flags(flags, index)
    }

    unsafe fn get(data: Self::Data, flags: Self::Flags, index: usize) -> Self::Output {
        V::get(data, flags, index)
    }
}

impl<'a, V> Not for Changed<'a, V>
where
    V: UnfilteredIterableView<'a>,
{
    type Output = NotChanged<'a, V>;

    fn not(self) -> Self::Output {
        NotChanged {
            view: self.view,
            phantom: self.phantom,
        }
    }
}

pub struct NotChanged<'a, V>
where
    V: UnfilteredIterableView<'a>,
{
    view: V,
    phantom: PhantomData<&'a ()>,
}

impl<'a, V> IterableView<'a> for NotChanged<'a, V>
where
    V: UnfilteredIterableView<'a>,
{
    type Data = V::Data;
    type Flags = V::Flags;
    type Output = V::Output;

    fn parent_group(&self) -> Option<ParentGroup> {
        V::parent_group(&self.view)
    }

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags) {
        V::split(self.view)
    }

    unsafe fn matches_flags(flags: Self::Flags, index: usize) -> bool {
        (V::get_flags(flags, index) & COMPONENT_FLAG_CHANGED) == 0
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags {
        V::get_flags(flags, index)
    }

    unsafe fn get(data: Self::Data, flags: Self::Flags, index: usize) -> Self::Output {
        V::get(data, flags, index)
    }
}

pub struct Updated<'a, V>
where
    V: UnfilteredIterableView<'a>,
{
    view: V,
    phantom: PhantomData<&'a ()>,
}

impl<'a, V> IterableView<'a> for Updated<'a, V>
where
    V: UnfilteredIterableView<'a>,
{
    type Data = V::Data;
    type Flags = V::Flags;
    type Output = V::Output;

    fn parent_group(&self) -> Option<ParentGroup> {
        V::parent_group(&self.view)
    }

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags) {
        V::split(self.view)
    }

    unsafe fn matches_flags(flags: Self::Flags, index: usize) -> bool {
        let component_flags = V::get_flags(flags, index);
        ((component_flags & COMPONENT_FLAG_ADDED) | (component_flags & COMPONENT_FLAG_CHANGED)) != 0
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags {
        V::get_flags(flags, index)
    }

    unsafe fn get(data: Self::Data, flags: Self::Flags, index: usize) -> Self::Output {
        V::get(data, flags, index)
    }
}

impl<'a, V> Not for Updated<'a, V>
where
    V: UnfilteredIterableView<'a>,
{
    type Output = NotUpdated<'a, V>;

    fn not(self) -> Self::Output {
        NotUpdated {
            view: self.view,
            phantom: self.phantom,
        }
    }
}

pub struct NotUpdated<'a, V>
where
    V: UnfilteredIterableView<'a>,
{
    view: V,
    phantom: PhantomData<&'a ()>,
}

impl<'a, V> IterableView<'a> for NotUpdated<'a, V>
where
    V: UnfilteredIterableView<'a>,
{
    type Data = V::Data;
    type Flags = V::Flags;
    type Output = V::Output;

    fn parent_group(&self) -> Option<ParentGroup> {
        V::parent_group(&self.view)
    }

    unsafe fn split(self) -> (&'a SparseArray, &'a [Entity], Self::Data, Self::Flags) {
        V::split(self.view)
    }

    unsafe fn matches_flags(flags: Self::Flags, index: usize) -> bool {
        (V::get_flags(flags, index) & (COMPONENT_FLAG_ADDED | COMPONENT_FLAG_CHANGED)) == 0
    }

    unsafe fn get_flags(flags: Self::Flags, index: usize) -> ComponentFlags {
        V::get_flags(flags, index)
    }

    unsafe fn get(data: Self::Data, flags: Self::Flags, index: usize) -> Self::Output {
        V::get(data, flags, index)
    }
}

pub fn added<'a, V>(view: V) -> Added<'a, V>
where
    V: UnfilteredIterableView<'a>,
{
    Added {
        view,
        phantom: PhantomData,
    }
}

pub fn changed<'a, V>(view: V) -> Changed<'a, V>
where
    V: UnfilteredIterableView<'a>,
{
    Changed {
        view,
        phantom: PhantomData,
    }
}

pub fn updated<'a, V>(view: V) -> Updated<'a, V>
where
    V: UnfilteredIterableView<'a>,
{
    Updated {
        view,
        phantom: PhantomData,
    }
}

pub(crate) unsafe fn get_output<'a, V>(
    data: V::Data,
    flags: V::Flags,
    index: usize,
) -> Option<V::Output>
where
    V: IterableView<'a>,
{
    if V::matches_flags(flags, index) {
        Some(V::get(data, flags, index))
    } else {
        None
    }
}
