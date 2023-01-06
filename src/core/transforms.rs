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
}

#[cfg(test)]
pub mod transforms_tests {
    use crate::core::point::Point;

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
}
