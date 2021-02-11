use crate::storage::{Entity, IndexEntity};
use std::hint::unreachable_unchecked;
use std::{iter, ptr};

const PAGE_SIZE: usize = 32;
type EntityPage = Option<Box<[Option<IndexEntity>; PAGE_SIZE]>>;

#[derive(Clone, Default, Debug)]
pub struct SparseArray {
    pages: Vec<EntityPage>,
}

impl SparseArray {
    pub fn clear(&mut self) {
        for page in self.pages.iter_mut() {
            *page = uninit_page();
        }
    }

    pub fn insert(&mut self, entity: Entity) -> Option<IndexEntity> {
        let dest = self.get_mut_or_allocate(entity.index());
        let prev = *dest;
        *dest = Some(IndexEntity::new(entity.id(), entity.gen()));
        prev
    }

    pub fn remove(&mut self, entity: Entity) -> Option<IndexEntity> {
        self.get_mut(entity).map(|e| e.take()).flatten()
    }

    pub fn delete(&mut self, entity: Entity) {
        self.get_mut(entity).map(|e| *e = None);
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.get_index_entity(entity).is_some()
    }

    pub fn get_index_entity(&self, entity: Entity) -> Option<IndexEntity> {
        self.pages
            .get(page_index(entity))
            .and_then(|p| p.as_ref())
            .and_then(|p| p[local_index(entity)])
            .filter(|e| e.gen() == entity.gen())
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut Option<IndexEntity>> {
        let index_entity = self
            .pages
            .get_mut(page_index(entity))
            .and_then(|p| p.as_mut())
            .map(|p| &mut p[local_index(entity)])?;

        if (*index_entity)?.gen() == entity.gen() {
            Some(index_entity)
        } else {
            None
        }
    }

    pub fn get_mut_or_allocate(&mut self, index: usize) -> &mut Option<IndexEntity> {
        self.allocate_at(index);
        unsafe { self.get_unchecked_mut(index) }
    }

    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut Option<IndexEntity> {
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
    Some(Box::new([None; PAGE_SIZE]))
}
