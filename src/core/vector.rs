use std::ops::{Add, Div, Mul, Neg, Sub};

use float_cmp::approx_eq;

use crate::core::point::Point;

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z, w: 0.0 }
    }
}

impl Vector {
    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();

        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
            w: 0.0,
        }
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
            w: 0.0,
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        approx_eq!(f64, self.x, other.x)
            && approx_eq!(f64, self.y, other.y)
            && approx_eq!(f64, self.z, other.z)
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: 0.0,
        }
    }
}

impl Mul for Vector {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: 0.0,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

#[cfg(test)]
mod point_tests {
    use super::*;

    #[test]
    fn sum_vector_vector() {
        let v1 = Vector::new(3.0, -2.0, 5.0);
        let v2 = Vector::new(-2.0, 3.0, 1.0);

        let expected = Vector::new(1.0, 1.0, 6.0);

        assert_eq!(v1 + v2, expected)
    }

    #[test]
    fn sum_vector_point() {
        let vector = Vector::new(-2.0, 3.0, 1.0);

        let point = Point::new(3.0, -2.0, 5.0);

        let expected = Point::new(1.0, 1.0, 6.0);

        assert_eq!(vector + point, expected)
    }

    #[test]
    fn sub_vector_vector() {
        let v1 = Vector::new(3.0, 2.0, 1.0);

        let v2 = Vector::new(5.0, 6.0, 7.0);

        let expected = Vector::new(-2.0, -4.0, -6.0);

        assert_eq!(v1 - v2, expected);
    }

    #[test]
    fn negate_vector() {
        let point = Vector::new(1.0, -2.0, 3.0);

        let expected = Vector::new(-1.0, 2.0, -3.0);

        assert_eq!(-point, expected);
    }

    #[test]
    fn dot_product() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(v1 * v2, 20.0);
    }

    #[test]
    fn multiply_vector_by_f64() {
        let vector = Vector::new(1.0, -2.0, 3.0);

        let expected = Vector::new(3.0, -6.0, 9.0);

        assert_eq!(vector * 3.0, expected);
    }

    #[test]
    fn divide_vector_by_f64() {
        let vector = Vector::new(1.0, -2.0, 3.0);

        let expected = Vector::new(1.0 / 3.0, -2.0 / 3.0, 1.0);

        assert_eq!(vector / 3.0, expected);
    }

    #[test]
    fn magnitude() {
        let mut v = Vector::new(0.0, 1.0, 0.0);

        assert_eq!(v.magnitude(), 1.0);

        v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(v.magnitude(), f64::sqrt(14.0));

        v = Vector::new(-1.0, -2.0, -3.0);

        assert_eq!(v.magnitude(), f64::sqrt(14.0));
    }

    #[test]
    fn normalization() {
        let mut v = Vector::new(4.0, 0.0, 0.0);

        assert_eq!(v.normalize(), Vector::new(1.0, 0.0, 0.0));

        v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(
            v.normalize(),
            Vector::new(
                1.0 / f64::sqrt(14.0),
                2.0 / f64::sqrt(14.0),
                3.0 / f64::sqrt(14.0)
            )
        );
    }

    #[test]
    fn cross_product() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        let expected = Vector::new(-1.0, 2.0, -1.0);

        assert_eq!(v1.cross(&v2), expected);
        assert_eq!(v2.cross(&v1), -expected);
    }
}
