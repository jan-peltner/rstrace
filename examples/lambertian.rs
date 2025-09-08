use rand::rngs::ThreadRng;
use rstrace::camera::Camera;
use rstrace::geometry::Sphere;
use rstrace::ray::Hittables;
use rstrace::vec::*;

fn main() {
    // --- Camera ---
    let camera = Camera::<ThreadRng>::with_default_rng_and_pose(1600, 16.0 / 9.0, 10, 10, 90.0);

    // --- World ---
    let central_sphere = Box::from(Sphere::lambertian(
        0.5,
        Point {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
    ));

    let world_sphere = Box::from(Sphere::lambertian(
        100.0,
        Point {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
    ));

    let world = Hittables {
        objects: vec![central_sphere, world_sphere],
    };

    // --- Render ---
    let _ = camera.render(world, "lambertian_sphere.ppm");
}
