use rand::rngs::ThreadRng;
use rand::{random, Rng};
use rstrace::bvh::BvhNode;
use rstrace::camera::{Camera, CameraIntrinsics, CameraPose};
use rstrace::geometry::Sphere;
use rstrace::ray::Hittables;
use rstrace::texture::CheckerTex;
use rstrace::vec::*;
use std::rc::Rc;

fn main() {
    // --- Camera ---
    let mut intrinsics = CameraIntrinsics::default();
    intrinsics.vfov = 20.0;
    intrinsics.img_w = 800;
    intrinsics.rays_per_pixel = 100;
    intrinsics.max_bounces = 50;

    let pose = CameraPose {
        lookfrom: Point {
            x: 13.0,
            y: 2.0,
            z: 3.0,
        },
        lookat: Point::zero(),
        vup: Point {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
    };

    let camera = Camera::<ThreadRng>::new(intrinsics, pose, rand::rng());
    let mut rng = rand::rng();

    // --- World ---
    let mut world = Hittables::new();

    world.add(Rc::from(Sphere::lambertian_with_texture(
        1000.0,
        Point {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        CheckerTex::new(Color::green(), Color::white(), 0.32),
    )));

    for i in -11..11 {
        for j in -11..11 {
            let rand_mat_sample = random::<f64>();
            let center = Point {
                x: i as f64 + 0.9 * random::<f64>(),
                y: 0.2,
                z: j as f64 + 0.9 * random::<f64>(),
            };

            if (&center
                - &Point {
                    x: 4.0,
                    y: 0.2,
                    z: 0.0,
                })
                .len()
                > 0.9
            {
                if rand_mat_sample < 0.8 {
                    let albedo = Color::rand(&mut rng);
                    world.add(Rc::new(Sphere::lambertian_with_albedo(0.2, center, albedo)));
                } else if rand_mat_sample < 0.95 {
                    let albedo = Color::rand_range(&mut rng, 0.0, 0.5);
                    let fuzz = rng.random_range(0.0..0.5);
                    world.add(Rc::new(Sphere::metal_with_albedo(
                        0.2, center, albedo, fuzz,
                    )));
                } else {
                    world.add(Rc::new(Sphere::dielectric(0.2, center, 1.5)));
                }
            }
        }
    }

    world.add(Rc::new(Sphere::dielectric(
        1.0,
        Point {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        1.9,
    )));
    world.add(Rc::new(Sphere::lambertian_with_albedo(
        1.0,
        Point {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        Color {
            x: 0.4,
            y: 0.2,
            z: 0.1,
        },
    )));
    world.add(Rc::new(Sphere::metal_with_albedo(
        1.0,
        Point {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        Color {
            x: 0.7,
            y: 0.6,
            z: 0.5,
        },
        0.0,
    )));

    let world_root = BvhNode::from_hittables(&mut world.objects, &mut rng);

    // --- Render ---
    let _ = camera.render(world_root, "checker_tex.ppm");
}
