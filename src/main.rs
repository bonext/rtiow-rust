extern crate image;
use std::env;
use std::f32;
mod vector;
mod ray;
mod hitable;
mod sphere;
use vector::Vector3f;
use sphere::Sphere;
use hitable::HitRecord;
use hitable::Hitable;

fn compute_color<T: Hitable>(r: &ray::Ray, world: &[T]) -> Vector3f {
    let hit_t_min = 0.0;
    let hit_t_max = std::f32::MAX;

    // let's implement iteration over hitables here at first
    let mut hit_anything = false;
    let mut closest_so_far = hit_t_max;
    let mut temp_rec = HitRecord {
        t: 0.0,
        p: Vector3f::new(0.0, 0.0, 0.0),
        normal: Vector3f::new(0.0, 0.0, 0.0)
    };
    for item in world.iter() {
        if item.hit(r, hit_t_min, closest_so_far, &mut temp_rec) {
            hit_anything = true;
            closest_so_far = temp_rec.t;
            // *record = temp_rec;
        }
    }

    if hit_anything {
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

    let world = vec![
        Sphere::new(
            Vector3f::new(0.0, 0.0, -1.0), 0.5
        ),
        Sphere::new(
            Vector3f::new(0.0, -100.5, -1.0), 100.0
        )
    ];

    let mut img = image::ImageBuffer::new(width, height);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let u = x as f32 / width as f32;
        let v = y as f32 / height as f32;

        let r = ray::Ray::new(origin, llc + horizontal * u + vertical * v);

        let color = compute_color(&r, &world[..]);

        *pixel = image::Rgb([
            (255.99 * color.x) as u8,
            (255.99 * color.y) as u8,
            (255.99 * color.z) as u8
        ]);
    }

    let out_fname = &args[1];
	img.save(out_fname).unwrap();
}
