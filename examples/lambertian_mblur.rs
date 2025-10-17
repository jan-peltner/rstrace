use std::rc::Rc;

use rstrace::camera::{Camera, CameraIntrinsics, CameraPose};
use rstrace::geometry::Sphere;
use rstrace::ray::Hittables;
use rstrace::vec::*;

fn main() {
    // --- Camera ---
    let mut intrinsics = CameraIntrinsics::default();
    intrinsics.vfov = 90.0;
    intrinsics.img_w = 800;
    intrinsics.rays_per_pixel = 1000;
    intrinsics.max_bounces = 50;

    let camera = Camera::new(intrinsics, CameraPose::default(), rand::rng());

    // --- World ---
    let central_sphere = Rc::from(
        Sphere::lambertian_with_default_albedo(
            0.5,
            Point {
                x: 0.0,
                y: 1.0,
                z: -1.0,
            },
        )
        .add_movement(Point {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        }),
    );

    let world_sphere = Rc::from(Sphere::lambertian_with_default_albedo(
        100.0,
        Point {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
    ));

    let world = Hittables::from_vec(vec![central_sphere, world_sphere]);

    // --- Render ---
    let _ = camera.render(Rc::from(world), "lambertian_mblur.ppm");
}
