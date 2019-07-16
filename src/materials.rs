extern crate rand;
use rand::distributions::{Distribution, Uniform};

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

fn reflect(falling: Vector3f, normal: Vector3f) -> Vector3f {
    falling - normal * (2.0 * falling.dot(normal))
}

impl Material for Metal {
    fn scatter(&self, hit: HitRecord) -> Option<(Ray, Vector3f)> {
        // TODO: add fuzz
        let reflected = reflect(hit.in_ray.direction, hit.normal);
        Some((Ray::new(hit.p, reflected), self.albedo))
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Dielectric {
    pub ref_idx: f32
}

fn refract(falling: Vector3f, normal: Vector3f, ni_over_nt: f32) -> Option<Vector3f> {
    let uv = falling.normalized();
    let dt = uv.dot(normal);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        return Some((uv - normal * dt) * ni_over_nt - normal * discriminant.sqrt());
    }
    else {
        return None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter(&self, hit: HitRecord) -> Option<(Ray, Vector3f)> {

        let reflected = Ray::new(hit.p, reflect(hit.in_ray.direction, hit.normal));
        
        let out_normal: Vector3f;
        let ni_over_nt: f32;

        let attenuation = Vector3f::new(1.0, 1.0, 1.0);

        if hit.in_ray.direction.dot(hit.normal) > 0.0 {
            out_normal = - hit.normal;
            ni_over_nt = self.ref_idx;
        }
        else {
            out_normal = hit.normal;
            ni_over_nt = 1.0 / self.ref_idx;
        }
        let cosine = - hit.in_ray.direction.normalized().dot(out_normal);

        match refract(hit.in_ray.direction, out_normal, ni_over_nt){
            Some(refracted) => {
                let p_reflect = schlick(cosine, self.ref_idx);
                // let p_reflect = 0.0;
                let uniform = Uniform::new(0.0, 1.0);
                let p = uniform.sample(&mut rand::thread_rng()) as f32;
                if p < p_reflect {
                    return Some((reflected, attenuation));
                }
                else {
                    return Some((Ray::new(hit.p, refracted), attenuation));
                }
            },
            None => {
                return Some((reflected, attenuation));
            },
        }
    }
}