use rstrace::bvh::BvhNode;
use rstrace::camera::{Camera, CameraIntrinsics, CameraPose};
use rstrace::geometry::Quad;
use rstrace::material::{Emitter, Lambertian};
use rstrace::ray::Hittables;
use rstrace::texture::SolidTex;
use rstrace::vec::*;

fn main() {
    // --- Camera ---
    let mut intrinsics = CameraIntrinsics::default();
    intrinsics.ar = 1.0;
    intrinsics.img_w = 600;
    intrinsics.rays_per_pixel = 1000;
    intrinsics.max_bounces = 50;
    intrinsics.vfov = 40.0;
    intrinsics.background = Color::zero();

    let mut pose = CameraPose::default();
    pose.lookfrom = Point {
        x: 278.0,
        y: 278.0,
        z: -800.0,
    };
    pose.lookat = Point {
        x: 278.0,
        y: 278.0,
        z: 0.0,
    };

    let camera = Camera::with_default_rng(intrinsics, pose);

    let red = Lambertian::new(SolidTex::new(Color {
        x: 0.65,
        y: 0.05,
        z: 0.05,
    }));
    let green = Lambertian::new(SolidTex::new(Color {
        x: 0.12,
        y: 0.45,
        z: 0.15,
    }));
    let white = Lambertian::new(SolidTex::new(Color {
        x: 0.73,
        y: 0.73,
        z: 0.73,
    }));
    let light = Emitter::new(SolidTex::new(Color {
        x: 15.0,
        y: 15.0,
        z: 15.0,
    }));

    let right_quad = Quad::new_rc(
        Point {
            x: 555.0,
            y: 0.0,
            z: 0.0,
        },
        Vec3 {
            x: 0.0,
            y: 555.0,
            z: 0.0,
        },
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 555.0,
        },
        green,
    );

    let left_quad = Quad::new_rc(
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Vec3 {
            x: 0.0,
            y: 555.0,
            z: 0.0,
        },
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 555.0,
        },
        red,
    );

    let light_quad = Quad::new_rc(
        Point {
            x: 343.0,
            y: 554.0,
            z: 332.0,
        },
        Vec3 {
            x: -130.0,
            y: 0.0,
            z: 0.0,
        },
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: -105.0,
        },
        light,
    );

    let floor_quad = Quad::new_rc(
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Vec3 {
            x: 555.0,
            y: 0.0,
            z: 0.0,
        },
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 555.0,
        },
        white.clone(),
    );

    let ceiling_quad = Quad::new_rc(
        Point {
            x: 555.0,
            y: 555.0,
            z: 555.0,
        },
        Vec3 {
            x: -555.0,
            y: 0.0,
            z: 0.0,
        },
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: -555.0,
        },
        white.clone(),
    );

    let back_wall_quad = Quad::new_rc(
        Point {
            x: 0.0,
            y: 0.0,
            z: 555.0,
        },
        Vec3 {
            x: 555.0,
            y: 0.0,
            z: 0.0,
        },
        Vec3 {
            x: 0.0,
            y: 555.0,
            z: 0.0,
        },
        white.clone(),
    );

    let front_box = Quad::spawn_box(
        Point {
            x: 130.0,
            y: 0.0,
            z: 65.0,
        },
        Point {
            x: 295.0,
            y: 165.0,
            z: 230.0,
        },
        white.clone(),
    );

    let back_box = Quad::spawn_box(
        Point {
            x: 265.0,
            y: 0.0,
            z: 295.0,
        },
        Point {
            x: 430.0,
            y: 330.0,
            z: 460.0,
        },
        white.clone(),
    );

    let mut world = Hittables::from_vec(vec![
        right_quad,
        left_quad,
        light_quad,
        floor_quad,
        ceiling_quad,
        back_wall_quad,
    ]);
    world.extend(front_box);
    world.extend(back_box);

    let world_root = BvhNode::from_hittables(&mut world.objects, &mut rand::rng());

    // --- Render ---
    let _ = camera.render(world_root, "cornell.ppm");
}
