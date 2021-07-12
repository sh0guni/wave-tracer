use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
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

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

type Point3 = Vec3;
type Color = Vec3;

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
}
