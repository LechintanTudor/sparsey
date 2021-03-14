use crate::data::{Entity, IndexEntity};
use std::hint::unreachable_unchecked;
use std::{iter, mem, ptr};

const PAGE_SIZE: usize = 32;
type EntityPage = Option<Box<[Option<IndexEntity>; PAGE_SIZE]>>;

/// Data structure which maps `Entities` to indexes.
#[derive(Clone, Debug, Default)]
pub struct SparseVec {
    pages: Vec<EntityPage>,
}

impl SparseVec {
    /// Remove all `Entities` in the `SparseVec`.
    pub fn clear(&mut self) {
        for page in self.pages.iter_mut() {
            *page = uninit_page();
        }
    }

    /// Insert an `IndexEntity` at the given `index`
    /// and return the previous `IndexEntity`, if any.
    pub fn insert(&mut self, index: usize, entity: IndexEntity) -> Option<IndexEntity> {
        mem::replace(self.get_mut_or_allocate_at(index), Some(entity))
    }

    /// Remove the `IndexEntity` at the given `Entity`.
    pub fn remove(&mut self, entity: Entity) -> Option<IndexEntity> {
        self.get_mut(entity).map(|e| e.take()).flatten()
    }

    /// Delete the `IndexEntity` at the given `Entity`.
    pub fn delete(&mut self, entity: Entity) {
        self.get_mut(entity).map(|e| *e = None);
    }

    /// Check if the `SparseVec` contains the given `Entity`.
    pub fn contains(&self, entity: Entity) -> bool {
        self.get_index_entity(entity).is_some()
    }

    /// Get the `IndexEntity` at the given `Entity`.
    pub fn get_index_entity(&self, entity: Entity) -> Option<IndexEntity> {
        self.pages
            .get(page_index(entity))
            .and_then(|p| p.as_ref())
            .and_then(|p| p[local_index(entity)])
            .filter(|e| e.ver() == entity.ver())
    }

    /// Get an exclusive reference to the `IndexEntity` slot at the given `Entity`.
    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut Option<IndexEntity>> {
        self.pages
            .get_mut(page_index(entity))
            .and_then(|page| page.as_mut())
            .map(|page| &mut page[local_index(entity)])
            .filter(|e| e.map(|e| e.ver()) == Some(entity.ver()))
    }

    /// Get an excusive reference to the `IndexEntity` slot at the given `index`
    /// without checking the validity of the `index`.
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut Option<IndexEntity> {
        match self.pages.get_unchecked_mut(index / PAGE_SIZE) {
            Some(page) => page.get_unchecked_mut(index % PAGE_SIZE),
            None => unreachable_unchecked(),
        }
    }

    /// Get an exclusive reference to the `IndexEntity` slot at the given `index`.
    /// If the `index` is larger than the maximum capacity of the `SparseVec`,
    /// extra memory is allocated to accomodate the new slot.
    pub fn get_mut_or_allocate_at(&mut self, index: usize) -> &mut Option<IndexEntity> {
        let page_index = index / PAGE_SIZE;

        if page_index < self.pages.len() {
            let page = &mut self.pages[page_index];

            if page.is_none() {
                *page = default_page();
            }
        } else {
            let extra_uninit_pages = page_index - self.pages.len();
            self.pages.reserve(extra_uninit_pages + 1);
            self.pages
                .extend(iter::repeat(uninit_page()).take(extra_uninit_pages));
            self.pages.push(default_page())
        }

        unsafe {
            match self.pages.get_unchecked_mut(page_index) {
                Some(page) => page.get_unchecked_mut(index % PAGE_SIZE),
                None => unreachable_unchecked(),
            }
        }
    }

    /// Swap the `IndexEntities` at the given indexes.
    /// The indexes must be valid and must not overlap.
    pub unsafe fn swap_unchecked(&mut self, a: usize, b: usize) {
        let p1 = self.get_unchecked_mut(a) as *mut _;
        let p2 = self.get_unchecked_mut(b) as *mut _;
        ptr::swap_nonoverlapping(p1, p2, 1);
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

fn default_page() -> EntityPage {
    Some(Box::new([None; PAGE_SIZE]))
}
