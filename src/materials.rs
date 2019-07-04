use crate::ray::Ray;
use crate::vector::Vector3f;
use crate::hitable::HitRecord;

pub trait Material {
    fn scatter(&self, hit: HitRecord) -> Option<(Ray, Vector3f)>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Lambertian {
    pub attenuation: Vector3f,
}

impl Material for Lambertian {
    fn scatter(&self, hit: HitRecord) -> Option<(Ray, Vector3f)> {
        let scatter_position = hit.p + hit.normal + Vector3f::random_unit();
        let scattered_ray = Ray::new(hit.p, scatter_position - hit.p);
        return Some((scattered_ray, self.attenuation));
    }
}
