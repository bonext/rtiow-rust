extern crate image;
extern crate rand;

use std::{env, f32};

mod camera;
mod hitable;
mod materials;
mod ray;
mod sphere;
mod vector;
use camera::Camera;
use vector::Vector3f;
use ray::Ray;
use sphere::Sphere;
use hitable::{Hitable, HitableList};
use materials::Lambertian;

fn compute_color<'a, T: Hitable>(r: Ray, world: &'a HitableList<'a, T>, depth_limit: u16) -> Vector3f {
    let unit_dir = r.direction.normalized();
    let t = 0.5 * (unit_dir.y + 1.0);
    let default_color = Vector3f::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3f::new(0.5, 0.7, 1.0) * t;

    if depth_limit == 0 {
        return default_color;
    }

    let hit_t_min = 0.0001;
    let hit_t_max = std::f32::MAX;

    match world.hit(r, hit_t_min, hit_t_max) {
        Some(hit) => {
            let target = hit.p + hit.normal + Vector3f::random_unit();
            let other_ray = Ray::new(hit.p, target - hit.p);
            compute_color(other_ray, world, depth_limit - 1) * 0.5
        },
        None => default_color,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let height = 240;
    let width = 480;

    let antialiasing_samples = 100;
    let secondary_ray_limit = 8;

    let spheres = vec![
        Sphere::new(
            Vector3f::new(0.0, 0.0, -1.0), 
            0.5, 
            Lambertian{
                attenuation: Vector3f::new(0.5, 0.5, 0.5)
            }
        ),
        Sphere::new(
            Vector3f::new(0.0, -100.5, -1.0), 
            100.0, 
            Lambertian{
                attenuation: Vector3f::new(0.5, 0.5, 0.5)
            }
        )
    ];
    let world = HitableList::from(&spheres[..]);

    let cam = Camera::default();

    let mut img = image::ImageBuffer::new(width, height);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let mut color = Vector3f::new(0.0, 0.0, 0.0);
        for _ in 0..antialiasing_samples {
            let du = rand::random::<f32>();
            let dv = rand::random::<f32>();

            let u = (x as f32 + du)/ width as f32;
            let v = ((height - y) as f32 + dv) / height as f32;

            let r = cam.get_ray(u, v);
            let color_sample = compute_color(r, &world, secondary_ray_limit);
            color = color + color_sample;
        }
        color = color / antialiasing_samples as f32;

        *pixel = image::Rgb([
            (255.99 * color.x.sqrt()) as u8,
            (255.99 * color.y.sqrt()) as u8,
            (255.99 * color.z.sqrt()) as u8
        ]);
    }

    let out_fname = &args[1];
	img.save(out_fname).unwrap();
}
