use std::{
    iter,
    ops::{Index, IndexMut},
};

const PAGE_SIZE: usize = 32;

#[derive(Clone, Debug)]
pub struct SparseArray<T>
where
    T: Copy + Clone,
{
    pages: Vec<Option<Box<[T; PAGE_SIZE]>>>,
}

impl<T> Default for SparseArray<T>
where
    T: Copy + Clone,
{
    fn default() -> Self {
        Self {
            pages: Default::default(),
        }
    }
}

impl<T> SparseArray<T>
where
    T: Copy + Clone,
{
    pub fn clear(&mut self) {
        self.pages.clear();
    }

    pub fn clear_pages(&mut self) {
        self.pages.iter_mut().for_each(|p| *p = None);
    }

    pub fn get_mut_or_extend(&mut self, index: usize, default: T) -> &mut T {
        let page_index = index / PAGE_SIZE;
        let index_in_page = index % PAGE_SIZE;

        if page_index >= self.pages.len() {
            let extra_empty = page_index - self.pages.len();
            self.pages.reserve(extra_empty + 1);
            self.pages.extend(iter::repeat(None).take(extra_empty));
            self.pages.push(Some(Box::new([default; PAGE_SIZE])));
            &mut self.pages.last_mut().unwrap().as_mut().unwrap()[index_in_page]
        } else {
            if self.pages[page_index].is_none() {
                self.pages[page_index] = Some(Box::new([default; PAGE_SIZE]));
            }

            &mut self.pages[page_index].as_mut().unwrap()[index_in_page]
        }
    }

    pub fn insert(&mut self, index: usize, value: T, default: T) {
        *self.get_mut_or_extend(index, default) = value;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.pages
            .get(index / PAGE_SIZE)
            .and_then(|page| page.as_ref())
            .and_then(|page| Some(&page[index % PAGE_SIZE]))
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.pages
            .get_mut(index / PAGE_SIZE)
            .and_then(|page| page.as_mut())
            .and_then(|page| Some(&mut page[index % PAGE_SIZE]))
    }
}

impl<T> Index<usize> for SparseArray<T>
where
    T: Copy + Clone,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> IndexMut<usize> for SparseArray<T>
where
    T: Copy + Clone,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut array = SparseArray::<i32>::default();
        array.insert(100, 0, -1);
        array.insert(200, 1, -1);
        array.insert(300, 2, -1);

        assert_eq!(array.get(100), Some(&0));
        assert_eq!(array.get(200), Some(&1));
        assert_eq!(array.get(300), Some(&2));
    }
}
