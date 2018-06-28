use hitables::hitable::HitRecord;
use rand;
use ray::Ray;
use std::fmt::Debug;
use utils::random_point_in_unit_sphere;
use vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3, f64),
    Dielectric(f64),
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        match self {
            Material::Lambertian(albedo) => {
                let target = &(&hit_record.p + &hit_record.normal) + &random_point_in_unit_sphere();
                return Some((Ray {
                    origin: hit_record.p,
                    direction: &target - &hit_record.p,
                }, albedo.clone()));
            }
            Material::Metal(albedo, material_fuzz) => {
                let one = 1.0;
                let fuzz = if material_fuzz < &one { &material_fuzz } else { &one };
                let reflected = total_reflection(&(*ray_in).direction.unit_vector(), &hit_record.normal);
                if reflected.dot(&hit_record.normal) > 0.0 {
                    Some((Ray {
                        origin: hit_record.p,
                        direction: &reflected + &(&random_point_in_unit_sphere() * &(*fuzz)),
                    }, albedo.clone()))
                } else {
                    None
                }
            }
            Material::Dielectric(refraction_index) => {
                Material::dielectric_scatter(&ray_in, &hit_record, refraction_index.clone())
            }
        }
    }

    fn dielectric_scatter(ray_in: &&Ray, hit_record: &&HitRecord, refraction_index: f64) -> Option<(Ray, Vec3)> {
        let first_ref_index_over_second_ref_index;
        let outward_normal;
        let cosine;

        if ray_in.direction.dot(&hit_record.normal) > 0.0 {
            first_ref_index_over_second_ref_index = refraction_index;
            outward_normal = &hit_record.normal * &-1.0;
            cosine = refraction_index * ray_in.direction.dot(&hit_record.normal) / ray_in.direction.length();
        } else {
            first_ref_index_over_second_ref_index = &1.0 / refraction_index;
            outward_normal = hit_record.normal.clone();
            cosine = -ray_in.direction.dot(&hit_record.normal) / ray_in.direction.length();
        };

        match dielectric_refract(&ray_in.direction, &outward_normal, first_ref_index_over_second_ref_index) {
            Some(refracted_ray) => {
                let reflection_probability = schlick(cosine, refraction_index.clone());
                if rand::random::<f64>() < reflection_probability {
                    Some((Ray {
                        origin: hit_record.p,
                        direction: total_reflection(&ray_in.direction, &hit_record.normal),
                    }, Vec3 { x: 1.0, y: 1.0, z: 1.0 }))
                } else {
                    Some((Ray {
                        origin: hit_record.p,
                        direction: refracted_ray,
                    }, Vec3 { x: 1.0, y: 1.0, z: 1.0 }))
                }
            }
            None => {
                Some((Ray {
                    origin: hit_record.p,
                    direction: total_reflection(&ray_in.direction, &hit_record.normal),
                }, Vec3 { x: 1.0, y: 1.0, z: 1.0 }))
            }
        }
    }
}


fn total_reflection(direction: &Vec3, hit_point_normal: &Vec3) -> Vec3 {
    direction - &(hit_point_normal * &(&(direction.dot(hit_point_normal)) * &2.0))
}

fn dielectric_refract(direction: &Vec3, hit_point_normal: &Vec3,
                      first_ref_index_over_second_ref_index: f64) -> Option<Vec3> {
    let unit_direction = direction.unit_vector();

    let dt = unit_direction.dot(hit_point_normal);
    let discriminant = 1.0 - first_ref_index_over_second_ref_index * first_ref_index_over_second_ref_index * (1.0 - dt * dt);
    if discriminant > 0.0 {
        let refracted = &(&(&unit_direction - &(hit_point_normal * &dt)) * &first_ref_index_over_second_ref_index) -
            &(hit_point_normal * &discriminant.sqrt());
        Some(refracted)
    } else {
        None
    }
}

fn schlick(cosine: f64, refraction_index: f64) -> f64 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * ((1.0 - cosine).powi(5)) as f64
}