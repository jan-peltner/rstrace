use core::f64::{self, consts::PI};
use std::rc::Rc;

use crate::{
    aabb::AABB,
    material::{Dielectric, Lambertian, Material, Metal},
    ray::{Hit, Hittable, Ray3},
    texture::{SolidTex, Texture},
    utils::Interval,
    vec::{Color, Point, Vec3},
};

#[derive(Debug)]
pub struct Sphere {
    pub radius: f64,
    pub center: Ray3,
    mat: Rc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    fn aabb(center: &Point, radius: f64) -> AABB {
        let radius_vec = Vec3::splat(radius);
        AABB::from_points(&(center - &radius_vec), &(center + &radius_vec))
    }

    fn new_lambertian<T: Texture + 'static>(radius: f64, center: Point, tex: T) -> Self {
        let mat = Rc::new(Lambertian { tex });
        Self {
            radius,
            center: Ray3::without_time(center.clone(), Vec3::zero()),
            mat,
            bbox: Self::aabb(&center, radius),
        }
    }

    pub fn lambertian(radius: f64, center: Point) -> Self {
        Self::new_lambertian(
            radius,
            center,
            SolidTex::new(Color {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            }),
        )
    }

    pub fn lambertian_with_albedo(radius: f64, center: Point, albedo: Color) -> Self {
        Self::new_lambertian(radius, center, SolidTex::new(albedo))
    }

    pub fn lambertian_with_texture<T: Texture + 'static>(
        radius: f64,
        center: Point,
        tex: T,
    ) -> Self {
        Self::new_lambertian(radius, center, tex)
    }

    fn new_metal(radius: f64, center: Point, albedo: Vec3, fuzz: f64) -> Self {
        let mat = Rc::new(Metal { albedo, fuzz });
        Self {
            radius,
            center: Ray3::without_time(center.clone(), Vec3::zero()),
            mat,
            bbox: Self::aabb(&center, radius),
        }
    }

    pub fn metal(radius: f64, center: Point, fuzz: f64) -> Self {
        Self::new_metal(
            radius,
            center,
            Color {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
            fuzz,
        )
    }

    pub fn metal_with_albedo(radius: f64, center: Point, albedo: Color, fuzz: f64) -> Self {
        Self::new_metal(radius, center, albedo, fuzz)
    }

    fn new_dielectric(radius: f64, center: Point, refractive_index: f64) -> Self {
        let mat = Rc::new(Dielectric { refractive_index });
        Self {
            radius,
            center: Ray3::without_time(center.clone(), Vec3::zero()),
            mat,
            bbox: Self::aabb(&center, radius),
        }
    }

    pub fn dielectric(radius: f64, center: Point, refractive_index: f64) -> Self {
        Self::new_dielectric(radius, center, refractive_index)
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

    fn get_uv(intersection: &Point) -> (f64, f64) {
        let polar = (intersection.y * -1.0).acos();
        let azimuth = (intersection.z * -1.0).atan2(intersection.x) + PI;

        // normalize to [0,1]
        (azimuth / (2.0 * PI), polar / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray3, t_range: &mut Interval) -> Option<Hit<'_>> {
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
        let front_face = ray.dir.dot(&outward_normal) < 0.0;

        return Some(Hit {
            p: intersection_point.clone(),
            t: eval,
            uv: Self::get_uv(&outward_normal),
            // Determine if the ray is hitting the front face or back face of the sphere.
            // A front face hit occurs when the ray's direction is generally opposite to the
            // surface's inherent outward normal. A back face hit occurs when they are generally
            // in the same direction (meaning the ray is inside the object and trying to exit)
            normal: if front_face {
                outward_normal
            } else {
                outward_normal * -1.0
            },
            front_face,
            mat: &*self.mat,
        });
    }

    fn bbox(&self) -> AABB {
        self.bbox
    }
}
