use std::rc::Rc;

use rstrace::camera::Camera;
use rstrace::geometry::Sphere;
use rstrace::ray::Hittables;
use rstrace::vec::*;

fn main() {
    // --- Camera ---
    let camera = Camera::default();

    let r = (std::f64::consts::PI / 4.0).cos();

    let left_sphere = Rc::from(Sphere::lambertian_with_albedo(
        r,
        Point {
            x: -r,
            y: 0.0,
            z: -1.0,
        },
        Color::blue(),
    ));

    let right_sphere = Rc::from(Sphere::lambertian_with_albedo(
        r,
        Point {
            x: r,
            y: 0.0,
            z: -1.0,
        },
        Color::red(),
    ));

    let world = Hittables::from_vec(vec![left_sphere, right_sphere]);

    // --- Render ---
    let _ = camera.render(Rc::from(world), "fov90.ppm");
}
