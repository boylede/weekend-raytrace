use crate::{numbers::{Color, Vector}, ray::{Ray, Hit, Bounce}};
use rand::Rng;

pub enum Material {
    Metal(f32, Color), // roughness and albedo
    Diffuse(f32, Color),// roughness and albedo
    Dielectric(f32, Color), // ior and attenuation?
}

impl Material {
    pub const TEST_GLOSSY: Material = Material::Metal(0.25, Color::GREEN);
    pub const TEST_METAL_RED: Material = Material::Metal(0.25, Color::REDDISH); 
    pub const TEST_METAL_BLUE: Material = Material::Metal(0.0, Color::BLUE);
    pub const TEST_ROUGH: Material = Material::Diffuse(0.5, Color::BLUE);
    pub const TEST_DIE: Material = Material::Dielectric(1.5, Color::BLUE);
    pub const fn new() -> Material {
        Material::Diffuse(1.0, Color::GRAY)
    }
}

pub trait Shader {
    fn scatter(&self, ray: Hit) -> Option<Bounce>;
}

impl Shader for Material {
    fn scatter(&self, hit: Hit) -> Option<Bounce> {
        let Hit{by, pos, normal, front, ..} = hit;
        use Material::*;
        match self {
            Metal(roughness, color) => {
                let reflected = by.direction.unit().reflect(&normal);
                let rough_reflected = if *roughness != 0.0 {
                    let random = Vector::random() * *roughness;
                    random + reflected
                } else {
                    reflected
                };
                let ray = Ray::new(pos,rough_reflected);
                let attenuation = color.clone();
                if ray.direction.dot(&normal) > 0.0 {
                    Some(Bounce {ray, attenuation})
                } else {
                    None
                }
            },
            Diffuse(roughness, color) => {
                let mut scatter_direction = normal + Vector::random().unit() * *roughness;
                if scatter_direction.near_zero() {
                    scatter_direction = normal;
                }
                let ray = Ray::new(pos, scatter_direction - pos);
                let attenuation = color.clone();
                Some(Bounce {ray, attenuation})
            },
            Dielectric(ior, color) => {
                let attenuation = color.clone();
                let refraction_ratio = if front {
                    1.0/ior
                } else {
                    *ior
                };
                let direction = by.direction.unit();
                let refracted = direction.refract(&normal, refraction_ratio);
                let ray = Ray::new(pos, refracted);

                Some(Bounce{ ray, attenuation})
                
            },
        }
        

    }
}