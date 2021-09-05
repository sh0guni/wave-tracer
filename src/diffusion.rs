use crate::Vec3;
use rand::distributions::{Distribution, Uniform};
use rand::{thread_rng, Rng};

// Simple diffuse
pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = thread_rng();
    let between = Uniform::from(-1.0..1.0);
    loop {
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

// True Lambertian
pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit_vector()
}

// Uniform scatter direction away from the hit point
#[allow(dead_code)]
pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        // In the same hemisphere as the normal
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = thread_rng();
    loop {
        let (x, y) = rng.gen();
        let p = Vec3::new(x, y, 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
