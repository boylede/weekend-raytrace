use std::rc::Rc;

use crate::{numbers::*, world::*, material::{Material, Shader}};
use rand::Rng;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

#[derive(Clone)]
pub struct Bounce {
    pub ray: Ray,
    pub attenuation: Color,
}

#[derive(Clone)]
pub struct Hit {
    pub length: f32,
    pub pos: Vector,
    pub normal: Vector,
    pub front: bool,
    pub material: Rc<Material>
}

impl Hit {
    pub fn new(ray: &Ray, length: f32, pos: Vector, mut normal: Vector, material: &Rc<Material>) -> Hit {
        
        let front = ray.direction.dot(&normal) < 0.0;
        if !front {
            normal = -normal;
        }
        let material = material.clone();
        Hit { length, pos, normal, front, material }
    }
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        Ray { origin, direction }
    }
    pub fn at(&self, magnitude: f32) -> Vector {
        self.origin + (magnitude * self.direction)
    }
    pub fn unit(&self) -> Ray {
        let length = self.direction.length();
        Ray {
            origin: self.origin,
            direction: self.direction / length,
        }
    }
    pub fn hit_sphere(&self, center: Vector, radius: f32, near: f32, far: f32, material: &Rc<Material>) -> Option<Hit> {
        let oc: Vector = self.origin - center;
        let a = self.direction.square_length();
        let half_b = oc.dot(&self.direction);
        let c = oc.square_length() - radius * radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            None
        } else {
            let root = {
                let sqrtd = discriminant.sqrt();
                let mut root = (-half_b - sqrtd) / a;
                if root < near || far < root {
                    root = (-half_b + sqrtd) / a;
                    if root < near || far < root {
                        None
                    } else {
                        Some(root)
                    }
                } else {
                    Some(root)
                }
            };
            if let Some(root) = root {
                let length = root;
                let pos = self.at(length);
                let normal = (pos - center) / radius;
                Some(Hit::new(self, length, pos, normal, material))
            } else {
                None
            }
        }
    }
    fn cast_inner(&self, world: &World, depth: usize, scale: f32, gathered: Color) -> Color {
        let hit = world.hit(self);
        if let Some(hit) = hit {
            // let Hit{pos, normal, material, ..} = hit;
            if depth > 0 {
                let material = hit.material.clone();
                let bounce = material.scatter(hit);
                let Bounce {ray, attenuation} = bounce;
                ray.cast_inner(world, depth - 1, scale * 0.5, attenuation)
            } else {
                // world.background_color(self)
                Color::RED
            }
        } else {
            gathered * (world.background_color(self) * scale)
        }
    }
    pub fn cast(&self, world: &World, depth: usize) -> Color {
        self.cast_inner(world, depth, 1.0, Color::WHITE )
    }
    /// move the ray around a bit
    /// todo: this is a mess
    pub fn perturb(&self, scale_x: f32, scale_y: f32) -> Ray {
        let mut rng = rand::thread_rng();
        let dx: f32 = rng.gen();
        let dy: f32 = rng.gen();
        Ray {
            origin: self.origin + Vector::new(dx * scale_x, dy * scale_y, 0.0),
            direction: self.direction,
        }
        
    }
}