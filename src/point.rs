use float_cmp::approx_eq;
use std::ops::Add;

use crate::vector::Vector;

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        approx_eq!(f64, self.x, other.x)
            && approx_eq!(f64, self.y, other.y)
            && approx_eq!(f64, self.z, other.z)
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, other: Vector) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[cfg(test)]
mod point_tests {
    use super::*;
    #[test]
    fn sum_point_vector() {
        let point = Point {
            x: 3.0,
            y: -2.0,
            z: 5.0,
        };

        let vector = Vector {
            x: -2.0,
            y: 3.0,
            z: 1.0,
        };

        let expected = Point {
            x: 1.0,
            y: 1.0,
            z: 6.0,
        };

        assert_eq!(point + vector, expected)
    }
}
