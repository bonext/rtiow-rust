use super::vector::Vector3f;
use super::ray::Ray;
use super::materials::Material;

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vector3f,
    pub normal: Vector3f,
    pub in_ray: Ray,
    pub material: &'a Material
}

pub trait Hitable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitableList {
    pub items: Vec<Box<dyn Hitable>>,
}

impl Hitable for HitableList {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // let's implement iteration over hitables here at first
        let mut closest_so_far = t_max;
        let mut hit: Option<HitRecord> = None;
        for item in self.items.iter() {
            match item.hit(r, t_min, closest_so_far) {
                Some(record) => {
                    closest_so_far = record.t;
                    hit = Some(record);
                },
                None => {},
            }
        }
        hit
    }
}
