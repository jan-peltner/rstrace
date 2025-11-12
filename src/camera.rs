use crate::{
    interval::Interval,
    ray::{Hittable, Ray3},
    utils::{linear_to_gamma, map_rgb},
    vec::{Color, Pixel, Point, Vec3},
};
use core::f64;
use image::{ImageBuffer, ImageResult, Rgb};
use rand::{rngs::ThreadRng, Rng};
use std::{cell::RefCell, path::Path, rc::Rc};

pub struct CameraPose {
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
}

impl Default for CameraPose {
    fn default() -> Self {
        Self {
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
        }
    }
}

pub struct CameraIntrinsics {
    pub img_w: u32,
    pub ar: f64,
    pub rays_per_pixel: u32,
    pub max_bounces: u32,
    pub vfov: f64,
    pub defoucs_angle: f64,
    pub focus_distance: f64,
    pub background: Color,
}

impl Default for CameraIntrinsics {
    fn default() -> Self {
        Self {
            img_w: 1600,
            ar: 16.0 / 9.0,
            rays_per_pixel: 100,
            max_bounces: 10,
            vfov: 90.0,
            defoucs_angle: 0.0,
            focus_distance: 1.0,
            background: (108, 166, 193).into(),
        }
    }
}

pub type RenderResult<T> = ImageResult<T>;

// make Camera generic over R so we can potentially use different rngs later
pub struct Camera<R: Rng> {
    img_w: u32,
    img_h: u32,
    px00: Point,
    px_delta_u: Vec3,
    px_delta_v: Vec3,
    defocus_disk_radius: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    rays_per_pixel: u32,
    max_bounces: u32,
    pose: CameraPose,
    background: Color,
    rng: RefCell<R>,
}

impl<R: Rng> Camera<R> {
    pub fn new(intrinsics: CameraIntrinsics, pose: CameraPose, rng: R) -> Self {
        let img_h = Interval {
            min: 1.0,
            max: f64::INFINITY,
        }
        .clamp(intrinsics.img_w as f64 / intrinsics.ar) as u32;

        // Half angle of vertical fov -> measured from z-axis to top
        let theta = intrinsics.vfov.to_radians() / 2.0;
        let h = theta.tan();
        let vp_h = h * 2.0 * intrinsics.focus_distance; // Viewport height in world units
        let vp_w = vp_h * (intrinsics.img_w as f64 / img_h as f64);

        // Orthonormal basis (u,v,w)
        // Direction that the camera looks at but in reverse
        let w = (&pose.lookfrom - &pose.lookat).norm();
        // Perpendicular vector to both vup and w, or in other words: normal vector to the plane
        // containing w and vup
        let u = &pose.vup.cross(&w).norm();
        // use the two perpendicular vectors w and u to compute the final "up" vector v
        let v = &u.cross(&w);

        // Vectors along the edges of the viewport
        let vp_u = u * vp_w;
        let vp_v = v * vp_h;

        // Pixel spacing -> the amount of world units that one image pixel takes up on the viewport
        // Multiplying the pixel coordinates (i, j) by these deltas moves us to the
        // corresponding location on the viewport plane
        let px_delta_u = &vp_u / intrinsics.img_w as f64;
        let px_delta_v = &vp_v / img_h as f64;

        let vp_upper_left =
            &pose.lookfrom - &(&w * intrinsics.focus_distance) - &vp_u / 2.0 - &vp_v / 2.0;

        // Inset the pixel grid by half a unit from the viewport edges
        let px00 = &vp_upper_left + &((&px_delta_u + &px_delta_v) * 0.5);

        // defocus disk radius -> opposite side
        // focus distance -> adjacent side
        let defocus_disk_radius =
            (intrinsics.defoucs_angle.to_radians() / 2.0).tan() * intrinsics.focus_distance;
        let defocus_disk_u = u * defocus_disk_radius;
        let defocus_disk_v = v * defocus_disk_radius;

        Camera {
            img_w: intrinsics.img_w,
            img_h,
            px00,
            px_delta_u,
            px_delta_v,
            defocus_disk_radius,
            defocus_disk_u,
            defocus_disk_v,
            rays_per_pixel: intrinsics.rays_per_pixel,
            max_bounces: intrinsics.max_bounces,
            pose,
            background: intrinsics.background,
            rng: RefCell::new(rng),
        }
    }

    pub fn render(&self, world: Rc<dyn Hittable>, path: impl AsRef<Path>) -> RenderResult<()> {
        println!("Rendering image @ {}x{}...", self.img_w, self.img_h);

        let mut image = ImageBuffer::new(self.img_w, self.img_h);

        for y in 0..self.img_h {
            for x in 0..self.img_w {
                let mut px = Pixel::zero();
                let rng = &mut self.rng.borrow_mut();

                for _ in 0..self.rays_per_pixel {
                    let ray = self.get_ray(x, y, rng);
                    px = px + self.color_ray(&ray, world.clone(), self.max_bounces, rng);
                }

                px = px / self.rays_per_pixel as f64;

                px.x = map_rgb(linear_to_gamma(px.x));
                px.y = map_rgb(linear_to_gamma(px.y));
                px.z = map_rgb(linear_to_gamma(px.z));

                image.put_pixel(x, y, Rgb::from([px.x as u8, px.y as u8, px.z as u8]));
            }

            let scanline = y + 1;
            if scanline % 100 == 0 {
                println!("Scanlines processed: {}/{}", scanline, self.img_h);
            }
        }

        image.save(path)
    }

    fn color_ray(
        &self,
        ray: &Ray3,
        world: Rc<dyn Hittable>,
        bounces_left: u32,
        rng: &mut R,
    ) -> Pixel {
        if bounces_left <= 0 {
            return Pixel::zero();
        }

        let mut t_range = Interval {
            min: 0.001,
            max: f64::INFINITY,
        };

        if let Some(hit) = world.hit(&ray, &mut t_range) {
            // if we hit an emissive material we won't scatter and we will directly return the
            // emissive color up the stack
            let emission_color = hit.mat.emit(hit.uv, &hit.p);

            if let Some(scatter) = hit.mat.scatter(ray, &hit, rng) {
                return &self.color_ray(
                    &Ray3::with_time(hit.p, scatter.scattered_ray.dir, scatter.scattered_ray.time),
                    world,
                    bounces_left - 1,
                    rng,
                ) * &scatter.attenuation;
            } else {
                return Pixel {
                    x: emission_color.x,
                    y: emission_color.y,
                    z: emission_color.z,
                };
            }
        } else {
            return Pixel {
                x: self.background.x,
                y: self.background.y,
                z: self.background.z,
            };
        }
    }

    fn get_ray(&self, i: u32, j: u32, rng: &mut R) -> Ray3 {
        let square_offset = Vec3::rand_unit_square_offset(rng);

        let px_sample = &(&self.px00 + &(&self.px_delta_u * (i as f64 + square_offset.x)))
            + &(&self.px_delta_v * (j as f64 + square_offset.y));

        let origin = if self.defocus_disk_radius <= 0.0 {
            self.pose.lookfrom.clone()
        } else {
            self.defocus_disk_sample(rng)
        };

        let dir = (&px_sample - &origin).norm();
        let time = rng.random::<f64>();

        Ray3::with_time(
            if self.defocus_disk_radius <= 0.0 {
                self.pose.lookfrom.clone()
            } else {
                self.defocus_disk_sample(rng)
            },
            dir,
            time,
        )
    }

    fn defocus_disk_sample(&self, rng: &mut R) -> Point {
        let p = Vec3::rand_in_unit_disc(rng);
        &self.pose.lookfrom + &(&(&self.defocus_disk_u * p.x) + &(&self.defocus_disk_v * p.y))
    }
}

impl Default for Camera<ThreadRng> {
    fn default() -> Self {
        Self::with_default_rng_and_pose(CameraIntrinsics::default())
    }
}

impl Camera<ThreadRng> {
    pub fn with_default_rng(intrinsics: CameraIntrinsics, pose: CameraPose) -> Self {
        Self::new(intrinsics, pose, rand::rng())
    }

    pub fn with_default_rng_and_pose(intrinsics: CameraIntrinsics) -> Self {
        Self::with_default_rng(intrinsics, CameraPose::default())
    }
}
