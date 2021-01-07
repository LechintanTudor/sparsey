use crate::storage::{Entity, IndexEntity};
use std::hint::unreachable_unchecked;
use std::{iter, mem, ptr};

pub const PAGE_SIZE: usize = 32;
pub type EntityPage = Option<Box<[IndexEntity; PAGE_SIZE]>>;

#[derive(Clone, Default, Debug)]
pub struct SparseArray {
    pages: Vec<EntityPage>,
}

impl SparseArray {
    pub fn clear(&mut self) {
        self.pages.clear()
    }

    pub fn insert(&mut self, entity: Entity) -> IndexEntity {
        let dest = self.get_mut_or_allocate(entity.index());
        let prev = *dest;
        *dest = IndexEntity::new(entity.id(), entity.gen());
        prev
    }

    pub fn remove(&mut self, entity: Entity) -> Option<IndexEntity> {
        self.get_mut(entity)
            .map(|e| mem::replace(e, IndexEntity::INVALID))
    }

    pub fn delete(&mut self, entity: Entity) {
        self.get_mut(entity).map(|e| *e = IndexEntity::INVALID);
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.get(entity).is_some()
    }

    pub fn get(&self, entity: Entity) -> Option<&IndexEntity> {
        self.pages
            .get(page_index(entity))
            .and_then(|page| page.as_ref())
            .map(|page| &page[local_index(entity)])
            .filter(|e| e.is_valid() && e.gen() == entity.gen())
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut IndexEntity> {
        self.pages
            .get_mut(page_index(entity))
            .and_then(|page| page.as_mut())
            .map(|page| &mut page[local_index(entity)])
            .filter(|e| e.is_valid() && e.gen() == entity.gen())
    }

    pub fn get_mut_or_allocate(&mut self, index: usize) -> &mut IndexEntity {
        self.allocate_at(index);
        unsafe { self.get_unchecked_mut(index) }
    }

    pub unsafe fn get_unchecked(&self, index: usize) -> &IndexEntity {
        match self.pages.get_unchecked(index / PAGE_SIZE) {
            Some(page) => page.get_unchecked(index % PAGE_SIZE),
            None => unreachable_unchecked(),
        }
    }

    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut IndexEntity {
        match self.pages.get_unchecked_mut(index / PAGE_SIZE) {
            Some(page) => page.get_unchecked_mut(index % PAGE_SIZE),
            None => unreachable_unchecked(),
        }
    }

    pub fn allocate_at(&mut self, index: usize) {
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
    }

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
    Some(Box::new([IndexEntity::INVALID; PAGE_SIZE]))
}
