use std::rc::Rc;

use rstrace::bvh::BvhNode;
use rstrace::camera::Camera;
use rstrace::geometry::Sphere;
use rstrace::ray::Hittables;
use rstrace::texture::StripeTex;
use rstrace::vec::*;

fn main() {
    // --- Camera ---
    let camera = Camera::default();

    // --- World ---
    let central_sphere = Rc::from(Sphere::lambertian_with_texture(
        0.5,
        Point {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        StripeTex::new(Color::green(), Color::red(), 0.5f64.powi(4)),
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
    let _ = camera.render(world_root, "stripe_tex.ppm");
}
