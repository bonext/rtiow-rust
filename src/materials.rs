use crate::ray::Ray;
use crate::vector::Vector3f;
use crate::hitable::HitRecord;

pub trait Material {
    fn scatter(&self, hit: HitRecord) -> Option<(Ray, Vector3f)>;
}

pub struct Lambertian {
}

impl Material for Lambertian {
    fn scatter(&self, hit: HitRecord) -> Option<(Ray, Vector3f)> {
        return None;
    }
}
