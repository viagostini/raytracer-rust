use super::matrix4::Matrix4;

struct Transforms {}

impl Transforms {
    pub fn translation(x: f64, y: f64, z: f64) -> Matrix4 {
        Matrix4::from([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Matrix4 {
        Matrix4::from([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_x(r: f64) -> Matrix4 {
        Matrix4::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, r.cos(), -r.sin(), 0.0],
            [0.0, r.sin(), r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_y(r: f64) -> Matrix4 {
        Matrix4::from([
            [r.cos(), 0.0, r.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-r.sin(), 0.0, r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_z(r: f64) -> Matrix4 {
        Matrix4::from([
            [r.cos(), -r.sin(), 0.0, 0.0],
            [r.sin(), r.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix4 {
        Matrix4::from([
            [1.0, xy, xz, 0.0],
            [yx, 1.0, yz, 0.0],
            [zx, zy, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

#[cfg(test)]
pub mod transforms_tests {
    use std::f64::consts::PI;

    use crate::core::{point::Point, vector::Vector};

    use super::*;

    #[test]
    fn multiplying_point_by_translation() {
        let p = Point::new(-3.0, 4.0, 5.0);
        let t = Transforms::translation(5.0, -3.0, 2.0);

        let actual = t * p;
        let expected = Point::new(2.0, 1.0, 7.0);

        assert_eq!(expected, actual);
    }

    #[test]
    fn multiplying_point_by_inverse_translation() {
        let p = Point::new(-3.0, 4.0, 5.0);
        let t = Transforms::translation(5.0, -3.0, 2.0);
        let inv = t.inverse();

        let actual = inv * p;
        let expected = Point::new(-8.0, 7.0, 3.0);

        assert_eq!(expected, actual);
    }

    #[test]
    fn multiplying_vector_by_scaling() {
        let v = Vector::new(-4.0, 6.0, 8.0);
        let t = Transforms::scaling(2.0, 3.0, 4.0);

        let actual = t * v;
        let expected = Vector::new(-8.0, 18.0, 32.0);

        assert_eq!(expected, actual);
    }

    #[test]
    fn multiplying_vector_by_inverse_scaling() {
        let v = Vector::new(-4.0, 6.0, 8.0);
        let t = Transforms::scaling(2.0, 3.0, 4.0);
        let inv = t.inverse();

        let actual = inv * v;
        let expected = Vector::new(-2.0, 2.0, 2.0);

        assert_eq!(expected, actual);
    }

    #[test]
    fn multiplying_by_negative_scaling_is_reflection() {
        let p = Point::new(2.0, 3.0, 4.0);
        let t = Transforms::scaling(-1.0, 1.0, 1.0);

        let actual = t * p;
        let expected = Point::new(-2.0, 3.0, 4.0);

        assert_eq!(expected, actual);
    }

    #[test]
    fn rotating_tuple_around_x_axis() {
        let p = Point::new(0.0, 1.0, 0.0);

        let half_quarter = Transforms::rotation_x(PI / 4.0);

        let actual = half_quarter * p;
        let expected = Point::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0);

        assert_eq!(expected, actual);

        let full_quarter = Transforms::rotation_x(PI / 2.0);

        let actual = full_quarter * p;
        let expected = Point::new(0.0, 0.0, 1.0);

        assert_eq!(expected, actual);
    }

    #[test]
    fn rotating_tuple_around_x_axis_by_inverse_goes_the_other_way() {
        let p = Point::new(0.0, 1.0, 0.0);

        let half_quarter_inv = Transforms::rotation_x(PI / 4.0).inverse();

        let actual = half_quarter_inv * p;
        let expected = Point::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);

        assert_eq!(expected, actual);
    }

    #[test]
    fn rotating_tuple_around_y_axis() {
        let p = Point::new(0.0, 0.0, 1.0);

        let half_quarter = Transforms::rotation_y(PI / 4.0);

        let actual = half_quarter * p;
        let expected = Point::new(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0);

        assert_eq!(expected, actual);

        let full_quarter = Transforms::rotation_y(PI / 2.0);

        let actual = full_quarter * p;
        let expected = Point::new(1.0, 0.0, 0.0);

        assert_eq!(expected, actual);
    }

    #[test]
    fn rotating_tuple_around_z_axis() {
        let p = Point::new(0.0, 1.0, 0.0);

        let half_quarter = Transforms::rotation_z(PI / 4.0);

        let actual = half_quarter * p;
        let expected = Point::new(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);

        assert_eq!(expected, actual);

        let full_quarter = Transforms::rotation_z(PI / 2.0);

        let actual = full_quarter * p;
        let expected = Point::new(-1.0, 0.0, 0.0);

        assert_eq!(expected, actual);
    }

    #[test]
    fn shearing_tuples() {
        let s = Transforms::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);

        let actual = s * p;
        let expected = Point::new(6.0, 3.0, 4.0);

        assert_eq!(expected, actual);

        let s = Transforms::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);

        let actual = s * p;
        let expected = Point::new(2.0, 5.0, 4.0);

        assert_eq!(expected, actual);

        let s = Transforms::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        let actual = s * p;
        let expected = Point::new(2.0, 7.0, 4.0);

        assert_eq!(expected, actual);

        let s = Transforms::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);

        let actual = s * p;
        let expected = Point::new(2.0, 3.0, 6.0);

        assert_eq!(expected, actual);

        let s = Transforms::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);

        let actual = s * p;
        let expected = Point::new(2.0, 3.0, 7.0);

        assert_eq!(expected, actual);
    }
}
