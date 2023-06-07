use std::sync::{LockResult, Mutex, MutexGuard, PoisonError};
use thiserror::Error;

use crate::marker::{PhantomUnsend, PhantomUnsync};

type EmtuxVecResult<T> = Result<T, EmtuxVecError<T>>;

#[derive(Error, Debug)]
pub enum EmtuxVecError<T> {
    #[error("There was no element with the given index")]
    IndexOutsideBounds,
    #[error("The same element was asked for more than once")]
    DuplicateIndex,
    #[error("Lock is poisoned")]
    Poison(#[from] PoisonError<T>),
}

fn convert_result<T>(r: LockResult<T>) -> EmtuxVecResult<T> {
    match r {
        Ok(x) => Ok(x),
        Err(e) => Err(EmtuxVecError::Poison(e)),
    }
}

#[derive(Debug)]
pub struct EmtuxVec<T> {
    vec: Vec<Mutex<T>>,
    _markers: (PhantomUnsend, PhantomUnsync),
}

impl<T> Default for EmtuxVec<T> {
    fn default() -> Self {
        Self {
            vec: Default::default(),
            _markers: Default::default(),
        }
    }
}

impl<T> FromIterator<T> for EmtuxVec<T> {
    fn from_iter<ITER: IntoIterator<Item = T>>(iter: ITER) -> Self {
        let mut s = Self::default();
        s.extend(iter.into_iter());
        s
    }
}

impl<T> IntoIterator for EmtuxVec<T> {
    type Item = LockResult<T>;

    type IntoIter = EmtuxVecIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        EmtuxVecIntoIter(self.vec.into_iter())
    }
}

impl<'a, T> IntoIterator for &'a EmtuxVec<T> {
    type Item = LockResult<MutexGuard<'a, T>>;

    type IntoIter = EmtuxVecIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        EmtuxVecIter(self.vec.iter())
    }
}

pub struct EmtuxVecIter<'a, T>(std::slice::Iter<'a, Mutex<T>>);

impl<'a, T> Iterator for EmtuxVecIter<'a, T> {
    type Item = LockResult<MutexGuard<'a, T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.0.next()?;

        Some(n.lock())
    }
}

pub struct EmtuxVecIntoIter<T>(<Vec<Mutex<T>> as IntoIterator>::IntoIter);

impl<T> Iterator for EmtuxVecIntoIter<T> {
    type Item = LockResult<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.0.next()?;
        Some(n.into_inner())
    }
}

impl<T> EmtuxVec<T> {
    pub fn push(&mut self, value: T) {
        self.vec.push(Mutex::new(value));
    }

    pub fn extend(&mut self, iter: impl Iterator<Item = T>) {
        self.vec.extend(iter.map(|x| Mutex::new(x)))
    }

    pub fn get_view<'a>(self: &'a Self) -> EmtuxVecView<'a, T> {
        EmtuxVecView { vec: &self.vec }
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = LockResult<MutexGuard<'a, T>>> {
        self.into_iter()
    }

    pub fn get<'b>(&'b self, index: usize) -> EmtuxVecResult<MutexGuard<T>> {
        match self.vec.get(index) {
            Some(x) => convert_result(x.lock()),
            None => Err(EmtuxVecError::IndexOutsideBounds),
        }
    }

    pub fn get_many<const COUNT: usize>(
        &self,
        indices: [usize; COUNT],
    ) -> [EmtuxVecResult<MutexGuard<T>>; COUNT] {
        get_many(&self.vec, indices)
    }

    pub fn clear(&mut self) {
        self.vec.clear()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
}

#[derive(Debug)]
pub struct EmtuxVecView<'a, T> {
    vec: &'a Vec<Mutex<T>>,
}

fn get_many<'b, T, const COUNT: usize>(
    vec: &'b Vec<Mutex<T>>,
    indices: [usize; COUNT],
) -> [EmtuxVecResult<MutexGuard<T>>; COUNT] {
    debug_assert!(COUNT <= 34);

    let permutation = importunate::Permutation::<u128, COUNT>::calculate_incomplete(&indices);

    let mut results: [EmtuxVecResult<MutexGuard<T>>; COUNT] =
        [(); COUNT].map(|_| Err(EmtuxVecError::IndexOutsideBounds));
    let mut last_index: Option<usize> = None;

    for i in 0..indices.len() {
        let indices_index = permutation.index_of(&i, |x| *x as u8) as usize;

        let vec_index = indices[indices_index as usize];

        if Some(vec_index) == last_index {
            results[indices_index] = Err(EmtuxVecError::DuplicateIndex);
            continue;
        }
        last_index = Some(vec_index);

        match vec.get(vec_index as usize) {
            Some(mutex) => {
                let r = mutex.lock();
                results[indices_index] = convert_result(r);
            }
            None => {}
        }
    }

    results
}

impl<'a, T> EmtuxVecView<'a, T> {
    pub fn get<'b>(&'b mut self, index: usize) -> EmtuxVecResult<MutexGuard<T>> {
        match self.vec.get(index) {
            Some(x) => convert_result(x.lock()),
            None => Err(EmtuxVecError::IndexOutsideBounds),
        }
    }

    /// COUNT must be at most 34
    pub fn get_many<'b, const COUNT: usize>(
        &'b mut self,
        indices: [usize; COUNT],
    ) -> [EmtuxVecResult<MutexGuard<T>>; COUNT] {
        get_many(&self.vec, indices)
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }
}
