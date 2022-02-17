use crate::storage::{Entity, IndexEntity};
use std::{iter, ptr};

const PAGE_SIZE: usize = 64;
type EntityPage = Option<Box<[Option<IndexEntity>; PAGE_SIZE]>>;

/// Maps sparse indexes to dense indexes. Used internally by `ComponentStorage`.
#[doc(hidden)]
#[derive(Clone, Debug, Default)]
pub struct SparseArray {
    pages: Vec<EntityPage>,
}

impl SparseArray {
    /// Returns the dense index mapped to `entity`, if any.
    pub fn get(&self, entity: Entity) -> Option<usize> {
        self.pages
            .get(page_index(entity))
            .and_then(Option::as_ref)
            .and_then(|p| p[local_index(entity)].as_ref())
            .filter(|e| e.version() == entity.version())
            .map(IndexEntity::dense)
    }

    /// Returns `true` if the array contains `entity`.
    pub fn contains(&self, entity: Entity) -> bool {
        self.pages
            .get(page_index(entity))
            .and_then(Option::as_ref)
            .and_then(|p| p[local_index(entity)].as_ref())
            .filter(|e| e.version() == entity.version())
            .is_some()
    }

    /// Returns the dense index mapped to `sparse`, if any.
    pub fn get_from_sparse(&self, sparse: usize) -> Option<usize> {
        self.pages
            .get(sparse / PAGE_SIZE)
            .and_then(Option::as_ref)
            .and_then(|p| p[sparse % PAGE_SIZE].as_ref())
            .map(IndexEntity::dense)
    }

    /// Returns `true` if the array contains `sparse`.
    pub fn contains_sparse(&self, sparse: usize) -> bool {
        self.pages
            .get(sparse / PAGE_SIZE)
            .and_then(Option::as_ref)
            .and_then(|p| p[sparse % PAGE_SIZE].as_ref())
            .is_some()
    }

    pub(crate) fn remove(&mut self, entity: Entity) -> Option<usize> {
        self.pages
            .get_mut(page_index(entity))
            .and_then(Option::as_mut)
            .map(|p| &mut p[local_index(entity)])
            .filter(|e| e.map(|e| e.version() == entity.version()).unwrap_or(false))?
            .take()
            .map(|e| e.dense())
    }

    /// Returns the `IndexEntity` slot at `index` without checking if the
    /// `index` it is valid.
    pub(crate) unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut Option<IndexEntity> {
        self.pages
            .get_unchecked_mut(index / PAGE_SIZE)
            .as_mut()
            .unwrap_unchecked()
            .get_unchecked_mut(index % PAGE_SIZE)
    }

    /// Returns the `IndexEntity` slot at `index`. May allocate memory if the
    /// index cannot be stored in the allocated pages.
    pub(crate) fn get_mut_or_allocate_at(&mut self, index: usize) -> &mut Option<IndexEntity> {
        let page_index = index / PAGE_SIZE;

        if page_index < self.pages.len() {
            let page = &mut self.pages[page_index];

            if page.is_none() {
                *page = empty_page();
            }
        } else {
            let extra_uninit_pages = page_index - self.pages.len();
            self.pages.reserve(extra_uninit_pages + 1);
            self.pages.extend(iter::repeat(uninit_page()).take(extra_uninit_pages));
            self.pages.push(empty_page())
        }

        unsafe {
            self.pages
                .get_unchecked_mut(page_index)
                .as_mut()
                .unwrap_unchecked()
                .get_unchecked_mut(index % PAGE_SIZE)
        }
    }

    /// Swaps the `IndexEntities` at `a` and `b` without checking if `a` and `b`
    /// are valid.
    pub(crate) unsafe fn swap_nonoverlapping(&mut self, a: usize, b: usize) {
        let pa = self.get_unchecked_mut(a) as *mut _;
        let pb = self.get_unchecked_mut(b) as *mut _;
        ptr::swap_nonoverlapping(pa, pb, 1);
    }

    /// Removes all entities from the array.
    pub(crate) fn clear(&mut self) {
        self.pages.iter_mut().for_each(|p| *p = None);
    }
}

fn page_index(entity: Entity) -> usize {
    entity.sparse() / PAGE_SIZE
}

fn local_index(entity: Entity) -> usize {
    entity.sparse() % PAGE_SIZE
}

fn uninit_page() -> EntityPage {
    None
}

fn empty_page() -> EntityPage {
    Some(Box::new([None; PAGE_SIZE]))
}
