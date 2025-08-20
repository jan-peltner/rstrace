use crate::{
    image::Image,
    ray::{Hittables, Ray3},
    utils::{lerp, Interval},
    vec::{Pixel, Point3, Vec3},
};

pub struct Camera {
    img_w: u32,
    img_h: u32,
    center: Point3,
    px00: Point3,
    px_delta_u: Vec3,
    px_delta_v: Vec3,
}

impl Camera {
    pub fn new(img_w: u32, ar: f64, center: Point3) -> Self {
        let img_h = Image::compute_height(img_w, ar);

        let focal_length = 1.0; // Distance from camera to the viewport in world units
        let vp_h = 2.0; // Viewport height in world units

        // We recompute the aspect ratio here because the actual ratio can be different since img_w and
        // img_h are casted to u32s
        let vp_w = vp_h * (img_w as f64 / img_h as f64);

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
        let px_delta_u = &vp_u / img_w as f64;
        let px_delta_v = &vp_v / img_h as f64;

        // Move -1 on the z-axis to reach the viewport plane, move half of the viewport width to the
        // left, move half of the viewport height up
        let vp_upper_left = &center
            - &Vec3 {
                x: 0.0,
                y: 0.0,
                z: focal_length,
            }
            - &vp_u / 2.0
            - &vp_v / 2.0;

        // Inset the pixel grid by half a unit from the viewport edges
        let px00 = &vp_upper_left + &((&px_delta_u + &px_delta_v) * 0.5);

        Camera {
            img_w,
            img_h,
            center,
            px00,
            px_delta_u,
            px_delta_v,
        }
    }

    pub fn render(&self, world: Hittables) {
        let image = Image::new(self.img_w, self.img_h, |x, y| {
            let pixel_center =
                (&self.px00 + &(&self.px_delta_u * x as f64)) + (&self.px_delta_v * y as f64);
            let ray_dir = (&pixel_center - &self.center).norm();
            let r = Ray3 {
                origin: self.center.clone(),
                dir: ray_dir,
            };

            let mut t_range = Interval {
                min: 0.0,
                max: 100.0,
            };
            if let Some(hit) = world.check_hit(&r, &mut t_range) {
                Pixel {
                    x: (hit.normal.x + 1.0) * 255.99 * 0.5,
                    y: (hit.normal.y + 1.0) * 255.99 * 0.5,
                    z: (hit.normal.z + 1.0) * 255.99 * 0.5,
                }
            } else {
                let t = 0.5 * (r.dir.y + 1.0);

                Pixel {
                    x: lerp(1.0, 0.5, t) * 255.99,
                    y: lerp(1.0, 0.7, t) * 255.99,
                    z: 1.0 * 255.99,
                }
            }
        });

        println!("{}", image);
    }
}
