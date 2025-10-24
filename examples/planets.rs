use rstrace::bvh::BvhNode;
use rstrace::camera::{Camera, CameraIntrinsics, CameraPose};
use rstrace::geometry::{Quad, Sphere};
use rstrace::material::{Emitter, Metal};
use rstrace::ray::Hittables;
use rstrace::texture::{ImageTex, SolidTex};
use rstrace::vec::*;

fn main() {
    // --- Camera ---
    let mut intrinsics = CameraIntrinsics::default();
    intrinsics.max_bounces = 50;
    intrinsics.rays_per_pixel = 200;
    intrinsics.background = Color::zero();

    let pose = CameraPose::default();
    let camera = Camera::with_default_rng(intrinsics, pose);

    // --- Textures ---
    let earth_tex = ImageTex::new("assets/textures/earth.jpg").unwrap();
    let mars_tex = ImageTex::new("assets/textures/mars.jpg").unwrap();
    let moon_tex = ImageTex::new("assets/textures/moon.jpg").unwrap();

    // --- Materials ---
    let earth_mat = Metal::new(earth_tex, 1.0);
    let mars_mat = Metal::new(mars_tex, 1.0);
    let moon_mat = Metal::new(moon_tex, 1.0);

    let light_mat = Emitter::new(SolidTex::new(Color {
        x: 15.0,
        y: 15.0,
        z: 15.0,
    }));

    // --- Geometry ---
    let earth = Sphere::new(
        0.5,
        Point {
            x: -1.5,
            y: 0.0,
            z: -2.0,
        },
        earth_mat,
    );

    let mars = Sphere::new(
        0.5,
        Point {
            x: 0.0,
            y: 0.0,
            z: -2.0,
        },
        mars_mat,
    );

    let moon = Sphere::new(
        0.5,
        Point {
            x: 1.5,
            y: 0.0,
            z: -2.0,
        },
        moon_mat,
    );

    let light = Quad::new(
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
        light_mat,
    );

    let world_floor = Sphere::new(
        100.0,
        Point {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        Metal::new(SolidTex::new((88, 91, 112).into()), 1.0),
    );

    // --- World ---
    let mut world = Hittables::from_vec(vec![earth, mars, moon, light, world_floor]);
    let world_root = BvhNode::from_hittables(&mut world.objects, &mut rand::rng());

    // --- Render ---
    let _ = camera.render(world_root, "planets.ppm");
}
