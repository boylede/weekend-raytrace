use crate::numbers::{Vector, Ray, Hit, Color};


pub struct World {
    spheres: Vec<(Vector, f32)>,
}

impl World {
    pub fn hit(&self, ray: &Ray) -> Option<Hit> {
        self.spheres.iter().map(|(pos, radius)| {
            ray.hit_sphere(*pos, *radius)
        }).filter(|o|o.is_some()).next().flatten()
    }
    pub fn new() -> World {
        let mut spheres = vec![(Vector::new(0.0, 0.0, -1.0), 0.5), (Vector::new(2.0, -100.5, -1.0), 100.0)];
        World { spheres }
    }
        let unit_direction = ray.direction.unit();
        let horizon = 0.5 * (unit_direction.y + 1.0);
        Color::blend(Color::WHITE, Color::GRADE, horizon)
    }
}

