use crate::ray::Ray;
use crate::vector::Vector3f;
use crate::hitable::HitRecord;

pub trait Material {
    fn scatter(&self, hit: HitRecord) -> Option<(Ray, Vector3f)>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Lambertian {
    pub albedo: Vector3f,
}

impl Material for Lambertian {
    fn scatter(&self, hit: HitRecord) -> Option<(Ray, Vector3f)> {
        let scatter_position = hit.p + hit.normal + Vector3f::random_unit();
        let scattered_ray = Ray::new(hit.p, scatter_position - hit.p);
        return Some((scattered_ray, self.albedo));
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Metal {
    pub albedo: Vector3f
}

impl Material for Metal {
    fn scatter(&self, hit: HitRecord) -> Option<(Ray, Vector3f)> {
        let reflected = hit.in_ray.direction - hit.normal * (2.0 * hit.in_ray.direction.dot(hit.normal));
        Some((Ray::new(hit.p, reflected), self.albedo))
    }
}