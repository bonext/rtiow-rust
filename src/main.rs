extern crate image;
extern crate rand;

use std::{env, f32};

mod camera;
mod vector;
mod ray;
mod sphere;
mod hitable;
use camera::Camera;
use vector::Vector3f;
use ray::Ray;
use sphere::Sphere;
use hitable::{HitRecord, Hitable, HitableList};

use rand::Rng;

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
    let num_samples = 100;

    let spheres = vec![
        Sphere::new(
            Vector3f::new(0.0, 0.0, -1.0), 0.5
        ),
        Sphere::new(
            Vector3f::new(0.0, -100.5, -1.0), 100.0
        )
    ];
    let world = HitableList::from(&spheres[..]);

    let cam = Camera::default();

    let mut img = image::ImageBuffer::new(width, height);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let mut color = Vector3f::new(0.0, 0.0, 0.0);
        for _ in (0..num_samples) {
            let du = rand::random::<f32>();
            let dv = rand::random::<f32>();

            let u = (x as f32 + du)/ width as f32;
            let v = ((height - y) as f32 + dv) / height as f32;

            let r = cam.get_ray(u, v);
            let color_sample = compute_color(&r, &world);
            color = color + color_sample;
        }
        color = color / num_samples as f32;

        *pixel = image::Rgb([
            (255.99 * color.x) as u8,
            (255.99 * color.y) as u8,
            (255.99 * color.z) as u8
        ]);
    }

    let out_fname = &args[1];
	img.save(out_fname).unwrap();
}
