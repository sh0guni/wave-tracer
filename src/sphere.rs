use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Point3;
use std::rc::Rc;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        if let Some(t) = find_root_in_range(a, half_b, t_min, t_max, sqrtd) {
            let p = r.at(t);
            let outward_normal = (p - self.center) / self.radius;
            let rec = HitRecord::new(p, t, r, &outward_normal, Rc::clone(&self.material));
            return Some(rec);
        } else {
            return None;
        }
    }
}

// Find the nearest root that lies in the acceptable range.
fn find_root_in_range(a: f64, half_b: f64, t_min: f64, t_max: f64, sqrtd: f64) -> Option<f64> {
    let root = (-half_b - sqrtd) / a;
    if root < t_min || t_max < root {
        let root = (-half_b + sqrtd) / a;
        if root < t_min || t_max < root {
            return None;
        }
        return Some(root);
    }
    Some(root)
}
