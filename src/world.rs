use crate::numbers::{Vector, Ray, Hit};


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
        let spheres = vec![(Vector::new(0.0, 0.0, -1.0), 0.5), (Vector::new(0.0, -401.0, 0.0), 400.0)];
        World { spheres }
    }
}