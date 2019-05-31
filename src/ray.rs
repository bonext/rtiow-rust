use super::vector::Vector3f;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Ray {
    pub origin: Vector3f,
    pub direction: Vector3f,
}

impl Ray {
    pub fn new(origin: Vector3f, direction: Vector3f) -> Ray {
        Ray{origin: origin, direction: direction}
    }

    pub fn at(&self, t: f32) -> Vector3f {
        self.origin + self.direction * t
    }
}
