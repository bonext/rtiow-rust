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
        let r = x as f32 / width as f32;
        let g = (height - y) as f32 / height as f32;
        let b = 0.2;
        *pixel = image::Rgb([
            (255.99 * r) as u8, 
            (255.99 * g) as u8, 
            (255.99 * b) as u8
        ]);
    }

    let out_fname = &args[1];
	img.save(out_fname).unwrap();
}
