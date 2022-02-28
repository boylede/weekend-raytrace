use rand::Rng;

use crate::{image::*, numbers::*};

pub struct Camera {
    pos: Vector,
    aim: Vector,
    width: f32,
    height: f32,
    focal: f32,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Camera {
        Camera {
            pos: Vector::new(0.0,0.0,0.0),
            aim: Vector::new(0.0,0.0,-1.0),
            width,
            height,
            focal: 1.0,
        }
    }
    pub fn rays(&self, width: usize, height: usize, samples: usize) -> RayIter {
        let horizontal = Vector::new(self.width, 0.0, 0.0);
        let vertical = Vector::new(0.0, self.height, 0.0);
        let focal_vec = Vector::new(0.0, 0.0, self.focal);
        RayIter {
            camera: self,
            pixel_width: width,
            pixel_height: height,
            horizontal,
            vertical,
            llc: self.pos - (horizontal / 2.0) - (vertical / 2.0) - focal_vec,
            i: 0,
            j: 0,
            samples,
        }
    }
}

pub struct RayIter<'a> {
    camera: &'a Camera,
    pixel_width: usize,
    pixel_height: usize,
    horizontal: Vector,
    vertical: Vector,
    llc: Vector,
    i: usize,
    j: usize,
    samples: usize
}

pub struct Uv {
    pub u: f32,
    pub v: f32,
}

impl<'a> Iterator for RayIter<'a> {
    type Item = (Uv, Vec<Ray>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.pixel_width && self.j < self.pixel_height {
            let mut rng = rand::thread_rng();
            let u = self.i as f32 / (self.pixel_width as f32 - 1.0);
            let v = (self.pixel_height - self.j) as f32 / (self.pixel_height as f32 - 1.0);
            let uv = Uv { u, v };
            let rays: Vec<Ray> = (0..self.samples).map(|_|{
                let ru = rng.gen::<f32>() / (self.pixel_width as f32 - 1.0);
                let rv = rng.gen::<f32>() / (self.pixel_height as f32 - 1.0);
                let du = (u + ru) * self.horizontal;
                let dv = (v + rv) * self.vertical;
                Ray::new(self.camera.pos, self.llc + du + dv - self.camera.pos)
            }).collect();

            

            self.i += 1;
            if self.i >= self.pixel_width && self.j < self.pixel_height {
                self.j += 1;
                self.i = 0;
            }
            

            Some((uv, rays))
        } else {
            None
        }
    }
}
