use std::ops::Index;

use float_cmp::approx_eq;

#[derive(Debug, Copy, Clone)]
pub struct Matrix3 {
    pub data: [[f64; 3]; 3],
}

impl Matrix3 {
    pub const fn from(data: [[f64; 3]; 3]) -> Matrix3 {
        Matrix3 { data }
    }
}

impl Index<[usize; 2]> for Matrix3 {
    type Output = f64;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let [i, j] = index;

        &self.data[i][j]
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
}
