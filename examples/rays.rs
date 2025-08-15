use rstrace::image::*;
use rstrace::ray::Ray3;
use rstrace::utils::lerp;
use rstrace::v3::*;

fn main() {
    // --- Image dimensions ---
    let img_w = 400;
    let ar = 16.0 / 9.0;
    let img_h = Image::compute_height(img_w, ar);

    // --- Camera & viewport ---
    let focal_length = 1.0; // Distance from camera to the viewport
    let vp_h = 2.0;
    // We recompute the aspect ratio here because the actual ratio can be different since img_w and
    // img_h are casted to u32s
    let vp_w = vp_h * (img_w as f64 / img_h as f64);
    let camera_center = Point3::zero();

    // Vectors along the edges (x and y axes) of the viewport
    let vp_u = Vec3 {
        x: vp_w,
        y: 0.0,
        z: 0.0,
    };

    let vp_v = Vec3 {
        x: 0.0,
        y: -vp_h,
        z: 0.0,
    };

    // Pixel spacing -> amount of wolrd units that one viewport pixel takes up
    let pixel_delta_u = &vp_u / img_w as f64;
    let pixel_delta_v = &vp_v / img_h as f64;

    // Move -1 on the z-axis to reach the viewport plane, move half of the viewport width to the
    // left, move half of the viewport height up
    let vp_upper_left = &camera_center
        - &Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        }
        - &vp_u / 2.0
        - &vp_v / 2.0;

    // Inset the pixel grid by half a unit from the viewport edges
    let pixel_00_pos = &vp_upper_left + &((&pixel_delta_u + &pixel_delta_v) * 0.5);

    let image = Image::new(img_w, img_h, |x, y| {
        let pixel_center =
            (&pixel_00_pos + &(&pixel_delta_u * x as f64)) + (&pixel_delta_v * y as f64);
        let ray_dir = (&pixel_center - &camera_center).norm();
        let r = Ray3 {
            origin: camera_center.clone(),
            dir: ray_dir,
        };

        let t = 0.5 * (r.dir.y + 1.0);

        return Pixel {
            x: lerp(1.0, 0.5, t) * 255.99,
            y: lerp(1.0, 0.7, t) * 255.99,
            z: 1.0 * 255.99,
        };
    });

    println!("{}", image);
}
