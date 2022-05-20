use float_cmp::approx_eq;
use std::ops::{Add, Neg, Sub};

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

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
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

    #[test]
    fn sub_point_point() {
        let p1 = Point {
            x: 3.0,
            y: 2.0,
            z: 1.0,
        };

        let p2 = Point {
            x: 5.0,
            y: 6.0,
            z: 7.0,
        };

        let expected = Vector {
            x: -2.0,
            y: -4.0,
            z: -6.0,
        };

        assert_eq!(p1 - p2, expected);
    }

    #[test]
    fn sub_point_vector() {
        let p1 = Point {
            x: 3.0,
            y: 2.0,
            z: 1.0,
        };

        let v1 = Vector {
            x: 5.0,
            y: 6.0,
            z: 7.0,
        };

        let expected = Point {
            x: -2.0,
            y: -4.0,
            z: -6.0,
        };

        assert_eq!(p1 - v1, expected);
    }

    #[test]
    fn negate_point() {
        let point = Point {
            x: 1.0,
            y: -2.0,
            z: 3.0,
        };

        let expected = Point {
            x: -1.0,
            y: 2.0,
            z: -3.0,
        };

        assert_eq!(-point, expected);
    }
}
