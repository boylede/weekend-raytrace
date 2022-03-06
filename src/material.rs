use crate::{numbers::{Color, Vector}, ray::{Ray, Hit, Bounce}};
use rand::Rng;

pub struct Material {
    pub metalness: f32,
    pub roughness: f32,
    pub albedo: Color,
}

impl Material {
    pub const TEST_GLOSSY: Material = Material {metalness: 1.0, roughness: 0.0, albedo: Color::GREEN};
    pub const TEST_METAL_RED: Material = Material {metalness: 1.0, roughness: 0.00001, albedo: Color::RED};
    pub const TEST_METAL_BLUE: Material = Material {metalness: 1.0, roughness: 0.05, albedo: Color::BLUE};
    pub const TEST_ROUGH: Material = Material {metalness: 0.0, roughness: 1.0, albedo: Color::BLUE};
    pub const fn new() -> Material {
        Material { metalness: 0.0, roughness: 1.0, albedo: Color::GRAY }
    }
}

pub trait Shader {
    fn scatter(&self, ray: Hit) -> Option<Bounce>;
}

impl Shader for Material {
    fn scatter(&self, hit: Hit) -> Option<Bounce> {
        let Hit{by, pos, normal, ..} = hit;
        if self.metalness == 0.0 {
            let mut scatter_direction = normal + Vector::random().unit();
            if scatter_direction.near_zero() {
                scatter_direction = normal;
            }
            let ray = Ray::new(pos, scatter_direction - pos);
            let attenuation = self.albedo.clone();
            Some(Bounce {ray, attenuation})
        } else {
            let reflected = by.direction.unit().reflect(&normal);
            let rough_reflected = if self.roughness != 0.0 {
                
                let random = Vector::random() * self.roughness;
                random * reflected
            } else {
                reflected
            };
            let ray = Ray::new(pos,rough_reflected);
            let attenuation = self.albedo.clone();
            if ray.direction.dot(&normal) > 0.0 {
                Some(Bounce {ray, attenuation})
            } else {
                None
            }
            
        }

    }
}