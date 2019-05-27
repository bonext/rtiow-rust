extern crate image;
use std::env;
mod vector;

fn main() {
	let args: Vec<String> = env::args().collect();

    let height = 240;
    let width = 320;

    let mut img = image::ImageBuffer::new(width, height);

    // fill-in fancy background
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let color = vector::Vector3f{
            x: x as f32 / width as f32,
            y: (height - y) as f32 / height as f32,
            z: 0.2
        };
        *pixel = image::Rgb([
            (255.99 * color.x) as u8, 
            (255.99 * color.y) as u8, 
            (255.99 * color.z) as u8
        ]);
    }

    let out_fname = &args[1];
	img.save(out_fname).unwrap();
}
