use crate::vectors::{Vec3, Point3};
struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    fn new() -> Self {
        Ray {
            origin: Point3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(0.0, 0.0, 0.0),
        }
    }
    fn from(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}