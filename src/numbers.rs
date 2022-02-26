use std::ops::{Add, Div, Mul, Sub};

use crate::image::Pixel;
///
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Color {
        Color { r: self.r + rhs.r, g: self.g + rhs.g, b: self.b + rhs.b }
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Color {
        Color { r: self.r * rhs, g: self.g * rhs, b: self.b * rhs }
    }
}

impl Color {
    pub const WHITE: Color = Color {r: 1.0, g: 1.0, b: 1.0};
    pub const BLACK: Color = Color {r: 0.0, g: 0.0, b: 0.0};
    pub const GRADE: Color = Color {r: 0.5, g: 0.7, b: 1.0};
    pub const RED: Color = Color {r: 1.0, g: 0.0, b: 0.0};
    pub fn blend(a: Color, b: Color, t: f32) -> Color {
        // let g = 1.0 - t;
        let a = a * (1.0-t);
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

        Pixel::new(r,g,b)
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
    pub fn non_zero(&self) -> bool {
        self.x != 0.0 && self.y != 0.0 && self.z != 0.0
    }
    pub fn unit(&self) -> Vector {
        *self / self.length()
    }
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn dot(&self, rhs: &Vector) -> f32 {
        self.x * rhs.x
         + self.y * rhs.y
         + self.z * rhs.z
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

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        Ray { origin, direction }
    }
    pub fn at(&self, magnitude: f32) -> Vector {
        self.origin + self.direction * magnitude
    }
    pub fn background_color(&self) -> Color {
        let unit_direction = self.direction.unit();
        let horizon = 0.5 * (unit_direction.y + 1.0);
        Color::blend(Color::WHITE, Color::GRADE, horizon)
    }
    pub fn unit(&self) -> Ray {
        let length = self.direction.length();
        Ray { origin: self.origin, direction: self.direction / length }
    }
    pub fn hit_sphere(&self, center: Vector, radius: f32) -> bool {
        let oc: Vector = self.origin - center;
        let a = self.direction.dot(&self.direction);
        let b = 2.0 * oc.dot(&self.direction);
        let c = oc.dot(&oc) - radius*radius;
        let discriminant = b*b - 4.0 * a* c;
        discriminant > 0.0
    }
    pub fn cast(&self, _world: ()) -> Color {
        if self.hit_sphere(Vector::new(0.0, 0.0, 0.0), 0.5) {
            Color::RED
        } else {
            self.background_color()
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
