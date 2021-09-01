use crate::hittable::HitRecord;
use crate::random_unit_vector;
use crate::Color;
use crate::Ray;
use std::fmt::Debug;

pub struct Scatter {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter>;
}

#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let scatter_direction = rec.normal + random_unit_vector();

        // Catch degenerate scatter direction
        let corrected_scatter_direction = if scatter_direction.near_zero() {
            rec.normal
        } else {
            scatter_direction
        };

        Some(Scatter {
            scattered: Ray::new(rec.p, corrected_scatter_direction),
            attenuation: self.albedo,
        })
    }
}
