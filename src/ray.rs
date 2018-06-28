use vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        &self.origin + &(&self.direction * &t)
    }
}

mod tests {
    use vec3::Vec3;
    use ray::Ray;

    #[test]
    fn find_point_after_a_second() {
        let ray = Ray {
            origin: Vec3 { x: 1.0, y: 2.0, z: 3.0 },
            direction: Vec3 { x: 2.0, y: 2.0, z: 3.0 },
        };
        assert_eq!(ray.point_at_parameter(1.0), Vec3 { x: 3.0, y: 4.0, z: 6.0 });
    }
}