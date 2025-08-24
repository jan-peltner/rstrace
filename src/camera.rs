use crate::{
    image::Image,
    ray::{Hittables, Ray3},
    utils::{lerp, Interval},
    vec::{Pixel, Point3, Vec3},
};
use core::f64;
use rand::{rngs::ThreadRng, Rng};
use std::cell::RefCell;

// make Camera generic over R so we can potentially use different rngs later
pub struct Camera<R: Rng> {
    img_w: u32,
    img_h: u32,
    center: Point3,
    px00: Point3,
    px_delta_u: Vec3,
    px_delta_v: Vec3,
    rays_per_pixel: u32,
    max_bounces: u32,
    rng: RefCell<R>,
}

impl<R: Rng> Camera<R> {
    pub fn new(
        img_w: u32,
        ar: f64,
        camera_center: Point3,
        rays_per_pixel: u32,
        max_bounces: u32,
        rng: R,
    ) -> Self {
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

        // Pixel spacing -> the amount of world units that one image pixel takes up on the viewport
        // Multiplying the pixel coordinates (i, j) by these deltas moves us to the
        // corresponding location on the viewport plane
        let px_delta_u = &vp_u / img_w as f64;
        let px_delta_v = &vp_v / img_h as f64;

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
        let px00 = &vp_upper_left + &((&px_delta_u + &px_delta_v) * 0.5);

        Camera {
            img_w,
            img_h,
            center: camera_center,
            px00,
            px_delta_u,
            px_delta_v,
            rays_per_pixel,
            max_bounces,
            rng: RefCell::new(rng),
        }
    }

    pub fn with_default_rng(
        img_w: u32,
        ar: f64,
        camera_center: Point3,
        rays_per_pixel: u32,
        max_bounces: u32,
    ) -> Camera<ThreadRng> {
        Camera::new(
            img_w,
            ar,
            camera_center,
            rays_per_pixel,
            max_bounces,
            rand::rng(),
        )
    }

    pub fn render(&self, world: Hittables) {
        let image = Image::new(self.img_w, self.img_h, |x, y| {
            let mut px = Pixel::zero();
            let rng = &mut self.rng.borrow_mut();

            for _ in 0..self.rays_per_pixel {
                let ray = self.get_ray(x, y, rng);
                px = px + self.color_ray(&ray, &world, self.max_bounces, rng);
            }

            px / self.rays_per_pixel as f64
        });

        println!("{}", image);
    }

    fn color_ray(&self, ray: &Ray3, world: &Hittables, bounces_left: u32, rng: &mut R) -> Pixel {
        if bounces_left <= 0 {
            return Pixel {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }

        let mut t_range = Interval {
            min: 0.001,
            max: f64::INFINITY,
        };

        if let Some(hit) = world.check_hit(&ray, &mut t_range) {
            if let Some(scatter) = hit.mat.scatter(ray, &hit, rng) {
                return &self.color_ray(
                    &Ray3 {
                        dir: scatter.scattered_ray.dir,
                        origin: hit.p,
                    },
                    world,
                    bounces_left - 1,
                    rng,
                ) * scatter.attenuation;
            } else {
                return Pixel::zero();
            }
        } else {
            let interpolant = 0.5 * (ray.dir.y + 1.0);

            let r = lerp(1.0, 0.5, interpolant);
            let g = lerp(1.0, 0.7, interpolant);
            let b = 1.0;

            return Pixel {
                x: Image::map_to_rgb_space(r),
                y: Image::map_to_rgb_space(g),
                z: Image::map_to_rgb_space(b),
            };
        }
    }

    fn get_ray(&self, i: u32, j: u32, rng: &mut R) -> Ray3 {
        let square_offset = Vec3::rand_unit_square_offset(rng);

        let px_sample = &(&self.px00 + &(&self.px_delta_u * (i as f64 + square_offset.x)))
            + &(&self.px_delta_v * (j as f64 + square_offset.y));

        let dir = (&px_sample - &self.center).norm();

        Ray3 {
            origin: self.center.clone(),
            dir,
        }
    }
}
