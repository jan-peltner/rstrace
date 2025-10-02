use std::rc::Rc;

use rand::rngs::ThreadRng;
use rstrace::camera::{Camera, CameraIntrinsics, CameraPose};
use rstrace::geometry::Sphere;
use rstrace::ray::Hittables;
use rstrace::vec::*;

fn main() {
    // --- Camera ---
    let pose = CameraPose {
        lookfrom: Vec3 {
            x: 2.0,
            y: 2.0,
            z: 1.0,
        },
        lookat: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        vup: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
    };
    let camera = Camera::<ThreadRng>::with_default_rng(CameraIntrinsics::default(), pose);

    // --- World ---
    let world_sphere = Rc::from(Sphere::lambertian_with_albedo(
        100.0,
        Point {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        Color {
            x: 0.8,
            y: 0.8,
            z: 0.0,
        },
    ));

    let center_sphere = Rc::from(Sphere::lambertian_with_albedo(
        0.5,
        Point {
            x: 0.0,
            y: 0.0,
            z: -1.2,
        },
        Color {
            x: 0.1,
            y: 0.2,
            z: 0.5,
        },
    ));

    let left_sphere = Rc::from(Sphere::metal_with_albedo(
        0.5,
        Point {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        Color {
            x: 0.8,
            y: 0.8,
            z: 0.8,
        },
        0.0,
    ));
    let right_sphere = Rc::from(Sphere::metal_with_albedo(
        0.5,
        Point {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        },
        Color {
            x: 0.8,
            y: 0.6,
            z: 0.2,
        },
        0.5,
    ));

    let world = Hittables::from_vec(vec![center_sphere, left_sphere, right_sphere, world_sphere]);

    // --- Render ---
    let _ = camera.render(Rc::from(world), "metal_pose.ppm");
}
