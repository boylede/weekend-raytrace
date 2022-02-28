use std::cmp::Ordering;

use crate::{numbers::{Color, Vector}, ray::{Hit, Ray}};

pub struct World {
    spheres: Vec<(Vector, f32)>,
}

impl World {
    pub fn hit(&self, ray: &Ray) -> Option<Hit> {
        self.spheres
            .iter()
            .filter_map(|(pos, radius)| ray.hit_sphere(*pos, *radius, f32::EPSILON, f32::INFINITY))
            .filter(|h| h.front)
            .min_by(|a, b| {
                (&a.length)
                    .partial_cmp(&b.length)
                    .unwrap_or(Ordering::Equal)
            })
    }
    pub fn new() -> World {
        let spheres = vec![
            (Vector::new(0.0, 0.0, -1.0), 0.5),
            (Vector::new(1.0, 0.0, -1.0), 0.4),
            (Vector::new(-1.0, 0.0, -1.0), 0.4),
            (Vector::new(2.0, -1000.5, -1.0), 1000.0),
        ];

        World { spheres }
    }
    pub fn background_color(&self, ray: &Ray) -> Color {
        let unit_direction = ray.direction.unit();
        let horizon = 0.5 * (unit_direction.y + 1.0);
        Color::blend(Color::WHITE, Color::GRADE, horizon)
    }
}
