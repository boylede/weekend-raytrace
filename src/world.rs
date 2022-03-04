use std::{cmp::Ordering, rc::Rc};

use crate::{
    material::Material,
    numbers::{Color, Vector},
    ray::{Hit, Ray},
};

pub struct World {
    spheres: Vec<Sphere>,
}

impl World {
    pub fn hit(&self, ray: &Ray) -> Option<Hit> {
        self.spheres
            .iter()
            .filter_map(|Sphere { pos, radius, material  }| {
                ray.hit_sphere(*pos, *radius, f32::EPSILON, f32::INFINITY, material)
            })
            .filter(|h| h.front)
            .min_by(|a, b| {
                (&a.length)
                    .partial_cmp(&b.length)
                    .unwrap_or(Ordering::Equal)
            })
    }
    pub fn new() -> World {
        let spheres = vec![
            Sphere::new(0.0, 0.0, -1.0, 0.5),
            Sphere::new(1.0, 0.0, -1.0, 0.4).with_material(Material::TEST_GLOSSY),
            Sphere::new(-1.0, 0.0, -1.0, 0.4).with_material(Material::TEST_ROUGH),
            Sphere::new(2.0, -1000.5, -1.0, 1000.0),
        ];

        World { spheres }
    }
    pub fn background_color(&self, ray: &Ray) -> Color {
        let unit_direction = ray.direction.unit();
        let horizon = 0.5 * (unit_direction.y + 1.0);
        Color::blend(Color::WHITE, Color::GRADE, horizon)
    }
}

pub struct Sphere {
    pos: Vector,
    radius: f32,
    material: Rc<Material>,
}

impl Sphere {
    pub fn new(x: f32, y: f32, z: f32, radius: f32,) -> Sphere {
        let pos = Vector::new(x,y,z);
        let material = Rc::new(Material::new());
        Sphere { pos, radius, material }
    }
    pub fn with_material(mut self, material: Material) -> Self {
        self.material = Rc::new(material);
        self
    }
}
