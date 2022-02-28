use std::{ops::{Add, Div, Mul, Sub}, iter::Sum};
use rand::Rng;
use crate::image::Pixel;

/// a group of RGB color samples
pub struct Samples {
    r: f32,
    g: f32,
    b: f32,
    count: u32,
}

impl Samples {
    const NONE: Samples = Samples {r: 0.0, g: 0.0, b: 0.0, count: 0};
    pub fn from_color(color: Color) -> Samples {
        Samples {
            r: color.r,
            g: color.g,
            b: color.b,
            count: 1,
        }
    }
    pub fn to_color(&self) -> Color {
        let n = self.count as f32;
        Color {
            r: self.r / n,
            g: self.g / n,
            b: self.b / n,
        }
    }
}

impl Add for Samples {
    type Output = Samples;
    fn add(self, rhs: Samples) -> Samples {
        Samples {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            count: self.count + rhs.count,
        }
    }

    
}

impl Sum for Samples {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a,s|a+s).unwrap_or(Samples::NONE)
    }
}

/// RGB color, in linear space with channels from 0-1
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Color {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Color {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Color {
    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };
    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };
    pub const GRADE: Color = Color {
        r: 0.5,
        g: 0.7,
        b: 1.0,
    };
    pub const RED: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
    };
    pub fn blend(a: Color, b: Color, t: f32) -> Color {
        // let g = 1.0 - t;
        let a = a * (1.0 - t);
        let b = b * t;
        a + b
        // Color { r: a.r*g+b.r*t, g: a.g*g+b.g*t, b: a.b*g+b.b*t }
    }
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }
    pub fn truncate(&self) -> Pixel {
        let r = (self.r * 255.0) as u8;
        let g = (self.g * 255.0) as u8;
        let b = (self.b * 255.0) as u8;

        Pixel::new(r, g, b)
    }
    pub fn sample(&self) -> Samples {
        Samples {
            r: self.r,
            g: self.g,
            b: self.b,
            count: 1,
        }
    }
}

/// a three dimensional value
#[derive(Copy, Clone, Debug)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector {
    pub const Z_NEG: Vector = Vector {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    pub const Z_POS: Vector = Vector {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };
    pub const ORIGIN: Vector = Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z }
    }
    /// produces a new random vector with length 1
    /// uses the technique described here: https://mathworld.wolfram.com/SpherePointPicking.html
    pub fn random() -> Vector {
        let mut rng = rand::thread_rng();
        let u: f32 = rng.gen();
        let v: f32 = rng.gen();
        let theta = u * 2.0 * PI;
        let phi = (2.0 * v - 1.0).acos();
        let r = rng.gen::<f32>().cbrt();
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        let sin_phi = phi.sin();
        let cos_phi = phi.cos();
        let x = r * sin_phi * cos_theta;
        let y = r * sin_phi * sin_theta;
        let z = r * cos_phi;
        Vector {x, y, z}
    }
    pub fn non_zero(&self) -> bool {
        self.x != 0.0 && self.y != 0.0 && self.z != 0.0
    }
    pub fn unit(&self) -> Vector {
        *self / self.length()
    }
    pub fn length(&self) -> f32 {
        self.square_length().sqrt()
    }
    pub fn square_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn dot(&self, rhs: &Vector) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn as_color(&self) -> Color {
        Color::new(self.x + 1.0, self.y + 1.0, self.z + 1.0) * 0.5
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Mul for Vector {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<Vector> for f32 {
    type Output = Vector;

    fn div(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Ray {
    origin: Vector,
    direction: Vector,
}

#[derive(Copy, Clone)]
pub struct Hit {
    length: f32,
    pos: Vector,
    normal: Vector,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        Ray { origin, direction }
    }
    pub fn at(&self, magnitude: f32) -> Vector {
        self.origin + (self.direction * magnitude)
    }
    pub fn background_color(&self) -> Color {
        let unit_direction = self.direction.unit();
        let horizon = 0.5 * (unit_direction.y + 1.0);
        Color::blend(Color::WHITE, Color::GRADE, horizon)
    }
    pub fn unit(&self) -> Ray {
        let length = self.direction.length();
        Ray {
            origin: self.origin,
            direction: self.direction / length,
        }
    }
    pub fn hit_sphere(&self, center: Vector, radius: f32) -> Option<Hit> {
        let oc: Vector = self.origin - center;
        let a = self.direction.dot(&self.direction);
        let half_b = oc.dot(&self.direction);
        let c = oc.square_length() - radius * radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            None
        } else {
            let sqrtd = discriminant.sqrt();
            let root = (-half_b - sqrtd) / a;
            // todo: skipping some root-related code here that returned early
            let length = root;
            let pos = self.at(length);
            let normal = (pos - center) / radius;
            // if !normal.is_unit() {
            //     eprintln!("debug: had a non-normalized normal vector: {}", normal.length());
            // }
            Some(Hit{length, pos, normal})
            // Some((-b - discriminant.sqrt()) / (2.0 * a))
        }
    }
    pub fn cast(&self, world: &World, depth: usize) -> Color {
        let hit = world.hit(self);
        if let Some(Hit{length, pos, normal}) = hit {
            // let normal = (self.at(distance) - Vector::Z_NEG).unit();
            // todo: i think this normal is wrong so this code is probably wrong
            let diffuse_target = pos + normal + Vector::random();
            // normal.as_color()
            // 0.5 * ray_color(ray(rec.p, target - rec.p), world);
            let next_ray = Ray::new(pos, diffuse_target - pos);
            if depth < 1 {
                0.5 * next_ray.cast(world, depth + 1)
            } else {
                next_ray.background_color()
            }
            
        } else {
            self.background_color()
        }
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

#[test]
fn test_vec_mul() {
    let a = Vector::new(1.0, 1.0, 1.0);
    let b = Vector::new(2.0, 1.0, 1.0);
    let c = a * b;
    assert!(c.x == 2.0);
    assert!(c.y == 1.0);
    assert!(c.z == 1.0);
}

#[test]
fn test_pos_vec_sub() {
    let a = Position::new(1.0, 1.0, 1.0);
    let b = Vector::new(2.0, 0.5, 1.0);
    let c = a - b;
    assert!(c.x == -1.0);
    assert!(c.y == 0.5);
    assert!(c.z == 0.0);
}

#[test]
fn test_vec_vec_sub() {
    let a = Vector::new(1.0, 1.0, 1.0);
    let b = Vector::new(2.0, 0.5, 1.0);
    let c = a - b;
    assert!(c.x == -1.0);
    assert!(c.y == 0.5);
    assert!(c.z == 0.0);
}
