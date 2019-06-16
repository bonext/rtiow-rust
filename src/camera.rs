use super::ray::Ray;
use super::vector::Vector3f;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    origin: Vector3f,
    llc: Vector3f,
    vertical: Vector3f,
    horizontal: Vector3f,
}

impl Camera {
    // TODO: new()
    pub fn default() -> Camera {
        Camera {
            origin: Vector3f::new(0.0, 0.0, 0.0),
            llc: Vector3f::new(-2.0, -1.0, -1.0),
            horizontal: Vector3f::new(4.0, 0.0, 0.0),
            vertical: Vector3f::new(0.0, 2.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.llc + self.horizontal * u + self.vertical * v)
    }
}

