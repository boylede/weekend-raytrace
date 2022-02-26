use indicatif::ProgressBar;

/// image buffer
mod image;
/// numerical primatives like Vector and Ray
mod numbers;
mod camera;

use image::*;
use camera::*;

const HEIGHT: usize = 144;
const WIDTH: usize = 256;
const PIXEL_COUNT: usize = HEIGHT * WIDTH;

fn main() {
    let bar = ProgressBar::new(PIXEL_COUNT as u64);
    let mut buffer = ImageBuffer::new(WIDTH, HEIGHT);
    let camera = Camera::new(2.0 * (16.0/9.0), 2.0);
    let pixels = camera.rays(WIDTH, HEIGHT).map(|(_uv,ray)| {
        bar.inc(1);
        let color = ray.cast(());
        color.truncate()
    }).collect();
    buffer.swap_pixels(pixels);
    bar.finish();
    let out_string = buffer.serialize_ppm();
    println!("{}", out_string);
}




