use std::ops::{Add, Mul};

use indicatif::ProgressBar;

use crate::numbers::Color;

/// RGB format with channel values from 0-255
/// expected to be gamma-corrected
#[derive(Copy, Clone)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Add for Pixel {
    type Output = Pixel;
    fn add(self, rhs: Pixel) -> Pixel {
        Pixel { r: self.r + rhs.r, g: self.g + rhs.g, b: self.b + rhs.b }
    }
}

impl Pixel {
    /// produce a pixel value from gamma-corrected channels
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel { r, g, b }
    }
    pub fn serialize_ppm(&self) -> String {
        format!("{} {} {}\n", self.r, self.g, self.b)
    }
}

/// simple image buffer with width and height
pub struct ImageBuffer {
    width: usize,
    height: usize,
    buffer: Vec<Pixel>,
}

impl ImageBuffer {
    pub fn new(width: usize, height: usize) -> ImageBuffer {
        let mut buffer = Vec::with_capacity(width * height);
        let max_width = width as f32 - 1.0;
        let max_height = height as f32 - 1.0;

        for y in 0..height {
            for x in 0..width {
                let r = (x as f32) / max_width;
                let g = (y as f32) / max_height;
                let b = 0.25;
                let c = Color::new(r,g,b);
                buffer.push(c.to_pixel());
            }
        }
        ImageBuffer {
            width,
            height,
            buffer,
        }
    }
    pub fn swap_pixels(&mut self, mut other: Vec<Pixel>) {
        if other.len() == self.buffer.len() {
            self.buffer.swap_with_slice(&mut other)
        }
    }

    pub fn serialize_ppm(&self) -> String {
        let bar = ProgressBar::new((self.width * self.height + 1) as u64);

        let pixels: String = self
            .buffer
            .iter()
            .map(|p| {
                bar.inc(1);
                p.serialize_ppm()
            })
            .collect();

        let o = format!("P3\n{} {}\n{}\n{}", self.width, self.height, 255, pixels);
        bar.inc(1);
        bar.finish();
        o
    }
}
