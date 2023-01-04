use std::ops::{Index, IndexMut};

use float_cmp::approx_eq;

use super::matrix2::Matrix2;

#[derive(Debug, Copy, Clone)]
pub struct Matrix3 {
    pub data: [[f64; 3]; 3],
}

impl Matrix3 {
    pub const fn new() -> Matrix3 {
        Matrix3 {
            data: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
        }
    }

    pub const fn from(data: [[f64; 3]; 3]) -> Matrix3 {
        Matrix3 { data }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix2 {
        assert!(row <= 3 && col <= 3);

        let mut m = Matrix2::new();

        for _row in 0..3 {
            for _col in 0..3 {
                if _row == row || _col == col {
                    continue;
                }

                let new_row = if _row < row { _row } else { _row - 1 };
                let new_col = if _col < col { _col } else { _col - 1 };

                m[[new_row, new_col]] = self[[_row, _col]];
            }
        }
        m
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);

        if (row + col) % 2 == 1 {
            -minor
        } else {
            minor
        }
    }

    pub fn determinant(&self) -> f64 {
        let mut det = 0.0;

        for col in 0..3 {
            det += self[[0, col]] * self.cofactor(0, col);
        }

        det
    }
}

impl Index<[usize; 2]> for Matrix3 {
    type Output = f64;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let [i, j] = index;

        &self.data[i][j]
    }
}

impl IndexMut<[usize; 2]> for Matrix3 {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let [i, j] = index;

        &mut self.data[i][j]
    }
}

impl PartialEq for Matrix3 {
    fn eq(&self, other: &Self) -> bool {
        self.data
            .iter()
            .flatten()
            .zip(other.data.iter().flatten())
            .all(|(&a, &b)| approx_eq!(f64, a, b))
    }
}

impl Eq for Matrix3 {}

#[cfg(test)]
pub mod matrix3_tests {
    use float_cmp::assert_approx_eq;

    use super::*;

    #[test]
    fn create_matrix3() {
        let m = Matrix3::from([[1.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]]);

        assert_eq!(m[[0, 0]], 1.0);
        assert_eq!(m[[1, 1]], 6.5);
        assert_eq!(m[[2, 2]], 11.0);
    }

    #[test]
    fn can_compare_two_equal_matrix3() {
        let m1 = Matrix3::from([[1.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]]);
        let m2 = Matrix3::from([[1.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]]);

        assert_eq!(m1, m2);
    }

    #[test]
    fn can_compare_two_similar_but_different_matrix3() {
        let m1 = Matrix3::from([[1.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]]);
        let m2 = Matrix3::from([[1.001, 2.0, 3.0], [5.5, 6.5, 7.55], [9.0, 10.0, 11.0]]);

        assert_ne!(m1, m2);
    }

    #[test]
    fn submatrix_from_matrix3_is_matrix2() {
        let m = Matrix3::from([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);

        let m_sub = m.submatrix(0, 2);

        let expected = Matrix2::from([[-3.0, 2.0], [0.0, 6.0]]);

        assert_eq!(m_sub, expected);
    }

    #[test]
    fn minor_of_matrix3() {
        let m = Matrix3::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        let m_sub = m.submatrix(1, 0);

        assert_approx_eq!(f64, m_sub.determinant(), m.minor(1, 0));
        assert_approx_eq!(f64, m.minor(1, 0), 25.0);
    }

    #[test]
    fn cofactor_of_matrix3() {
        let m = Matrix3::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        let m_cof = m.cofactor(0, 0);
        let m_min = m.minor(0, 0);

        assert_approx_eq!(f64, m_cof, -12.0);
        assert_approx_eq!(f64, m_cof, m_min);

        let m_cof = m.cofactor(1, 0);
        let m_min = m.minor(1, 0);

        assert_approx_eq!(f64, m_cof, -25.0);
        assert_approx_eq!(f64, m_cof, -m_min);

        let m = Matrix3::from([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

        assert_approx_eq!(f64, m.cofactor(0, 0), 56.0);
        assert_approx_eq!(f64, m.cofactor(0, 1), 12.0);
        assert_approx_eq!(f64, m.cofactor(0, 2), -46.0);
    }

    #[test]
    fn determinant_of_matrix3() {
        let m = Matrix3::from([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

        assert_approx_eq!(f64, m.determinant(), -196.0);
    }
}
