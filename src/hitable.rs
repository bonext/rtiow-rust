use super::vector::Vector3f;
use super::ray::Ray;

pub struct HitRecord {
    t: f32,
    p: Vector3f,
    normal: Vector3f
};

pub trait Hitable {
    pub fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> HitRecord;
};
