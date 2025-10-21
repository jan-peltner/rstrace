use std::rc::Rc;

use rstrace::bvh::BvhNode;
use rstrace::camera::{Camera, CameraIntrinsics, CameraPose};
use rstrace::geometry::{Quad, Sphere};
use rstrace::ray::Hittables;
use rstrace::texture::{ImageTex, SolidTex};
use rstrace::vec::*;

fn main() {
    // --- Camera ---
    let mut intrinsics = CameraIntrinsics::default();
    intrinsics.max_bounces = 50;
    intrinsics.rays_per_pixel = 1000;
    intrinsics.background = Color {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let pose = CameraPose::default();
    let camera = Camera::with_default_rng(intrinsics, pose);

    // --- World ---
    let earth = Rc::from(Sphere::metal_with_texture(
        0.5,
        Point {
            x: -1.5,
            y: 0.0,
            z: -2.0,
        },
        ImageTex::new("assets/textures/earth.jpg").unwrap(),
        1.0,
    ));

    let mars = Rc::from(Sphere::metal_with_texture(
        0.5,
        Point {
            x: 0.0,
            y: 0.0,
            z: -2.0,
        },
        ImageTex::new("assets/textures/mars.jpg").unwrap(),
        1.0,
    ));

    let moon = Rc::from(Sphere::metal_with_texture(
        0.5,
        Point {
            x: 1.5,
            y: 0.0,
            z: -2.0,
        },
        ImageTex::new("assets/textures/moon.jpg").unwrap(),
        1.0,
    ));

    let light = Rc::from(Quad::emitter_with_texture(
        Point {
            x: -2.0,
            y: 1.5,
            z: -2.5,
        },
        Vec3 {
            x: 0.0,
            y: 0.5,
            z: 0.5,
        },
        Vec3 {
            x: 4.0,
            y: 0.0,
            z: 0.0,
        },
        ImageTex::new("assets/textures/light.jpg").unwrap(),
    ));

    let world_sphere = Sphere::lambertian_with_texture(
        100.0,
        Point {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        SolidTex::new((88, 91, 112).into()),
    );
    let world_sphere = Rc::from(world_sphere);

    let mut world = Hittables::from_vec(vec![mars, world_sphere, earth, moon, light]);
    let world_root = BvhNode::from_hittables(&mut world.objects, &mut rand::rng());

    // --- Render ---
    let _ = camera.render(world_root, "planets.ppm");
}
