use std::ops::{Add, AddAssign, Deref, Mul};

use anyhow::{anyhow, Result};

pub struct Vector<T> {
    data: Vec<T>,
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    // assert_eq!(a.data.len(), b.data.len());
    if a.len() != b.len() {
        // a.len() -> a.data.len(): Deref trait
        return Err(anyhow!("Vectors must have the same length"));
    }

    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }

    // let result = a.data.iter().zip(b.data.iter()).map(|(x, y)| *x * *y).fold(T::default(), |acc, x| acc + x);
    Ok(sum)
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }
}
