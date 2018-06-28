use vec3::Vec3;
use ray::Ray;
use materials::material::Material;
use std::fmt::Debug;

#[derive(Debug)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}