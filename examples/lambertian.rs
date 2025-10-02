use std::rc::Rc;
use std::time::Instant;

use rstrace::bvh::BvhNode;
use rstrace::camera::Camera;
use rstrace::geometry::Sphere;
use rstrace::ray::Hittables;
use rstrace::vec::*;

fn main() {
    // --- Camera ---
    let camera = Camera::default();

    // --- World ---
    let central_sphere = Rc::from(Sphere::lambertian(
        0.5,
        Point {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
    ));

    let world_sphere = Rc::from(Sphere::lambertian(
        100.0,
        Point {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
    ));

    let mut world = Hittables::from_vec(vec![central_sphere, world_sphere]);
    let world_root = BvhNode::from_hittables(&mut world.objects, &mut rand::rng());

    // --- Render ---
    let now = Instant::now();
    let _ = camera.render(Rc::from(world), "lambertian_flat.ppm");
    let time = now.elapsed().as_secs();
    println!("Flat time: {time} secs");

    let now = Instant::now();
    let _ = camera.render(world_root, "lambertian_bvh.ppm");
    let time = now.elapsed().as_secs();
    println!("Bvh time: {time} secs");
}
