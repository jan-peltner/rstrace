use rand::rngs::ThreadRng;
use rstrace::camera::Camera;
use rstrace::geometry::Sphere;
use rstrace::ray::Hittables;
use rstrace::vec::*;

fn main() {
    // --- Camera ---
    let camera = Camera::<ThreadRng>::with_default_rng(1600, 16.0 / 9.0, Point3::zero(), 1000, 100);

    // --- World ---
    let world_sphere = Box::from(Sphere::lambertian_with_albedo(
        100.0,
        Point3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        Vec3 {
            x: 0.8,
            y: 0.8,
            z: 0.0,
        },
    ));

    let center_sphere = Box::from(Sphere::lambertian_with_albedo(
        0.5,
        Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.2,
        },
        Vec3 {
            x: 0.1,
            y: 0.2,
            z: 0.5,
        },
    ));

    let left_sphere = Box::from(Sphere::metal_with_albedo(
        0.5,
        Point3 {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        Vec3 {
            x: 0.8,
            y: 0.8,
            z: 0.8,
        },
    ));
    let right_sphere = Box::from(Sphere::metal_with_albedo(
        0.5,
        Point3 {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        },
        Vec3 {
            x: 0.8,
            y: 0.6,
            z: 0.2,
        },
    ));

    let world = Hittables {
        objects: vec![center_sphere, world_sphere, left_sphere, right_sphere],
    };

    // --- Render ---
    camera.render(world);
}
