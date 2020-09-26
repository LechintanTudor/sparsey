use crate::sparse_array::SparseArray;
use std::mem;

const INVALID: usize = usize::MAX;

pub struct SparseSet<T> {
    sparse: SparseArray<usize>,
    dense: Vec<usize>,
    data: Vec<T>,
}

impl<T> Default for SparseSet<T> {
    fn default() -> Self {
        Self {
            sparse: Default::default(),
            dense: Default::default(),
            data: Default::default(),
        }
    }
}

impl<T> SparseSet<T> {
    pub fn contains(&self, index: usize) -> bool {
        *self.sparse.get(index).unwrap_or(&INVALID) != INVALID
    }

    pub fn insert(&mut self, index: usize, value: T) -> Option<T> {
        let sparse_index = self.sparse.get_mut_or_extend(index, INVALID);

        if *sparse_index != INVALID {
            Some(mem::replace(&mut self.data[*sparse_index], value))
        } else {
            *sparse_index = self.dense.len();
            self.dense.push(index);
            self.data.push(value);
            None
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        let sparse_index = *self.sparse.get(index)?;

        if sparse_index != INVALID {
            let last_dense = *self.dense.last()?;
            self.dense.swap_remove(sparse_index);

            *self.sparse.get_mut(last_dense)? = sparse_index;
            *self.sparse.get_mut(index)? = INVALID;

            Some(self.data.swap_remove(sparse_index))
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let sparse_index = *self.sparse.get(index)?;

        if sparse_index != INVALID {
            Some(&self.data[sparse_index])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let sparse_index = *self.sparse.get(index)?;

        if sparse_index != INVALID {
            Some(&mut self.data[sparse_index])
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> &[T] {
        self.as_ref()
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.as_mut()
    }

    pub fn dense(&self) -> &[usize] {
        &self.dense
    }
}

impl<T> AsRef<[T]> for SparseSet<T> {
    fn as_ref(&self) -> &[T] {
        &self.data
    }
}

impl<T> AsMut<[T]> for SparseSet<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut set = SparseSet::<u32>::default();
        set.insert(100, 10);
        set.insert(200, 20);
        set.insert(300, 30);

        assert_eq!(set.insert(100, 11), Some(10));
        assert_eq!(set.insert(200, 21), Some(20));
        assert_eq!(set.insert(300, 31), Some(30));

        assert_eq!(set.get(100), Some(&11));
        assert_eq!(set.get(200), Some(&21));
        assert_eq!(set.get(300), Some(&31));
    }

    #[test]
    fn remove() {
        let mut set = SparseSet::<u32>::default();
        set.insert(0, 10);
        set.insert(1, 20);
        set.insert(2, 30);

        assert_eq!(set.remove(0), Some(10));
        assert_eq!(set.remove(0), None);

        assert_eq!(set.remove(1), Some(20));
        assert_eq!(set.remove(1), None);

        assert_eq!(set.remove(2), Some(30));
        assert_eq!(set.remove(2), None);
    }
}
