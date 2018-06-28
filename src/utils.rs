use rand;
use vec3::Vec3;

fn gen_in_distance_1() -> Vec3 {
    &(&Vec3 { x: rand::random(), y: rand::random(), z: rand::random() } * &2.0) - &Vec3 { x: 1.0, y: 1.0, z: 1.0 }
}

fn gen_in_plane_1() -> Vec3 {
    &(&Vec3 { x: rand::random(), y: rand::random(), z: 0.0 } * &2.0) - &Vec3 { x: 1.0, y: 1.0, z: 0.0 }
}


pub fn random_point_in_unit_sphere() -> Vec3 {
    let mut point = gen_in_distance_1();

    while point.squared_length() >= 1.0 {
        point = gen_in_distance_1();
    }

    point
}


pub fn random_point_in_unit_disk() -> Vec3 {
    let mut point = gen_in_plane_1();

    while point.dot(&point) >= 1.0 {
        point = gen_in_plane_1();
    }

    point
}