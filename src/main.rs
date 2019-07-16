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
use materials::{Lambertian, Metal, Dielectric};

fn compute_color(r: Ray, world: &HitableList, depth_limit: u16) -> Vector3f {
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
            match hit.material.scatter(hit){
                Some((scattered_ray, attenuation)) => {
                    compute_color(scattered_ray, world, depth_limit - 1) * attenuation
                },
                None => default_color,
            }
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

    let world = HitableList{
        items: vec![
            // huge green world
            Box::new(Sphere::new(
                Vector3f::new(0.0, -100.5, -1.0), 
                100.0, 
                Lambertian{
                    albedo: Vector3f::new(0.8, 0.8, 0.0)
                }
            )),
            // Box::new(Sphere::new(
            //     Vector3f::new(0.0, 0.0, -1.0), 
            //     0.5, 
            //     Dielectric{
            //         ref_idx: 1.5
            //     }
            // )),
            // Box::new(Sphere::new(
            //     Vector3f::new(0.5, 0.0, -1.5), 
            //     0.5, 
            //     Lambertian{
            //         albedo: Vector3f::new(0.1, 0.2, 0.5)
            //     }
            // )),
            Box::new(Sphere::new(
                Vector3f::new(0.0, 0.0, -1.0), 
                0.5, 
                Lambertian{
                    albedo: Vector3f::new(0.1, 0.2, 0.5)
                }
            )),
            Box::new(Sphere::new(
                Vector3f::new(1.0, 0.0, -1.0), 
                0.5, 
                Metal{
                    albedo: Vector3f::new(0.8, 0.6, 0.2)
                }
            )),
            Box::new(Sphere::new(
                Vector3f::new(-1.0, 0.0, -1.0), 
                0.5, Dielectric{ref_idx: 1.5}
            )),
        ]
    };

    let cam = Camera::default();
    let mut img = image::ImageBuffer::new(width, height);
    let mut du = 0.0;
    let mut dv = 0.0;
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let mut color = Vector3f::new(0.0, 0.0, 0.0);
        for _ in 0..antialiasing_samples {
            if antialiasing_samples > 1 {
                du = rand::random::<f32>();
                dv = rand::random::<f32>();
            }

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
