use indicatif::ProgressBar;
use rayon::prelude::*;

mod camera;
/// image buffer
mod image;
/// numerical primatives like Vector and Ray
mod numbers;

/// for physical things to be rendered
mod world;

use camera::*;
use image::*;

use crate::{numbers::Samples, world::World};

const WIDTH: usize = 256;
const HEIGHT: usize = WIDTH / 16 * 9;
const PIXEL_COUNT: usize = HEIGHT * WIDTH;
const SAMPLES: usize = 100;
const MAX_BOUNCES: usize = 50;

fn main() {
    let bar = ProgressBar::new(PIXEL_COUNT as u64);
    let mut buffer = ImageBuffer::new(WIDTH, HEIGHT);
    let camera = Camera::new(2.0 * (16.0 / 9.0), 2.0);

    let world = World::new();

    let pixels = camera
        .rays(WIDTH, HEIGHT, SAMPLES)
        .map(|(_uv, rays)| {
            let samples: Samples = rays.par_iter()
                .map(|ray| {
                    ray.cast(&world, MAX_BOUNCES).sample()
                })
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
