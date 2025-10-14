use std::rc::Rc;

use rand::rngs::ThreadRng;
use rstrace::bvh::BvhNode;
use rstrace::camera::{Camera, CameraIntrinsics, CameraPose};
use rstrace::geometry::Sphere;
use rstrace::ray::Hittables;
use rstrace::texture::{ImageTex, SolidTex};
use rstrace::vec::*;

fn main() {
    // --- Camera ---
    let mut intrinsics = CameraIntrinsics::default();
    intrinsics.max_bounces = 50;
    intrinsics.rays_per_pixel = 100;

    let pose = CameraPose::default();
    let camera = Camera::<ThreadRng>::with_default_rng(intrinsics, pose);

    // --- World ---
    let central_sphere = Rc::from(Sphere::lambertian_with_texture(
        0.5,
        Point {
            x: 0.0,
            y: 0.0,
            z: -2.0,
        },
        ImageTex::new("assets/earth_tex.png").unwrap(),
    ));

    let world_sphere = Rc::from(Sphere::lambertian_with_texture(
        100.0,
        Point {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        SolidTex::new((88, 91, 112).into()),
    ));

    let mut world = Hittables::from_vec(vec![central_sphere, world_sphere]);
    let world_root = BvhNode::from_hittables(&mut world.objects, &mut rand::rng());

    // --- Render ---
    let _ = camera.render(world_root, "image_tex.ppm");
}
