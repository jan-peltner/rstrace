use std::rc::Rc;

use crate::{
    aabb::AABB,
    ray::{Hit, Hittable, Ray3},
    utils::Interval,
    vec::Point,
};

#[derive(Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug)]
pub struct Rotate {
    object: Rc<dyn Hittable>,
    cos_theta: f64,
    sin_theta: f64,
    axis: Axis,
    bbox: AABB,
}

impl Hittable for Rotate {
    fn hit(&self, ray: &Ray3, t_range: &mut Interval) -> Option<Hit> {
        // math for rotating about the y-axis:
        // rotate the incident ray by -θ (inverse rotation) since we rotate the ray rather than the geometry itself
        // we rotate in the xz-plane starting from the positive x-axis
        // x = r * cos(α)
        // z = r * sin(α)
        // x' = r * cos(α - θ) = r * (cos(α)cos(θ) + sin(α)sin(θ)) = x * cos(θ) + z * sin(θ)
        // y' = y
        // z' = r * sin(α - θ) = r * (sin(α)cos(θ) - sin(θ)cos(α)) = z * cos(θ) - x * sin(θ)

        // pass in -sin_theta for inverse rotation
        let (origin, dir) = match self.axis {
            Axis::X => (
                ray.origin.rot_x(self.cos_theta, -self.sin_theta),
                ray.dir.rot_x(self.cos_theta, -self.sin_theta),
            ),
            Axis::Y => (
                ray.origin.rot_y(self.cos_theta, -self.sin_theta),
                ray.dir.rot_y(self.cos_theta, -self.sin_theta),
            ),
            Axis::Z => (
                ray.origin.rot_z(self.cos_theta, -self.sin_theta),
                ray.dir.rot_z(self.cos_theta, -self.sin_theta),
            ),
        };

        let rotated_ray = Ray3::with_time(origin, dir, ray.time);

        if let Some(mut hit) = self.object.hit(&rotated_ray, t_range) {
            // rotate back the intersection point and surface normal (positive sin_theta)
            // mathematically rotation doesn't change the length of a vector but for robustness sake (floating
            // point inaccuracies) we normalize the surface normal again
            match self.axis {
                Axis::X => {
                    hit.p = hit.p.rot_x(self.cos_theta, self.sin_theta);
                    hit.normal = hit.normal.rot_x(self.cos_theta, self.sin_theta).norm();
                }
                Axis::Y => {
                    hit.p = hit.p.rot_y(self.cos_theta, self.sin_theta);
                    hit.normal = hit.normal.rot_y(self.cos_theta, self.sin_theta).norm();
                }
                Axis::Z => {
                    hit.p = hit.p.rot_z(self.cos_theta, self.sin_theta);
                    hit.normal = hit.normal.rot_z(self.cos_theta, self.sin_theta).norm();
                }
            }
            Some(hit)
        } else {
            None
        }
    }

    fn bbox(&self) -> AABB {
        self.bbox
    }
}

impl Rotate {
    pub fn new(object: Rc<dyn Hittable>, angle: f64, axis: Axis) -> Self {
        let rad = angle.to_radians();
        let cos_theta = rad.cos();
        let sin_theta = rad.sin();

        let obj_bbox = object.bbox();
        let mut min = Point {
            x: f64::INFINITY,
            y: f64::INFINITY,
            z: f64::INFINITY,
        };
        let mut max = Point {
            x: f64::NEG_INFINITY,
            y: f64::NEG_INFINITY,
            z: f64::NEG_INFINITY,
        };

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * obj_bbox.x.max + (1 - i) as f64 * obj_bbox.x.min;
                    let y = j as f64 * obj_bbox.y.max + (1 - j) as f64 * obj_bbox.y.min;
                    let z = k as f64 * obj_bbox.z.max + (1 - k) as f64 * obj_bbox.z.min;

                    // bbox vertex
                    let mut v = Point { x, y, z };

                    // rotate the vertex by θ
                    match axis {
                        Axis::X => v = v.rot_x(cos_theta, sin_theta),
                        Axis::Y => v = v.rot_y(cos_theta, sin_theta),
                        Axis::Z => v = v.rot_z(cos_theta, sin_theta),
                    }

                    min.x = min.x.min(v.x);
                    max.x = max.x.max(v.x);
                    min.y = min.y.min(v.y);
                    max.y = max.y.max(v.y);
                    min.z = min.z.min(v.z);
                    max.z = max.z.max(v.z);
                }
            }
        }
        Self {
            object,
            cos_theta,
            sin_theta,
            axis,
            bbox: AABB::from_points(&min, &max),
        }
    }

    pub fn new_rc(object: Rc<dyn Hittable>, angle: f64, axis: Axis) -> Rc<Self> {
        Rc::from(Self::new(object, angle, axis))
    }
}
