use hitables::hitable::Hitable;
use hitables::hitable::HitRecord;
use materials::material::Material;
use ray::Ray;
use std::fmt::Debug;
use vec3::Vec3;

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Hitable for Sphere {
    #[inline(never)]
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = &ray.origin - &self.center;

        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let squared_discriminant = discriminant.sqrt();


        let time_hit = (-b - squared_discriminant) / (a);

        if t_min < time_hit && time_hit < t_max {
            return Some(self.generate_hit_record_at_point(ray, time_hit));
        }

        let time_hit = (-b + squared_discriminant) / (a);

        if t_min < time_hit && time_hit < t_max {
            return Some(self.generate_hit_record_at_point(ray, time_hit));
        }

        None
    }
}

impl Sphere {
    fn generate_hit_record_at_point(&self, ray: &Ray, time_hit: f64) -> HitRecord {
        let point_of_contact = ray.point_at_parameter(time_hit);
        let normal = &(&point_of_contact - &self.center) / &self.radius;
        return HitRecord {
            t: time_hit,
            p: point_of_contact,
            normal,
            material: self.material.clone(),
        };
    }
}
