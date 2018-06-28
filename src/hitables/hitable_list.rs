use hitables::hitable::Hitable;
use hitables::hitable::HitRecord;
use ray::Ray;
use std::fmt::Debug;
use materials::material::Material;

pub struct HitableList {
    pub hitables: Vec<Box<Hitable+Sync>>,
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_hit_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for hitable in self.hitables.iter() {
            match hitable.hit(ray, t_min, closest_so_far) {
                None => { continue; }
                Some(hit) => {
                    closest_so_far = hit.t;
                    closest_hit_record = Some(hit);
                }
            }
        }

        return closest_hit_record;
    }
}