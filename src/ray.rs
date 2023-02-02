use crate::vectors::{Vec3, Point3};
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new() -> Self {
        Ray {
            origin: Point3::new(),
            direction: Vec3::new(),
        }
    }

    pub fn from(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}