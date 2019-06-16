extern crate image;

use std::{env, f32};

mod vector;
mod ray;
mod sphere;
mod hitable;
use vector::Vector3f;
use ray::Ray;
use sphere::Sphere;
use hitable::{HitRecord, Hitable, HitableList};

fn compute_color<'a, T: Hitable>(r: &Ray, world: &'a HitableList<'a, T>) -> Vector3f {
    let hit_t_min = 0.0;
    let hit_t_max = std::f32::MAX;

     let mut temp_rec = HitRecord {
         t: 0.0,
         p: Vector3f::new(0.0, 0.0, 0.0),
         normal: Vector3f::new(0.0, 0.0, 0.0)
     };

    if world.hit(r, hit_t_min, hit_t_max, &mut temp_rec) {
        return (temp_rec.normal + Vector3f::new(1.0, 1.0, 1.0)) * 0.5;
    }

    else {
        let unit_dir = r.direction.normalized();
        let t = 0.5 * (unit_dir.y + 1.0);
        return Vector3f::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3f::new(0.5, 0.7, 1.0) * t;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let height = 240;
    let width = 480;

    let llc = Vector3f::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3f::new(4.0, 0.0, 0.0);
    let vertical = Vector3f::new(0.0, 2.0, 0.0);
    let origin = Vector3f::new(0.0, 0.0, 0.0);

    let spheres = vec![
        Sphere::new(
            Vector3f::new(0.0, 0.0, -1.0), 0.5
        ),
        Sphere::new(
            Vector3f::new(0.0, -100.5, -1.0), 100.0
        )
    ];
    let world = HitableList::from(&spheres[..]);

    let mut img = image::ImageBuffer::new(width, height);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let u = x as f32 / width as f32;
        let v = (height - y) as f32 / height as f32;

        let r = Ray::new(origin, llc + horizontal * u + vertical * v);

        let color = compute_color(&r, &world);

        *pixel = image::Rgb([
            (255.99 * color.x) as u8,
            (255.99 * color.y) as u8,
            (255.99 * color.z) as u8
        ]);
    }

    let out_fname = &args[1];
	img.save(out_fname).unwrap();
}
