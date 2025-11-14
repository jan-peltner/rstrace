use core::f64;
use std::sync::Arc;

use rand::Rng;

use crate::{
    aabb::AABB,
    interval::Interval,
    material::Material,
    ray::{Hit, Hittable, Ray3},
    vec::Vec3,
};

pub struct ConstantMedium<R: Rng> {
    boundary: Arc<dyn Hittable<R>>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl<R: Rng> std::fmt::Debug for ConstantMedium<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConstantMedium")
            .field("boundary", &self.boundary)
            .field("neg_inv_density", &self.neg_inv_density)
            .field("phase", &self.phase_function)
            .field("rng", &"<hidden>")
            .finish()
    }
}

impl<R: Rng> Hittable<R> for ConstantMedium<R> {
    fn hit(&self, ray: &Ray3, t_range: &mut Interval, rng: &mut R) -> Option<Hit> {
        let mut entry = match self.boundary.hit(ray, &mut Interval::universe(), rng) {
            Some(hit) => hit,
            None => return None,
        };

        let mut exit = match self.boundary.hit(
            ray,
            &mut Interval {
                min: entry.t + 0.0001,
                max: f64::INFINITY,
            },
            rng,
        ) {
            Some(hit) => hit,
            None => return None,
        };

        if entry.t < t_range.min {
            entry.t = t_range.min
        };
        if exit.t > t_range.max {
            exit.t = t_range.max
        };

        if entry.t >= exit.t {
            return None;
        }

        if entry.t < 0.0 {
            entry.t = 0.0
        }

        let ray_len = ray.dir.len();
        let dist_in_boundary = (exit.t - entry.t) * ray_len;
        let hit_dist = self.neg_inv_density * rng.random::<f64>().ln();

        // hit occured outside of medium's boundary
        if hit_dist > dist_in_boundary {
            return None;
        }

        let t = entry.t + hit_dist / ray_len;

        Some(Hit {
            p: ray.at(t),
            normal: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            uv: (1.0, 1.0),
            front_face: true,
            t,
            mat: self.phase_function.clone(),
        })
    }

    fn bbox(&self) -> AABB {
        self.boundary.bbox()
    }
}

impl<R: Rng> ConstantMedium<R> {
    pub fn new(
        boundary: Arc<dyn Hittable<R>>,
        density: f64,
        phase_function: Arc<dyn Material>,
    ) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function,
        }
    }

    pub fn new_arc(
        boundary: Arc<dyn Hittable<R>>,
        density: f64,
        phase_function: Arc<dyn Material>,
    ) -> Arc<Self> {
        Arc::new(Self::new(boundary, density, phase_function))
    }
}
