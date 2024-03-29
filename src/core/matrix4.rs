use std::ops::{Index, IndexMut, Mul};

use crate::utils::utils::FLOAT_MARGIN;
use float_cmp::{approx_eq, ApproxEq};

use super::{matrix3::Matrix3, point::Point, vector::Vector};

#[derive(Debug, Copy, Clone)]
pub struct Matrix4 {
    pub data: [[f64; 4]; 4],
}

impl Matrix4 {
    pub const fn new() -> Matrix4 {
        Matrix4::from([
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ])
    }

    pub const fn from(data: [[f64; 4]; 4]) -> Matrix4 {
        Matrix4 { data }
    }

    pub fn transposed(&self) -> Matrix4 {
        let mut m = Matrix4::new();

        for row in 0..4 {
            for col in 0..4 {
                m[[row, col]] = self[[col, row]]
            }
        }
        m
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix3 {
        assert!(row <= 4 && col <= 4);

        let mut m = Matrix3::new();

        for _row in 0..4 {
            for _col in 0..4 {
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

        for col in 0..4 {
            det += self[[0, col]] * self.cofactor(0, col);
        }

        det
    }

    pub fn is_invertible(&self) -> bool {
        if approx_eq!(f64, self.determinant(), 0.0) {
            false
        } else {
            true
        }
    }

    pub fn inverse(&self) -> Matrix4 {
        assert!(self.is_invertible());

        let mut m = Matrix4::new();
        let det = self.determinant();

        for row in 0..4 {
            for col in 0..4 {
                let c = self.cofactor(row, col);

                // already transposing here by indexing with col, row
                m[[col, row]] = c / det;
            }
        }

        m
    }

    const IDENTITY: Matrix4 = Matrix4::from([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);
}

impl Index<[usize; 2]> for Matrix4 {
    type Output = f64;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let [i, j] = index;

        &self.data[i][j]
    }
}

impl IndexMut<[usize; 2]> for Matrix4 {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let [i, j] = index;

        &mut self.data[i][j]
    }
}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Self) -> bool {
        self.data
            .iter()
            .flatten()
            .zip(other.data.iter().flatten())
            .all(|(&a, &b)| a.approx_eq(b, FLOAT_MARGIN))
    }
}

impl Eq for Matrix4 {}

impl Mul for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut m = Matrix4::new();

        for row in 0..4 {
            for col in 0..4 {
                m[[row, col]] = self[[row, 0]] * rhs[[0, col]]
                    + self[[row, 1]] * rhs[[1, col]]
                    + self[[row, 2]] * rhs[[2, col]]
                    + self[[row, 3]] * rhs[[3, col]]
            }
        }
        m
    }
}

impl Mul<Point> for Matrix4 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Self::Output::new(
            self[[0, 0]] * rhs.x
                + self[[0, 1]] * rhs.y
                + self[[0, 2]] * rhs.z
                + self[[0, 3]] * rhs.w,
            self[[1, 0]] * rhs.x
                + self[[1, 1]] * rhs.y
                + self[[1, 2]] * rhs.z
                + self[[1, 3]] * rhs.w,
            self[[2, 0]] * rhs.x
                + self[[2, 1]] * rhs.y
                + self[[2, 2]] * rhs.z
                + self[[2, 3]] * rhs.w,
        )
    }
}

impl Mul<Vector> for Matrix4 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Self::Output::new(
            self[[0, 0]] * rhs.x
                + self[[0, 1]] * rhs.y
                + self[[0, 2]] * rhs.z
                + self[[0, 3]] * rhs.w,
            self[[1, 0]] * rhs.x
                + self[[1, 1]] * rhs.y
                + self[[1, 2]] * rhs.z
                + self[[1, 3]] * rhs.w,
            self[[2, 0]] * rhs.x
                + self[[2, 1]] * rhs.y
                + self[[2, 2]] * rhs.z
                + self[[2, 3]] * rhs.w,
        )
    }
}

#[cfg(test)]
pub mod matrix_tests {
    use float_cmp::assert_approx_eq;

    use crate::utils::utils::EPSILON;

    use super::*;

    #[test]
    fn create_matrix4() {
        let m = Matrix4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(m[[0, 0]], 1.0);
        assert_eq!(m[[1, 1]], 6.5);
        assert_eq!(m[[2, 2]], 11.0);
        assert_eq!(m[[3, 3]], 16.5);
    }

    #[test]
    fn can_compare_two_equal_matrix4() {
        let m1 = Matrix4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        let m2 = Matrix4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(m1, m2);
    }

    #[test]
    fn can_compare_two_similar_but_different_matrix4() {
        let m1 = Matrix4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        let m2 = Matrix4::from([
            [1.0001, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_ne!(m1, m2);
    }

    #[test]
    fn multiply_two_matrix4() {
        let m1 = Matrix4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let m2 = Matrix4::from([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        let expected = Matrix4::from([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);

        assert_eq!(m1 * m2, expected);
    }

    #[test]
    fn create_matrix4_identity() {
        let expected = Matrix4::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_eq!(Matrix4::IDENTITY, expected)
    }

    #[test]
    fn multiply_matrix4_by_identity_yields_same_result() {
        let m = Matrix4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(m * Matrix4::IDENTITY, m);
    }

    #[test]
    fn multiply_matrix4_by_point() {
        let m = Matrix4::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let p = Point::new(1.0, 2.0, 3.0);

        let actual = m * p;
        let expected = Point::new(18.0, 24.0, 33.0);

        assert_eq!(expected, actual);
    }

    #[test]
    fn multiply_matrix4_by_vector() {
        let m = Matrix4::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let v = Vector::new(1.0, 2.0, 3.0);

        let actual = m * v;
        let expected = Vector::new(14.0, 22.0, 32.0);

        assert_eq!(expected, actual);
    }

    #[test]
    fn transpose_matrix4() {
        let m = Matrix4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let expected = Matrix4::from([
            [1.0, 5.0, 9.0, 5.0],
            [2.0, 6.0, 8.0, 4.0],
            [3.0, 7.0, 7.0, 3.0],
            [4.0, 8.0, 6.0, 2.0],
        ]);

        assert_eq!(m.transposed(), expected);
    }

    #[test]
    fn transpose_of_identity_is_identity() {
        assert_eq!(Matrix4::IDENTITY.transposed(), Matrix4::IDENTITY);
    }

    #[test]
    fn submatrix_from_matrix4_is_matrix3() {
        let m = Matrix4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let m_sub = m.submatrix(2, 1);

        let expected = Matrix3::from([[1.0, 3.0, 4.0], [5.0, 7.0, 8.0], [5.0, 3.0, 2.0]]);

        assert_eq!(m_sub, expected);
    }

    #[test]
    fn minor_of_matrix4() {
        let m = Matrix4::from([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);

        assert_approx_eq!(f64, m.minor(0, 0), 690.0, epsilon = EPSILON);
        assert_approx_eq!(f64, m.minor(1, 2), 431.0, epsilon = EPSILON);
        assert_approx_eq!(f64, m.minor(2, 3), 207.0, epsilon = EPSILON);
    }

    #[test]
    fn cofactor_of_matrix4() {
        let m = Matrix4::from([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);

        assert_approx_eq!(f64, m.cofactor(0, 0), 690.0, epsilon = EPSILON);
        assert_approx_eq!(f64, m.cofactor(0, 1), 447.0, epsilon = EPSILON);
        assert_approx_eq!(f64, m.cofactor(0, 2), 210.0, epsilon = EPSILON);
        assert_approx_eq!(f64, m.cofactor(0, 3), 51.0, epsilon = EPSILON);
    }

    #[test]
    fn determinant_of_matrix4() {
        let m = Matrix4::from([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);

        assert_approx_eq!(f64, m.determinant(), -4071.0, epsilon = EPSILON);
    }

    #[test]
    fn check_matrix4_is_invertible() {
        let m = Matrix4::from([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);

        assert_approx_eq!(f64, m.determinant(), -2120.0, epsilon = EPSILON);
        assert_eq!(m.is_invertible(), true);

        let m = Matrix4::from([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_approx_eq!(f64, m.determinant(), 0.0, epsilon = EPSILON);
        assert_eq!(m.is_invertible(), false);
    }

    #[test]
    fn invert_matrix4() {
        let m = Matrix4::from([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);

        let expected = Matrix4::from([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);

        assert_eq!(m.inverse(), expected);

        let m = Matrix4::from([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);

        let expected = Matrix4::from([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);

        assert_eq!(m.inverse(), expected);

        let m = Matrix4::from([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);

        let expected = Matrix4::from([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333],
        ]);

        assert_eq!(m.inverse(), expected);
    }

    #[test]
    fn multiply_product_by_inverse() {
        let a = Matrix4::from([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);

        let b = Matrix4::from([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);

        let product = a * b;

        // note that order matters in matrix multiplication
        assert_eq!(product * b.inverse(), a);
    }
}
