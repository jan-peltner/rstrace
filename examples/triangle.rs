use rand::rngs::ThreadRng;
use rstrace::bvh::BvhNode;
use rstrace::camera::Camera;
use rstrace::geometry::{Sphere, Triangle};
use rstrace::material::Lambertian;
use rstrace::ray::Hittables;
use rstrace::texture::SolidTex;
use rstrace::vec::*;

fn main() {
    // --- Camera ---
    let camera = Camera::<ThreadRng>::default();
    let mut rng = camera.get_rng();

    // --- World ---
    let mut world = Hittables::new();

    let world_mat = Lambertian::new(SolidTex::new(Color {
        x: 0.5,
        y: 0.5,
        z: 0.5,
    }));

    let tri_mat = Lambertian::new(SolidTex::new(Color {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    }));

    world.add(Sphere::new_arc(
        100.0,
        Point {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        world_mat,
    ));

    world.add(Triangle::new_arc(
        Point {
            x: -0.25,
            y: 0.0,
            z: -1.0,
        },
        Point {
            x: 0.5,
            y: 0.0,
            z: 0.0,
        },
        Point {
            x: 0.25,
            y: 0.5,
            z: 0.0,
        },
        tri_mat,
    ));

    let world_root = BvhNode::from_hittables(&mut world.objects, &mut rng);
    let _ = camera.render(world_root, "triangle2.png");
}
