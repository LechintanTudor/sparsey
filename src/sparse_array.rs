use crate::Entity;
use std::{hint::unreachable_unchecked, iter, mem};

pub const PAGE_SIZE: usize = 32;
pub type EntityPage = Option<Box<[Entity; PAGE_SIZE]>>;

#[derive(Clone, Default, Debug)]
pub struct SparseArray {
    pages: Vec<EntityPage>,
}

impl SparseArray {
    pub fn clear(&mut self) {
        self.pages.clear()
    }

    pub fn insert(&mut self, entity: Entity, value: Entity) {
        *self.get_mut_or_allocate(entity) = value;
    }

    pub fn remove(&mut self, entity: Entity) -> Option<Entity> {
        self.get_mut(entity)
            .map(|e| mem::replace(e, Entity::INVALID))
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.get(entity).unwrap_or(&Entity::INVALID).is_valid()
    }

    pub fn get(&self, entity: Entity) -> Option<&Entity> {
        self.pages
            .get(page_index(entity))
            .and_then(|page| page.as_ref())
            .map(|page| &page[local_index(entity)])
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut Entity> {
        self.pages
            .get_mut(page_index(entity))
            .and_then(|page| page.as_mut())
            .map(|page| &mut page[local_index(entity)])
    }

    pub fn get_mut_or_allocate(&mut self, entity: Entity) -> &mut Entity {
        self.allocate_at(entity.index() as usize);
        unsafe { self.get_mut_unchecked(entity) }
    }

    pub unsafe fn get_unchecked(&self, entity: Entity) -> &Entity {
        match self.pages.get_unchecked(page_index(entity)) {
            Some(page) => page.get_unchecked(local_index(entity)),
            None => unreachable_unchecked(),
        }
    }

    pub unsafe fn get_mut_unchecked(&mut self, entity: Entity) -> &mut Entity {
        match self.pages.get_unchecked_mut(page_index(entity)) {
            Some(page) => page.get_unchecked_mut(local_index(entity)),
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
    Some(Box::new([Entity::INVALID; PAGE_SIZE]))
}
