use core::f64::{self, consts::PI};
use std::rc::Rc;

use crate::{
    aabb::AABB,
    material::Material,
    ray::{Hit, Hittable, Ray3},
    utils::Interval,
    vec::{Point, Vec3},
};

#[derive(Debug)]
pub struct Sphere {
    pub radius: f64,
    pub center: Ray3,
    mat: Rc<dyn Material>,
    bbox: AABB,
    // normalized azimuthal rotation
    naz_rot: f64,
}

impl Sphere {
    pub fn rotate_texture(&mut self, rad: f64) {
        self.naz_rot = (rad / (2.0 * PI)) % 1.0;
    }
    pub fn new(radius: f64, center: Point, mat: Rc<dyn Material>) -> Rc<Self> {
        Rc::from(Self {
            radius,
            center: Ray3::without_time(center, Vec3::zero()),
            mat,
            bbox: Sphere::aabb(&center, radius),
            naz_rot: 0.0,
        })
    }
    fn aabb(center: &Point, radius: f64) -> AABB {
        let radius_vec = Vec3::splat(radius);
        AABB::from_points(&(center - &radius_vec), &(center + &radius_vec))
    }

    pub fn add_movement(mut self, center: Point) -> Self {
        let new_center = &center - &self.center.origin;
        self.center = Ray3::without_time(self.center.origin, new_center.clone());

        // generate bounding box that spans the entire path of the sphere
        let start = Self::aabb(&self.center.origin, self.radius);
        let end = Self::aabb(&center, self.radius);
        self.bbox = AABB::from_points(
            &Point {
                x: start.x.min.min(end.x.min),
                y: start.y.min.min(end.y.min),
                z: start.z.min.min(end.z.min),
            },
            &Point {
                x: start.x.max.max(end.x.max),
                y: start.y.max.max(end.y.max),
                z: start.z.max.max(end.z.max),
            },
        );
        self
    }

    fn get_uv(&self, intersection: &Point) -> (f64, f64) {
        let polar = (intersection.y * -1.0).acos();
        let azimuth = (intersection.z * -1.0).atan2(intersection.x) + PI;

        // normalize to [0,1]
        let u = azimuth / (2.0 * PI);
        let v = polar / PI;

        // [0,1)
        ((u + self.naz_rot) % 1.0, v % 1.0)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray3, t_range: &mut Interval) -> Option<Hit> {
        let current_center = self.center.at(ray.time);
        let cq = &ray.origin - &current_center;
        let a = ray.dir.dot(&ray.dir);
        let b = (&ray.dir * 2.0).dot(&cq);
        let c = &cq.dot(&cq) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let lhs = -b / (2.0 * a);
        let rhs = sqrtd / (2.0 * a);

        let mut eval = lhs - rhs;
        if !t_range.surrounds(eval) {
            eval = lhs + rhs;
            if !t_range.surrounds(eval) {
                return None;
            }
        }

        let intersection_point = ray.at(eval);

        // Dividing by radius normalizes the vector -> more performant than calling .norm()
        let outward_normal = (&intersection_point - &current_center) / self.radius;

        // Determine if the ray is hitting the front face or back face of the sphere.
        // A front face hit occurs when the ray's direction is generally opposite to the
        // surface's inherent outward normal. A back face hit occurs when they are generally
        // in the same direction (meaning the ray is inside the object and trying to exit)
        let front_face = ray.dir.dot(&outward_normal) < 0.0;

        return Some(Hit {
            p: intersection_point.clone(),
            t: eval,
            uv: self.get_uv(&outward_normal),
            normal: if front_face {
                outward_normal
            } else {
                outward_normal * -1.0
            },
            front_face,
            mat: self.mat.clone(),
        });
    }

    fn bbox(&self) -> AABB {
        self.bbox
    }
}
