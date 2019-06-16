use super::vector::Vector3f;
use super::ray::Ray;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vector3f,
    pub normal: Vector3f
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool;
}
