use rand::rngs::ThreadRng;
use rstrace::bvh::BvhNode;
use rstrace::camera::{Camera, CameraIntrinsics, CameraPose};
use rstrace::geometry::{ConstantMedium, Quad, Sphere};
use rstrace::material::{Emitter, Isotropic, Lambertian, Metal};
use rstrace::ray::Hittables;
use rstrace::texture::{ImageTex, SolidTex};
use rstrace::vec::*;

fn main() {
    // --- Camera ---
    let mut intrinsics = CameraIntrinsics::default();
    intrinsics.max_bounces = 10;
    intrinsics.rays_per_pixel = 100;
    intrinsics.background = Color::zero();
    intrinsics.img_w = 1000;

    let pose = CameraPose::default();
    let camera = Camera::new_default_rng(intrinsics, pose);
    let mut rng = camera.get_rng();

    // --- Textures ---
    let earth_tex = ImageTex::new("assets/textures/earth.jpg").unwrap();
    let mars_tex = ImageTex::new("assets/textures/mars.jpg").unwrap();
    let moon_tex = ImageTex::new("assets/textures/moon.jpg").unwrap();
    let fog_tex = SolidTex::new(Color::white());

    // --- Materials ---
    let earth_mat = Metal::new(earth_tex, 1.0);
    let mars_mat = Metal::new(mars_tex, 1.0);
    let moon_mat = Metal::new(moon_tex, 1.0);
    let fog_mat = Isotropic::new(fog_tex);

    let light_mat = Emitter::new(SolidTex::new(Color {
        x: 7.0,
        y: 7.0,
        z: 7.0,
    }));

    // --- Geometry ---
    let earth = Sphere::new_arc(
        0.5,
        Point {
            x: -1.5,
            y: 0.0,
            z: -2.0,
        },
        earth_mat,
    );

    let mars = Sphere::new_arc(
        0.5,
        Point {
            x: 0.0,
            y: 0.0,
            z: -2.0,
        },
        mars_mat,
    );

    let moon = Sphere::new_arc(
        0.5,
        Point {
            x: 1.5,
            y: 0.0,
            z: -2.0,
        },
        moon_mat,
    );

    let light = Quad::new_arc(
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

    let world_floor = Sphere::new_arc(
        100.0,
        Point {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        Metal::new(SolidTex::new((88, 91, 112).into()), 1.0),
    );

    let fog_bound = Sphere::new_arc(
        100.0,
        Point {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        Lambertian::new(SolidTex::white()),
    );

    let fog = ConstantMedium::<ThreadRng>::new_arc(fog_bound, 0.1, fog_mat);
    // --- World ---
    let mut world = Hittables::from_vec(vec![earth, mars, moon, light, world_floor, fog]);
    let world_root = BvhNode::from_hittables(&mut world.objects, &mut rng);

    // --- Render ---
    let _ = camera.render(world_root, "planets4.png");
}
