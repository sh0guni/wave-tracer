use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + -other
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = thread_rng();
    loop {
        let between = Uniform::from(-1.0..1.0);
        let p = Vec3::new(
            between.sample(&mut rng),
            between.sample(&mut rng),
            between.sample(&mut rng),
        );

        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub type Point3 = Vec3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmp_vec3() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn add_vec3() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) + Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(2.0, 4.0, 6.0)
        );
    }

    #[test]
    fn sub_vec3() {
        assert_eq!(
            Vec3::new(2.0, 4.0, 6.0) - Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(1.0, 2.0, 3.0)
        );
    }

    #[test]
    fn mul_vec3() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) * 2.0, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) * Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(1.0, 4.0, 9.0)
        );
    }

    #[test]
    fn div_vec3() {
        assert_eq!(Vec3::new(2.0, 4.0, 6.0) / 2.0, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn length() {
        let vx = Vec3::new(2.0, 0.0, 0.0);
        assert_eq!(vx.length(), 2.0);

        let vy = Vec3::new(0.0, 2.0, 0.0);
        assert_eq!(vy.length(), 2.0);

        let vz = Vec3::new(0.0, 0.0, 2.0);
        assert_eq!(vz.length(), 2.0);

        let v0 = Vec3::new(0.0, 0.0, 0.0);
        assert_eq!(v0.length(), 0.0);

        let v = Vec3::new(2.0, 3.0, 4.0);
        assert_eq!(v.length(), 29.0f64.sqrt());
    }

    #[test]
    fn dot_vec3() {
        assert_eq!(
            Vec3::new(1.0, 3.0, -5.0).dot(&Vec3::new(4.0, -2.0, -1.0)),
            3.0
        );

        // Two vectors at right angles to each other have a dot product of zero
        assert_eq!(
            Vec3::new(-12.0, 16.0, 0.0).dot(&Vec3::new(12.0, 9.0, 0.0)),
            0.0
        );
    }

    #[test]
    fn cross_vec3() {
        assert_eq!(
            Vec3::new(2.0, 3.0, 4.0).cross(Vec3::new(5.0, 6.0, 7.0)),
            Vec3::new(-3.0, 6.0, -3.0),
        );
    }
}
