use float_cmp::approx_eq;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::core::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1.0 }
    }
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

    fn add(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
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
            w: self.w - rhs.w,
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
            w: self.w - rhs.w,
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
            w: 1.0,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

#[cfg(test)]
mod point_tests {
    use super::*;
    #[test]
    fn sum_point_vector() {
        let point = Point::new(3.0, -2.0, 5.0);

        let vector = Vector::new(-2.0, 3.0, 1.0);

        let expected = Point::new(1.0, 1.0, 6.0);

        assert_eq!(point + vector, expected)
    }

    #[test]
    fn sub_point_point() {
        let p1 = Point::new(3.0, 2.0, 1.0);

        let p2 = Point::new(5.0, 6.0, 7.0);

        let expected = Vector::new(-2.0, -4.0, -6.0);

        assert_eq!(p1 - p2, expected);
    }

    #[test]
    fn sub_point_vector() {
        let p1 = Point::new(3.0, 2.0, 1.0);

        let v1 = Vector::new(5.0, 6.0, 7.0);

        let expected = Point::new(-2.0, -4.0, -6.0);

        assert_eq!(p1 - v1, expected);
    }

    #[test]
    fn negate_point() {
        let point = Point::new(1.0, -2.0, 3.0);

        let expected = Point::new(-1.0, 2.0, -3.0);

        assert_eq!(-point, expected);
    }

    #[test]
    fn multiply_point_by_f64() {
        let point = Point::new(1.0, -2.0, 3.0);

        let expected = Point::new(3.0, -6.0, 9.0);

        assert_eq!(point * 3.0, expected);
    }

    #[test]
    fn divide_point_by_f64() {
        let point = Point::new(1.0, -2.0, 3.0);

        let expected = Point::new(1.0 / 3.0, -2.0 / 3.0, 1.0);

        assert_eq!(point / 3.0, expected);
    }
}
