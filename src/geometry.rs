use crate::{
    ray::{Hit, Hittable, Ray3},
    vec::Point3,
};

pub struct Sphere {
    pub radius: f64,
    pub center: Point3,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray3, tmin: f64, tmax: f64) -> Option<Hit> {
        let cq = &ray.origin - &self.center;
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
        if eval <= tmin || eval >= tmax {
            eval = lhs + rhs;
            if eval <= tmin || eval >= tmax {
                return None;
            }
        }

        let intersection_point = ray.at(eval);

        // Dividing by radius normalizes the vector -> more performant than calling .norm()
        let outward_normal = (&intersection_point - &self.center) / self.radius;

        return Some(Hit {
            p: intersection_point.clone(),
            t: eval,
            // Determine if the ray is hitting the front face or back face of the sphere.
            // A front face hit occurs when the ray's direction is generally opposite to the
            // surface's inherent outward normal. A back face hit occurs when they are generally
            // in the same direction (meaning the ray is inside the object and trying to exit)
            normal: if ray.dir.dot(&outward_normal) < 0.0 {
                outward_normal
            } else {
                outward_normal * -1.0
            },
        });
    }
}

pub struct Hittables {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Hittables {
    pub fn check_hit(&self, ray: &Ray3, tmin: f64, tmax: f64) -> Option<Hit> {
        let mut closest_hit: Option<Hit> = None;
        let mut closest_hit_t: f64 = tmax;

        for hittable in self.objects.iter() {
            if let Some(hit) = hittable.hit(ray, tmin, closest_hit_t) {
                closest_hit_t = hit.t;
                closest_hit = Some(hit);
            }
        }
        closest_hit
    }
}
