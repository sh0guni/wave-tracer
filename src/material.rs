use crate::hittable::HitRecord;
use crate::Color;
use crate::Ray;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &Color, scattered: &Ray) -> bool;
}
