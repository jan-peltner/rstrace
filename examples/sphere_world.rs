use rstrace::camera::Camera;
use rstrace::geometry::Sphere;
use rstrace::ray::Hittables;
use rstrace::vec::*;

fn main() {
    // --- Camera ---
    let camera = Camera::new(1600, 16.0 / 9.0, Point3::zero(), 10);

    // --- World ---
    let central_sphere = Box::from(Sphere {
        center: Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
    });

    let world_sphere = Box::from(Sphere {
        center: Point3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
    });

    let world = Hittables {
        objects: vec![central_sphere, world_sphere],
    };

    // --- Render ---
    camera.render(world);
}
