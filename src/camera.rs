use crate::{
    image::Image,
    ray::{Hittables, Ray3},
    utils::{lerp, Interval},
    vec::{Pixel, Point, Vec3},
};
use core::f64;
use rand::{rngs::ThreadRng, Rng};
use std::{
    cell::RefCell,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

pub struct CameraPose {
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
}

// make Camera generic over R so we can potentially use different rngs later
pub struct Camera<R: Rng> {
    img_w: u32,
    img_h: u32,
    px00: Point,
    px_delta_u: Vec3,
    px_delta_v: Vec3,
    rays_per_pixel: u32,
    max_bounces: u32,
    pose: CameraPose,
    rng: RefCell<R>,
}

impl<R: Rng> Camera<R> {
    pub fn new(
        img_w: u32,
        ar: f64,
        rays_per_pixel: u32,
        max_bounces: u32,
        vfov: f64,
        pose: CameraPose,
        rng: R,
    ) -> Self {
        let img_h = Image::compute_height(img_w, ar);

        let focal_length = (&pose.lookfrom - &pose.lookat).len(); // Distance from camera to the viewport in world units

        // Half angle of vertical fov -> measured from z-axis to top
        let theta = vfov.to_radians() / 2.0;
        let h = theta.tan();
        let vp_h = h * 2.0 * focal_length; // Viewport height in world units

        // We recompute the aspect ratio here because the actual ratio can be different since img_w and
        // img_h are casted to u32s
        let vp_w = vp_h * (img_w as f64 / img_h as f64);

        // Vectors along the edges (x and y axes) of the viewport

        // Orthonormal basis
        // Direction that the camera looks at but in reverse
        let w = (&pose.lookfrom - &pose.lookat).norm();
        // Perpendicular vector to both vup and w, or in other words: normal vector to the plane
        // containing w and vup
        let u = &pose.vup.cross(&w).norm();
        // use the two perpendicular vectors w and u to compute the final "up" vector v
        let v = &u.cross(&w);

        let vp_u = u * vp_w;
        let vp_v = v * vp_h;

        // Pixel spacing -> the amount of world units that one image pixel takes up on the viewport
        // Multiplying the pixel coordinates (i, j) by these deltas moves us to the
        // corresponding location on the viewport plane
        let px_delta_u = &vp_u / img_w as f64;
        let px_delta_v = &vp_v / img_h as f64;

        let vp_upper_left = &pose.lookfrom - &(&w * focal_length) - &vp_u / 2.0 - &vp_v / 2.0;

        // Inset the pixel grid by half a unit from the viewport edges
        let px00 = &vp_upper_left + &((&px_delta_u + &px_delta_v) * 0.5);

        Camera {
            img_w,
            img_h,
            px00,
            px_delta_u,
            px_delta_v,
            rays_per_pixel,
            max_bounces,
            pose,
            rng: RefCell::new(rng),
        }
    }

    pub fn with_default_rng(
        img_w: u32,
        ar: f64,
        rays_per_pixel: u32,
        max_bounces: u32,
        vfov: f64,
        pose: CameraPose,
    ) -> Camera<ThreadRng> {
        Camera::new(
            img_w,
            ar,
            rays_per_pixel,
            max_bounces,
            vfov,
            pose,
            rand::rng(),
        )
    }

    pub fn with_default_rng_and_pose(
        img_w: u32,
        ar: f64,
        rays_per_pixel: u32,
        max_bounces: u32,
        vfov: f64,
    ) -> Camera<ThreadRng> {
        let pose = CameraPose {
            lookfrom: Vec3::zero(),
            lookat: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            vup: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        };
        Camera::<ThreadRng>::with_default_rng(img_w, ar, rays_per_pixel, max_bounces, vfov, pose)
    }

    pub fn render(&self, world: Hittables, path: impl AsRef<Path>) -> std::io::Result<()> {
        println!("Rendering image @ {}x{}...", self.img_w, self.img_h);

        let image = Image::new(self.img_w, self.img_h, |x, y| {
            let mut px = Pixel::zero();
            let rng = &mut self.rng.borrow_mut();

            for _ in 0..self.rays_per_pixel {
                let ray = self.get_ray(x, y, rng);
                px = px + self.color_ray(&ray, &world, self.max_bounces, rng);
            }

            px / self.rays_per_pixel as f64
        });

        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        write!(writer, "{}", image)?;

        Ok(())
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

        let dir = (&px_sample - &self.pose.lookfrom).norm();

        Ray3 {
            origin: self.pose.lookfrom.clone(),
            dir,
        }
    }
}
