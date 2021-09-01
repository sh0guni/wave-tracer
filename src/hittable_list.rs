use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let (hit_record, _) = self.objects.iter().fold((None, t_max), |acc, x| {
            let (_, closest_so_far) = acc;
            let hit_record = x.hit(r, t_min, closest_so_far);
            match hit_record {
                Some(ref hit) => {
                    let t = hit.t;
                    (hit_record, t)
                }
                None => acc,
            }
        });
        return hit_record;
    }
}
