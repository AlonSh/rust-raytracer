use vec3::Vec3;
use ray::Ray;
use std::f64::consts::PI;
use utils::random_point_in_unit_disk;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lens_radius: f64,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl Camera {
    /// vertical_field_of_view is in degrees from top to bottom
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vertical_field_of_view: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Camera {
        let theta = vertical_field_of_view * PI / 180.0; // convert to radians
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        let w = (&look_from - &look_at).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        Camera {
            lower_left_corner: &(&(&look_from - &(&u * &(half_width * focus_dist))) - &(&v * &(half_height * focus_dist))) - &(&w * &focus_dist),
            horizontal: &u * &(half_width * 2.0 * focus_dist),
            vertical: &v * &(half_height * 2.0 * focus_dist),
            origin: look_from,
            lens_radius: aperture / 2.0,
            u,
            v,
            w,
        }
    }

    pub fn get_ray_for_pixel(&self, s: f64, t: f64) -> Ray {
        let rd = &random_point_in_unit_disk() * &self.lens_radius;
        let offset = &(&self.u * &rd.x) + &(&self.v * &rd.y);
        Ray {
            origin: &self.origin + &offset,
            direction: &(&(&self.lower_left_corner + &(&(&self.horizontal * &s) + &(&self.vertical * &t))) - &self.origin) - &offset,
        }
    }
}