#![feature(array_zip)]
#![feature(generic_arg_infer)]

use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T, const M: usize, const N: usize>([[T; M]; N]);

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn new(mat: [[T; M]; N]) -> Self {
        Self(mat)
    }
}

impl<T, const M: usize, const N: usize> From<[[T; M]; N]> for Matrix<T, M, N> {
    fn from(value: [[T; M]; N]) -> Self {
        Self(value)
    }
}

impl<T: Add<U>, U: Add<T>, const M: usize, const N: usize> Add<Matrix<U, M, N>>
    for Matrix<T, M, N>
{
    type Output = Matrix<T::Output, M, N>;
    fn add(self, rhs: Matrix<U, M, N>) -> Self::Output {
        self.0
            .zip(rhs.0)
            .map(|(rows1, rows2)| rows1.zip(rows2).map(|(a, b)| a + b))
            .into()
    }
}

impl<T: Sub<U>, U: Sub<T>, const M: usize, const N: usize> Sub<Matrix<U, M, N>>
    for Matrix<T, M, N>
{
    type Output = Matrix<T::Output, M, N>;
    fn sub(self, rhs: Matrix<U, M, N>) -> Self::Output {
        self.0
            .zip(rhs.0)
            .map(|(rows1, rows2)| rows1.zip(rows2).map(|(a, b)| a - b))
            .into()
    }
}

impl<T: Mul<U>, U: Mul<T>, const M: usize, const N: usize, const O: usize> Mul<Matrix<U, N, O>>
    for Matrix<T, M, N>
{
    type Output = Matrix<<T as Mul<U>>::Output, M, O>;
    fn mul(self, rhs: Matrix<U, N, O>) -> Self::Output {
        todo!("No idea how to do this elegantly without unnecessarily initialising a matrix to then re-initialise it with values")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_test() {
        let test = Matrix::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]])
            + [[9, 8, 7], [6, 5, 4], [3, 2, 1]].into();
        assert_eq!(test, [[10, 10, 10], [10, 10, 10], [10, 10, 10]].into());
    }

    #[test]
    fn sub_test() {
        let test = Matrix::new([[10, 10, 10], [10, 10, 10], [10, 10, 10]])
            - [[9, 8, 7], [6, 5, 4], [3, 2, 1]].into();
        assert_eq!(test, [[1, 2, 3], [4, 5, 6], [7, 8, 9]].into());
    }
}
