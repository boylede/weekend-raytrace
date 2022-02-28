use indicatif::ProgressBar;

mod camera;
/// image buffer
mod image;
/// numerical primatives like Vector and Ray
mod numbers;

use camera::*;
use image::*;

use crate::numbers::Samples;

const HEIGHT: usize = 144;
const WIDTH: usize = 256;
const PIXEL_COUNT: usize = HEIGHT * WIDTH;
const SAMPLES: usize = 100;

fn main() {
    let bar = ProgressBar::new(PIXEL_COUNT as u64);
    let mut buffer = ImageBuffer::new(WIDTH, HEIGHT);
    let camera = Camera::new(2.0 * (16.0 / 9.0), 2.0);

    let world = World::new();

    let pixels = camera
        .rays(WIDTH, HEIGHT)
        .map(|(_uv, ray)| {
            let samples: Samples = (0..SAMPLES)
                .map(|_| ray.perturb(0.005, 0.005).cast(&world, 0).sample())
                .sum();
            let color = samples.to_color();
            bar.inc(1);
            color.truncate()
        })
        .collect();
    buffer.swap_pixels(pixels);
    bar.finish();
    let out_string = buffer.serialize_ppm();
    println!("{}", out_string);
}
