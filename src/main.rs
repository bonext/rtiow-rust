extern crate image;
use std::env;
mod vector;
mod ray;

fn hit_sphere(center: &vector::Vector3f, radius: f32, r: &ray::Ray) -> bool{
    let oc = r.origin - *center;
    let a = r.direction.dot(r.direction);
    let b = 2.0 * oc.dot(r.direction);
    let c = oc.dot(oc) - radius * radius;
    let dscr = b * b - 4.0 * a * c;
    return dscr > 0.0;
}

fn compute_color(r: &ray::Ray) -> vector::Vector3f {
    let sphere_center = vector::Vector3f{x: 0.0, y: 0.0, z: -1.0};
    let sphere_radius: f32 = 0.5;
    if hit_sphere(&sphere_center, sphere_radius, &r) {
        return vector::Vector3f{x: 1.0, y:0.0, z:0.0};
    }
    else {
        let unit_dir = r.direction.normalized();
        let t = 0.5 * (unit_dir.y + 1.0);
        return vector::Vector3f{x: 1.0, y: 1.0, z: 1.0} * (1.0 - t) + vector::Vector3f{x: 0.5, y: 0.7, z:1.0} * t;
    }
}

fn main() {
	let args: Vec<String> = env::args().collect();

    let height = 240;
    let width = 480;

    // TODO: constructing vectors like this is a pain
    let llc = vector::Vector3f{x: -2.0, y: -1.0, z: -1.0};
    let horizontal = vector::Vector3f{x: 4.0, y: 0.0, z: 0.0};
    let vertical = vector::Vector3f{x: 0.0, y: 2.0, z: 0.0};
    let origin = vector::Vector3f{x: 0.0, y: 0.0, z:0.0};

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
