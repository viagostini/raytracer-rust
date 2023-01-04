use std::ops::{Index, IndexMut};

use float_cmp::approx_eq;

#[derive(Debug, Copy, Clone)]
pub struct Matrix2 {
    pub data: [[f64; 2]; 2],
}

impl Matrix2 {
    pub const fn new() -> Matrix2 {
        Matrix2::from([[0.0, 0.0], [0.0, 0.0]])
    }

    pub const fn from(data: [[f64; 2]; 2]) -> Matrix2 {
        Matrix2 { data }
    }

    pub fn determinant(&self) -> f64 {
        self[[0, 0]] * self[[1, 1]] - self[[1, 0]] * self[[0, 1]]
    }
}

impl Index<[usize; 2]> for Matrix2 {
    type Output = f64;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let [i, j] = index;

        &self.data[i][j]
    }
}

impl IndexMut<[usize; 2]> for Matrix2 {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let [i, j] = index;

        &mut self.data[i][j]
    }
}

impl PartialEq for Matrix2 {
    fn eq(&self, other: &Self) -> bool {
        self.data
            .iter()
            .flatten()
            .zip(other.data.iter().flatten())
            .all(|(&a, &b)| approx_eq!(f64, a, b))
    }
}

impl Eq for Matrix2 {}

#[cfg(test)]
pub mod matrix2_tests {
    use float_cmp::assert_approx_eq;

    use super::*;

    #[test]
    fn create_matrix2() {
        let m = Matrix2::from([[1.0, 2.0], [5.5, 6.5]]);

        assert_eq!(m[[0, 0]], 1.0);
        assert_eq!(m[[0, 1]], 2.0);
        assert_eq!(m[[1, 0]], 5.5);
        assert_eq!(m[[1, 1]], 6.5);
    }

    #[test]
    fn can_compare_two_equal_matrix2() {
        let m1 = Matrix2::from([[1.0, 2.0], [5.5, 6.5]]);

        let m2 = Matrix2::from([[1.0, 2.0], [5.5, 6.5]]);

        assert_eq!(m1, m2);
    }

    #[test]
    fn can_compare_two_similar_but_different_matrix2() {
        let m1 = Matrix2::from([[1.0, 2.0], [5.5, 6.5]]);

        let m2 = Matrix2::from([[1.0001, 2.0], [5.5, 6.5]]);

        assert_ne!(m1, m2);
    }

    #[test]
    fn determinant_of_matrix2() {
        let m = Matrix2::from([[1.0, 5.0], [-3.0, 2.0]]);

        assert_approx_eq!(f64, m.determinant(), 17.0);
    }
}
