#![feature(alloc_system)]
extern crate alloc_system;

extern crate rand;
extern crate ray_tracer;
extern crate rayon;
extern crate indicatif;

use ray_tracer::camera::Camera;
use ray_tracer::hitables::hitable::Hitable;
use ray_tracer::hitables::hitable_list::HitableList;
use ray_tracer::hitables::sphere::Sphere;
use ray_tracer::materials::material::Material;
use ray_tracer::ray::Ray;
use ray_tracer::vec3::Vec3;
use rayon::prelude::*;
use indicatif::ProgressStyle;
use indicatif::ProgressBar;

const NX: usize = 400;
const NY: usize = 200;

fn main() {
    let anti_aliasing_ray_count = 100;

    print!("P3\n{} {}\n255\n", NX, NY);

    let (camera, world) = benchmark_world_setup();
    let mut output = vec![(0, 0, 0); NX * NY];

    let pbar = ProgressBar::new((NY * NX) as u64);

    pbar.set_style(ProgressStyle::default_bar().template(
        "[{elapsed} elapsed] {wide_bar:.cyan/white} {percent}% [{eta} remaining] [rendering]",
    ));


    let anti_aliasing_random: Vec<(f64, f64)> = (0..anti_aliasing_ray_count).map(|_| { (rand::random::<f64>(), rand::random::<f64>()) }).collect();


    output.par_iter_mut().enumerate().for_each(|(index, x)| {
//    output.iter_mut().enumerate().for_each(|(index, x)| {
        let i = index / NX;
        let j = index % NX;
        let mut col = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

        for aa_index in 0..anti_aliasing_ray_count {
            let u_random_part: f64 = anti_aliasing_random[aa_index].0;
            let v_random_part: f64 = anti_aliasing_random[aa_index].1;

            let u = (j as f64 + u_random_part) / NX as f64;
            let v = (i as f64 + v_random_part) / NY as f64;

            let r = camera.get_ray_for_pixel(u, v);

            col = &col + &color(&r, &world, 0);
        }

        col = &col / &(anti_aliasing_ray_count as f64);
        col = Vec3 {
            x: col.x.sqrt(),
            y: col.y.sqrt(),
            z: col.z.sqrt(),
        };

        let ir = (255.99 * col.x) as i32;
        let ig = (255.99 * col.y) as i32;
        let ib = (255.99 * col.z) as i32;
        pbar.inc(1);
        *x = (ir, ig, ib);
    });

//    for j in 0..NY {
//        for i in (0..NX).rev() {
//            let mut col = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
//
//            for _ in 0..anti_aliasing_ray_count {
//                let u_random_part: f64 = rand::random::<f64>();
//                let v_random_part: f64 = rand::random::<f64>();
//
//                let u = (i as f64 + u_random_part) / NX as f64;
//                let v = (j as f64 + v_random_part) / NY as f64;
//
//                let r = camera.get_ray_for_pixel(u, v);
//
//                col = &col + &color(&r, &world, 0);
//            }
//
//            col = &col / &(anti_aliasing_ray_count as f64);
//            col = Vec3 {
//                x: col.x.sqrt(),
//                y: col.y.sqrt(),
//                z: col.z.sqrt(),
//            };
//
//            let ir = (255.99 * col.x) as i32;
//            let ig = (255.99 * col.y) as i32;
//            let ib = (255.99 * col.z) as i32;
//            output[i + NX * j] = (ir, ig, ib);
////            println!("{} {} {}", ir, ig, ib);
//        }
//    }

    for index in (0..(NY * NX)).rev() {
        let cell = &output[index];
        println!("{} {} {}", cell.0, cell.1, cell.2);
    }
}

//fn world_setup_1() -> (Camera, HitableList) {
//    let look_from = Vec3 { x: 3.0, y: 3.0, z: 2.0 };
//    let look_at = Vec3 { x: 0.0, y: 0.0, z: -1.0 };
//    let dist_to_focus = (&look_from - &look_at).length();
//    let aperture = 2.0;
//    let camera = Camera::new(look_from,
//                             look_at,
//                             Vec3 { x: 0.0, y: 1.0, z: 0.0 },
//                             20.0, 2.0,
//                             aperture, dist_to_focus);
//    let sphere1 = Sphere {
//        center: Vec3 { x: 0.0, y: 0.0, z: -1.0 },
//        radius: 0.5,
//        material: Material::Lambertian(Vec3 { x: 0.1, y: 0.2, z: 0.5 }),
//    };
//    let sphere2 = Sphere {
//        center: Vec3 { x: 0.0, y: -100.5, z: -1.0 },
//        radius: 100.0,
//        material: Material::Lambertian(Vec3 { x: 0.8, y: 0.8, z: 0.0 }),
//    };
//    let sphere3 = Sphere {
//        center: Vec3 { x: 1.0, y: 0.0, z: -1.0 },
//        radius: 0.5,
//        material: Material::Metal(Vec3 { x: 0.8, y: 0.6, z: 0.2 }, 0.3),
//    };
//    let sphere4 = Sphere {
//        center: Vec3 { x: -1.0, y: 0.0, z: -1.0 },
//        radius: 0.5,
//        material: Material::Dielectric(1.5),
//    };
//    let sphere5 = Sphere {
//        center: Vec3 { x: -1.0, y: 0.0, z: -1.0 },
//        radius: -0.45,
//        material: Material::Dielectric(1.5),
//    };
//    let world = HitableList {
//        hitables: vec![
//            sphere1),
//            sphere2),
//            sphere3),
//            sphere4),
//            sphere5)]
//    };
//    (camera, world)
//}

fn benchmark_world_setup() -> (Camera, HitableList) {
    let mut world = HitableList {
        hitables: vec![]
    };

    let mut spheres = vec![];

    spheres.push(Sphere {
        center: Vec3 { x: 0.0, y: -1000.0, z: 0.0 },
        radius: 1000.0,
        material: Material::Lambertian(Vec3 { x: 0.5, y: 0.5, z: 0.5 }),
    });

    let what = Vec3 { x: 4.0, y: 0.2, z: 0.0 };

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center = Vec3 { x: a as f64 + 0.9 * rand::random::<f64>(), y: 0.2, z: b as f64 + 0.9 * rand::random::<f64>() };
            if (&center - &what).length() > 0.9 {
                if choose_mat < 0.8 { // diffuse
                    spheres.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Lambertian(Vec3 {
                            x: rand::random::<f64>() * rand::random::<f64>(),
                            y: rand::random::<f64>() * rand::random::<f64>(),
                            z: rand::random::<f64>() * rand::random::<f64>(),
                        }),
                    });
                } else if choose_mat < 0.95 { // metal
                    spheres.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Metal(Vec3 {
                            x: 0.5 * (1.0 + rand::random::<f64>()),
                            y: 0.5 * (1.0 + rand::random::<f64>()),
                            z: 0.5 * (1.0 + rand::random::<f64>()),
                        }, 0.5 * rand::random::<f64>()),
                    });
                } else { // glass
                    spheres.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Dielectric(1.5),
                    });
                }
            }
        }
    }

//    spheres.push(Sphere {
//        center: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
//        radius: 1.0,
//        material: Material::Dielectric(1.5),
//    });
//
//    spheres.push(Sphere {
//        center: Vec3 { x: -4.0, y: 1.0, z: 0.0 },
//        radius: 1.0,
//        material: Material::Lambertian(Vec3 {
//            x: 0.4,
//            y: 0.2,
//            z: 0.1,
//        }),
//    });
//
//
//    spheres.push(Sphere {
//        center: Vec3 { x: 4.0, y: 1.0, z: 0.0 },
//        radius: 1.0,
//        material: Material::Metal(Vec3 {
//            x: 0.8,
//            y: 0.8,
//            z: 0.85,
//        }, 0.0),
//    });

    spheres.push(Sphere {
        center: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        radius: 2.0,
        material: Material::Metal(Vec3 {
            x: 0.8,
            y: 0.8,
            z: 0.85,
        }, 0.0),
    });

    for sphere in spheres {
        world.hitables.push(Box::new(sphere));
    }

    let lookfrom = Vec3 { x: 13.0, y: 2.0, z: 3.0 };
    let lookat = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(lookfrom, lookat,
                          Vec3 { x: 0.0, y: 1.0, z: 0.0 },
                          20.0, NX as f64 / NY as f64, aperture, dist_to_focus);

    return (cam, world);
}
//fn world_setup_2() -> (Camera, HitableList) {
//    let camera = Camera::new(90.0, 2.0);
//
//    let r = (PI / 4.0).cos();
//    let sphere1 = Sphere {
//        center: Vec3 { x: -r, y: 0.0, z: -1.0 },
//        radius: r,
//        material: Material::Lambertian(Vec3 { x: 0.0, y: 0.0, z: 1.0 }),
//    };
//    let sphere2 = Sphere {
//        center: Vec3 { x: r, y: 0.0, z: -1.0 },
//        radius: r,
//        material: Material::Lambertian(Vec3 { x: 1.0, y: 0.0, z: 0.0 }),
//    };
//
//    let world = HitableList {
//        hitables: vec![
//            sphere1),
//            sphere2)]
//    };
//    (camera, world)
//}


fn color(ray: &Ray, world: &HitableList, depth: u8) -> Vec3 {
    match world.hit(&ray, 0.0001, std::f64::MAX) {
        Some(hit_record) => {
            match hit_record.material.scatter(ray, &hit_record) {
                Some((scattered_ray, attenuation)) => {
                    if depth < 50 {
                        return &attenuation * &color(&scattered_ray, world, depth + 1);
                    }
                }
                _ => { return Vec3 { x: 0.0, y: 0.0, z: 0.0 }; }
            }
            Vec3 { x: 0.0, y: 0.0, z: 0.0 }
        }
        None => {
            let unit_direction = ray.direction.unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);
            &(&Vec3 { x: 1.0, y: 1.0, z: 1.0 } * &(1.0 - t)) + &(&Vec3 { x: 0.5, y: 0.7, z: 1.0 } * &t)
        }
    }
}