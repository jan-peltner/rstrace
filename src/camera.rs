use crate::{
    interval::Interval,
    ray::{Hittable, Ray3},
    utils::{linear_to_gamma, map_rgb},
    vec::{Color, Pixel, Point, Vec3},
};
use core::f64;
use image::{ImageBuffer, ImageResult, Rgb};
use rand::{
    rngs::{SmallRng, ThreadRng},
    Rng, SeedableRng,
};
use std::{marker::PhantomData, path::Path, sync::Arc, thread, time::Instant};

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
    // use function pointer for PhantomData<T> so we get the Sync + Send auto trait implementations
    rng_marker: PhantomData<fn() -> R>,
    rng_base_seed: Option<u64>,
}

impl<R: Rng> Camera<R> {
    fn new(intrinsics: CameraIntrinsics, pose: CameraPose, seed: Option<u64>) -> Self {
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
            rng_marker: PhantomData,
            rng_base_seed: seed,
        }
    }

    fn render_with<F>(
        &self,
        world: Arc<dyn Hittable<R>>,
        path: impl AsRef<Path>,
        make_rng: F,
    ) -> RenderResult<()>
    where
        F: Fn(u64) -> R + Send + Clone + Copy + 'static,
    {
        let start = Instant::now();
        println!("Rendering image @ {}x{}...", self.img_w, self.img_h);

        let num_cpus = num_cpus::get();
        println!("{num_cpus} thread(s) available!");

        let rows_per_thread = (self.img_h as f64 / num_cpus as f64).ceil() as usize;
        println!("Processing {rows_per_thread} rows per thread!");

        let mut pixels = thread::scope(|scope| {
            let handles: Vec<_> = (0..num_cpus)
                .map(|i| {
                    scope.spawn({
                        let world = world.clone();
                        move || {
                            println!("#{} thread spawned!", i + 1);
                            let mut rows: Vec<(usize, Vec<Pixel>)> =
                                Vec::with_capacity(rows_per_thread);
                            let mut rng = make_rng(i as u64);
                            let start = i * rows_per_thread;
                            let end = (self.img_h as usize).min(start + rows_per_thread);
                            for y in start..end {
                                let mut row: Vec<Pixel> = Vec::with_capacity(self.img_w as usize);
                                for x in 0..self.img_w {
                                    let mut px = Pixel::zero();

                                    for _ in 0..self.rays_per_pixel {
                                        let ray = self.get_ray(x, y as u32, &mut rng);
                                        px = px
                                            + self.color_ray(
                                                &ray,
                                                world.clone(),
                                                self.max_bounces,
                                                &mut rng,
                                            );
                                    }

                                    px = px / self.rays_per_pixel as f64;

                                    px.x = map_rgb(linear_to_gamma(px.x));
                                    px.y = map_rgb(linear_to_gamma(px.y));
                                    px.z = map_rgb(linear_to_gamma(px.z));

                                    row.push(px);
                                }
                                rows.push((y, row));
                            }
                            rows
                        }
                    })
                })
                .collect();

            handles
                .into_iter()
                .map(|h| h.join().expect("Thread panicked"))
                .flatten()
                .collect::<Vec<(usize, Vec<Pixel>)>>()
        });

        pixels.sort_by_key(|(y, _)| *y);

        let end = start.elapsed().as_secs_f64();
        println!("Computed rays in {:.2} seconds", end);

        let mut image = ImageBuffer::new(self.img_w, self.img_h);

        for (y, row) in pixels.into_iter() {
            for (x, px) in row.into_iter().enumerate() {
                image.put_pixel(
                    x as u32,
                    y as u32,
                    Rgb::from([px.x as u8, px.y as u8, px.z as u8]),
                );
            }
        }

        image.save(path)
    }

    fn color_ray(
        &self,
        ray: &Ray3,
        world: Arc<dyn Hittable<R>>,
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

        if let Some(hit) = world.hit(&ray, &mut t_range, rng) {
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

impl Camera<ThreadRng> {
    pub fn new_default_rng(intrinsics: CameraIntrinsics, pose: CameraPose) -> Self {
        Self::new(intrinsics, pose, None)
    }

    pub fn render(
        &self,
        world: Arc<dyn Hittable<ThreadRng>>,
        path: impl AsRef<Path>,
    ) -> RenderResult<()> {
        let make_rng = |_| rand::rng();
        self.render_with(world, path, make_rng)
    }

    pub fn get_rng(&self) -> ThreadRng {
        rand::rng()
    }
}

impl Default for Camera<ThreadRng> {
    fn default() -> Self {
        Self::new_default_rng(CameraIntrinsics::default(), CameraPose::default())
    }
}

impl Camera<SmallRng> {
    pub fn new_seeded_rng(intrinsics: CameraIntrinsics, pose: CameraPose, seed: u64) -> Self {
        Self::new(intrinsics, pose, Some(seed))
    }

    pub fn render(
        &self,
        world: Arc<dyn Hittable<SmallRng>>,
        path: impl AsRef<Path>,
    ) -> RenderResult<()> {
        let base_seed = self.rng_base_seed.expect("No RNG seed");
        let make_rng = move |tid| {
            let thread_seed = base_seed.wrapping_add(tid);
            SmallRng::seed_from_u64(thread_seed)
        };
        self.render_with(world, path, make_rng)
    }

    pub fn get_rng(&self) -> SmallRng {
        SmallRng::seed_from_u64(self.rng_base_seed.expect("No RNG seed"))
    }
}

impl Default for Camera<SmallRng> {
    fn default() -> Self {
        Self::new_seeded_rng(CameraIntrinsics::default(), CameraPose::default(), 316u64)
    }
}
