use rand::{random, Rng};
use rstrace::bvh::BvhNode;
use rstrace::camera::{Camera, CameraIntrinsics, CameraPose};
use rstrace::geometry::Sphere;
use rstrace::material::{Dielectric, Lambertian, Metal};
use rstrace::ray::Hittables;
use rstrace::texture::SolidTex;
use rstrace::vec::*;

fn main() {
    // --- Camera ---
    let mut intrinsics = CameraIntrinsics::default();
    intrinsics.vfov = 20.0;
    intrinsics.img_w = 800;
    intrinsics.rays_per_pixel = 100;
    intrinsics.max_bounces = 50;

    let pose = CameraPose {
        lookfrom: Point {
            x: 13.0,
            y: 2.0,
            z: 3.0,
        },
        lookat: Point::zero(),
        vup: Point {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
    };

    let camera = Camera::new_default_rng(intrinsics, pose);
    let mut rng = rand::rng();

    // --- World ---
    let mut world = Hittables::new();

    let world_mat = Lambertian::new(SolidTex::new(Color {
        x: 0.5,
        y: 0.5,
        z: 0.5,
    }));
    world.add(Sphere::new_arc(
        1000.0,
        Point {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        world_mat,
    ));

    for i in -11..11 {
        for j in -11..11 {
            let rand_mat_sample = random::<f64>();
            let center = Point {
                x: i as f64 + 0.9 * random::<f64>(),
                y: 0.2,
                z: j as f64 + 0.9 * random::<f64>(),
            };

            if (&center
                - &Point {
                    x: 4.0,
                    y: 0.2,
                    z: 0.0,
                })
                .len()
                > 0.9
            {
                if rand_mat_sample < 0.8 {
                    let mat = Lambertian::new(SolidTex::new(Color::rand(&mut rng)));
                    world.add(Sphere::new_arc(0.2, center, mat));
                } else if rand_mat_sample < 0.95 {
                    let mat = Metal::new(
                        SolidTex::new(Color::rand_range(&mut rng, 0.0, 0.5)),
                        rng.random_range(0.0..0.5),
                    );
                    world.add(Sphere::new_arc(0.2, center, mat));
                } else {
                    let mat = Dielectric::new(1.5);
                    world.add(Sphere::new_arc(0.2, center, mat));
                }
            }
        }
    }

    world.add(Sphere::new_arc(
        1.0,
        Point {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        Dielectric::new(1.9),
    ));
    world.add(Sphere::new_arc(
        1.0,
        Point {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        Lambertian::new(SolidTex::new(Color {
            x: 0.4,
            y: 0.2,
            z: 0.1,
        })),
    ));
    world.add(Sphere::new_arc(
        1.0,
        Point {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        Metal::new(
            SolidTex::new(Color {
                x: 0.7,
                y: 0.6,
                z: 0.5,
            }),
            0.0,
        ),
    ));

    let world_root = BvhNode::from_hittables(&mut world.objects, &mut rng);
    let _ = camera.render(world_root, "rtiow_image.ppm");
}
