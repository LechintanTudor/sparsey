use crate::storage::{Entity, IndexEntity};
use crate::utils::UnsafeUnwrap;
use std::{iter, ptr};

const PAGE_SIZE: usize = 32;
type EntityPage = Option<Box<[Option<IndexEntity>; PAGE_SIZE]>>;

#[derive(Clone, Debug, Default)]
pub(crate) struct SparseArray {
    pages: Vec<EntityPage>,
}

impl SparseArray {
    /// Returns whether or not the array contains `entity`.
    pub fn contains(&self, entity: Entity) -> bool {
        self.get_index(entity).is_some()
    }

    /// Returns the index mapped to `entity`.
    pub fn get_index(&self, entity: Entity) -> Option<usize> {
        self.pages
            .get(page_index(entity))
            .and_then(|p| p.as_ref())
            .and_then(|p| p[local_index(entity)])
            .filter(|e| e.version() == entity.version())
            .map(|e| e.index())
    }

    /// Returns the `IndexEntity` slot at `index` without checking if the
    /// `index` it is valid.
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut Option<IndexEntity> {
        self.pages
            .get_unchecked_mut(index / PAGE_SIZE)
            .as_mut()
            .unsafe_unwrap()
            .get_unchecked_mut(index % PAGE_SIZE)
    }

    /// Returns the `IndexEntity` slot at `index`. May allocate memory if the
    /// index cannot be stored in the allocated pages.
    pub fn get_mut_or_allocate_at(&mut self, index: usize) -> &mut Option<IndexEntity> {
        let page_index = index / PAGE_SIZE;

        if page_index < self.pages.len() {
            let page = &mut self.pages[page_index];

            if page.is_none() {
                *page = empty_page();
            }
        } else {
            let extra_uninit_pages = page_index - self.pages.len();
            self.pages.reserve(extra_uninit_pages + 1);
            self.pages
                .extend(iter::repeat(uninit_page()).take(extra_uninit_pages));
            self.pages.push(empty_page())
        }

        unsafe {
            self.pages
                .get_unchecked_mut(page_index)
                .as_mut()
                .unsafe_unwrap()
                .get_unchecked_mut(index % PAGE_SIZE)
        }
    }

    /// Swaps the `IndexEntities` at `a` and `b` without checking if `a` and `b`
    /// are valid.
    pub unsafe fn swap_unchecked(&mut self, a: usize, b: usize) {
        let pa = self.get_unchecked_mut(a) as *mut _;
        let pb = self.get_unchecked_mut(b) as *mut _;
        ptr::swap(pa, pb);
    }

    /// Removes `entity` from the array and returns the `index` mapped to it.
    pub fn remove(&mut self, entity: Entity) -> Option<usize> {
        Some(self.get_mut(entity)?.take()?.index())
    }

    /// Removes all entities from the array.
    pub fn clear(&mut self) {
        self.pages.iter_mut().for_each(|p| *p = None);
    }

    /// Returns a read-only view of the array.
    pub fn as_view(&self) -> SparseArrayView {
        SparseArrayView { pages: &self.pages }
    }

    fn get_mut(&mut self, entity: Entity) -> Option<&mut Option<IndexEntity>> {
        self.pages
            .get_mut(page_index(entity))
            .and_then(|page| page.as_mut())
            .map(|page| &mut page[local_index(entity)])
            .filter(|e| e.map(|e| e.version()) == Some(entity.version()))
    }
}

/// Shared view over a `SparseArray`.
#[derive(Copy, Clone)]
pub struct SparseArrayView<'a> {
    pages: &'a [EntityPage],
}

impl SparseArrayView<'_> {
    /// Returns `true` if the view contains `entity`.
    pub fn contains(&self, entity: Entity) -> bool {
        self.pages
            .get(page_index(entity))
            .and_then(|p| p.as_ref())
            .and_then(|p| p[local_index(entity)])
            .filter(|e| e.version() == entity.version())
            .is_some()
    }

    /// Returns the index mapped to `entity` if it exists.
    pub fn get_index(&self, entity: Entity) -> Option<usize> {
        self.pages
            .get(page_index(entity))
            .and_then(|p| p.as_ref())
            .and_then(|p| p[local_index(entity)])
            .filter(|e| e.version() == entity.version())
            .map(|e| e.index())
    }
}

fn page_index(entity: Entity) -> usize {
    entity.index() as usize / PAGE_SIZE
}

fn local_index(entity: Entity) -> usize {
    entity.index() as usize % PAGE_SIZE
}

fn uninit_page() -> EntityPage {
    None
}

fn empty_page() -> EntityPage {
    Some(Box::new([None; PAGE_SIZE]))
}
