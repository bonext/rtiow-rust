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

pub struct HitableList<'a, T: Hitable> {
    items: &'a [T]
}

impl<'a, T: Hitable> HitableList<'a, T> {
    pub fn from(items: &'a [T]) -> HitableList<'a, T> {
        HitableList{items}
    }
}

impl<'a, T: Hitable> Hitable for HitableList<'a, T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        // let's implement iteration over hitables here at first
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut temp_rec = HitRecord {
            t: 0.0,
            p: Vector3f::new(0.0, 0.0, 0.0),
            normal: Vector3f::new(0.0, 0.0, 0.0)
        };
        for item in self.items.iter() {
            if item.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *record = temp_rec;
            }
        }
        return hit_anything;
    }
}

