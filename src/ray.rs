use crate::core::{point::Point, vector::Vector};

pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn from(origin: Point, direction: Vector) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, distance: f64) -> Point {
        self.origin + self.direction * distance
    }
}

#[cfg(test)]
pub mod ray_tests {
    use crate::core::{point::Point, vector::Vector};

    use super::Ray;

    #[test]
    fn compute_point_from_ray_at_distance() {
        let r = Ray::from(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));

        assert_eq!(Point::new(2.0, 3.0, 4.0), r.at(0.0));
        assert_eq!(Point::new(3.0, 3.0, 4.0), r.at(1.0));
        assert_eq!(Point::new(1.0, 3.0, 4.0), r.at(-1.0));
        assert_eq!(Point::new(4.5, 3.0, 4.0), r.at(2.5));
    }
}
