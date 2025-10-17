use std::rc::Rc;

use rand::rngs::ThreadRng;
use rstrace::camera::{Camera, CameraIntrinsics};
use rstrace::geometry::Sphere;
use rstrace::ray::Hittables;
use rstrace::texture::{ImageTex, SolidTex};
use rstrace::vec::*;

fn main() {
    // --- Camera ---
    let mut intrinsics = CameraIntrinsics::default();

    intrinsics.rays_per_pixel = 200;
    intrinsics.background = Color {
        x: 0.05,
        y: 0.05,
        z: 0.05,
    };

    let camera = Camera::<ThreadRng>::with_default_rng_and_pose(intrinsics);

    // --- World ---
    let world_sphere = Rc::from(Sphere::lambertian_with_albedo(
        100.0,
        Point {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        (88, 91, 112).into(),
    ));

    let earth = Rc::from(Sphere::metal_with_texture(
        0.5,
        Point {
            x: 0.0,
            y: 0.0,
            z: -1.2,
        },
        ImageTex::new("assets/textures/earth.jpg").unwrap(),
        0.5,
    ));

    let light = Rc::from(Sphere::emitter(
        0.75,
        Point {
            x: 0.0,
            y: 1.75,
            z: -1.25,
        },
        // ImageTex::new("assets/textures/light.avif").unwrap(),
        SolidTex::white(),
    ));

    let world = Hittables::from_vec(vec![world_sphere, earth, light]);

    // --- Render ---
    let _ = camera.render(Rc::from(world), "metal_earth.ppm");
}
