extern crate image;
use std::env;
mod vector;
mod ray;
use vector::Vector3f;

fn hit_sphere(center: &vector::Vector3f, radius: f32, r: &ray::Ray) -> f32{
    let oc = r.origin - *center;
    let a = r.direction.dot(r.direction);
    let b = 2.0 * oc.dot(r.direction);
    let c = oc.dot(oc) - radius * radius;
    let dscr = b * b - 4.0 * a * c;
    if dscr < 0.0 {
        return -1.0;
    }
    else {
        return (-b - dscr.sqrt()) / 2.0 / a;
    }
}

fn compute_color(r: &ray::Ray) -> vector::Vector3f {
    let sphere_center = Vector3f::new(0.0, 0.0, -1.0);
    let sphere_radius: f32 = 0.5;
    let t = hit_sphere(&sphere_center, sphere_radius, &r);
    if t > 0.0 {
        let N = (r.at(t) - sphere_center).normalized();
        return (N + Vector3f::new(1.0, 1.0, 1.0)) * 0.5
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

    // TODO: constructing vectors like this is a pain
    let llc = Vector3f::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3f::new(4.0, 0.0, 0.0);
    let vertical = Vector3f::new(0.0, 2.0, 0.0);
    let origin = Vector3f::new(0.0, 0.0, 0.0);

    let mut img = image::ImageBuffer::new(width, height);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let u = x as f32 / width as f32;
        let v = y as f32 / height as f32;

        let r = ray::Ray::new(origin, llc + horizontal * u + vertical * v);

        let color = compute_color(&r);

        *pixel = image::Rgb([
            (255.99 * color.x) as u8,
            (255.99 * color.y) as u8,
            (255.99 * color.z) as u8
        ]);
    }

    let out_fname = &args[1];
	img.save(out_fname).unwrap();
}
