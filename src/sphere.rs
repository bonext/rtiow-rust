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

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            center: Vector3f::new(0.0, 0.0, 0.0),
            radius: 1.0,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = 2.0 * oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let dscr = b * b - 4.0 * a * c;
        if dscr > 0.0 {
            let r_lo = (-b - dscr.sqrt()) / 2.0 / a;
            let r_hi = (-b + dscr.sqrt()) / 2.0 / a;
            if (t_min <= r_lo) && (r_lo < t_max) {
                return Some(HitRecord{
                    t: r_lo,
                    p: r.at(r_lo),
                    normal: (r.at(r_lo) - self.center).normalized(),
                    in_ray: r
                })
            }
            if (t_min <= r_hi) && (r_hi < t_max) {
                return Some(HitRecord{
                    t: r_hi,
                    p: r.at(r_hi),
                    normal: (r.at(r_hi) - self.center).normalized(),
                    in_ray: r
                })
            }
        }
        return None;
    }
}

