use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub t: f64,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Point3,
        t: f64,
        r: &Ray,
        outward_normal: &Vec3,
        material: &'a dyn Material,
    ) -> Self {
        let front_face = r.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        };
        Self {
            p,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
