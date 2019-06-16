use crate::hitable::Hitable;
use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vector::Vector3f;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Vector3f,
    radius: f32
}

impl Sphere {
    pub fn new(center: Vector3f, radius: f32) -> Sphere {
        Sphere{center, radius}
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = 2.0 * oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let dscr = b * b - 4.0 * a * c;
        if dscr > 0.0 {
            let r_lo = (-b - dscr.sqrt()) / 2.0 / a;
            let r_hi = (-b + dscr.sqrt()) / 2.0 / a;
            if (t_min <= r_lo) && (r_lo < t_max) {
                rec.t = r_lo;
                rec.p = r.at(r_lo);
                rec.normal = (r.at(r_lo) - self.center).normalized();
                return true;
            }
            if (t_min <= r_hi) && (r_hi < t_max) {
                rec.t = r_hi;
                rec.p = r.at(r_hi);
                rec.normal = (r.at(r_hi) - self.center).normalized();
                return true;
            }
        }
        return false;
    }
}

