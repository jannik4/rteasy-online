#![deny(rust_2018_idioms)]

use std::ops::Index;

#[derive(Debug)]
pub struct SplitVec<T> {
    inner: Vec<T>,
    split_at: usize,
}

impl<T> SplitVec<T> {
    pub fn new(vec: Vec<T>, split_at: usize) -> Self {
        assert!(split_at <= vec.len());
        Self { inner: vec, split_at }
    }

    pub fn get(&self, index: usize) -> Option<(&T, bool)> {
        let value = self.inner.get(index)?;
        Some((value, index < self.split_at))
    }

    pub fn as_slice(&self) -> &[T] {
        self.inner.as_slice()
    }

    pub fn split_at(&self) -> usize {
        self.split_at
    }

    pub fn front(&self) -> &[T] {
        &self.inner[..self.split_at]
    }

    pub fn back(&self) -> &[T] {
        &self.inner[self.split_at..]
    }

    pub fn mapped<F, U>(self, f: F) -> SplitVec<U>
    where
        F: FnMut(T) -> U,
    {
        SplitVec { inner: self.inner.into_iter().map(f).collect(), split_at: self.split_at }
    }

    pub fn try_mapped<F, U, E>(self, f: F) -> Result<SplitVec<U>, E>
    where
        F: FnMut(T) -> Result<U, E>,
    {
        Ok(SplitVec {
            inner: self.inner.into_iter().map(f).collect::<Result<_, _>>()?,
            split_at: self.split_at,
        })
    }
}

impl<T, I> Index<I> for SplitVec<T>
where
    [T]: Index<I, Output = [T]>,
{
    type Output = [T];

    fn index(&self, index: I) -> &Self::Output {
        &self.inner.as_slice()[index]
    }
}
