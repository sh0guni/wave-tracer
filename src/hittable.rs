use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(r: &Ray, t_min: f64, t_max: f64, rec: &HitRecord) -> bool;
}
