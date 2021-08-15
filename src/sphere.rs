use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::Point3;

struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &HitRecord) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        if let Some(root) = find_root_in_range(a, half_b, t_min, t_max, sqrtd) {
            let rec = HitRecord {
                p: r.at(rec.t),
                normal: (rec.p - self.center) / self.radius,
                t: root,
            };
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
