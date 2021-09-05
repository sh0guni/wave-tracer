use crate::vec3::Vec3;
use std::ops::{Add, AddAssign, Mul};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

fn translate_color_value(n: f64) -> u32 {
    (256.0 * n.clamp(0.0, 0.999)) as u32
}

pub fn get_pixel(color: Color, samples_per_pixel: usize) -> String {
    // Divbide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;

    let Color { r, g, b } = color * scale;

    let ir = translate_color_value(r.sqrt());
    let ig = translate_color_value(g.sqrt());
    let ib = translate_color_value(b.sqrt());
    format!("{} {} {}\n", ir, ig, ib)
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Add<Color> for Vec3 {
    type Output = Color;

    fn add(self, other: Color) -> Self::Output {
        Color {
            r: self.x + other.r,
            g: self.y + other.g,
            b: self.z + other.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        };
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        rhs * self
    }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}
