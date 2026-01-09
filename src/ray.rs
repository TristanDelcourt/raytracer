use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(o: Vec3, d: Vec3) -> Self {
        Self {
            origin: o,
            direction: d,
        }
    }

    pub fn at(self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
