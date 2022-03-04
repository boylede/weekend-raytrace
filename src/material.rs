use crate::{numbers::{Color, Vector}, ray::{Ray, Hit, Bounce}};


pub struct Material {
    // pub metalness: f32,
    pub roughness: f32,
    pub albedo: Color,
}

impl Material {
    pub const TEST_GLOSSY: Material = Material {roughness: 0.0, albedo: Color::RED};
    pub const TEST_ROUGH: Material = Material {roughness: 1.0, albedo: Color::BLUE};
    pub fn new() -> Material {
        Material { roughness: 1.0, albedo: Color::GRAY }
    }
}

pub trait Shader {
    fn scatter(&self, ray: Hit) -> Bounce;
}

impl Shader for Material {
    fn scatter(&self, hit: Hit) -> Bounce {
        let Hit{pos, normal, ..} = hit;
        let diffuse_target = pos + normal + Vector::random().unit();
        let ray = Ray::new(pos, diffuse_target - pos);
        let attenuation = self.albedo.clone();
        Bounce {ray, attenuation}
    }
}