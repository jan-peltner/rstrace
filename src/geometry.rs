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

        return Some(Hit {
            p: intersection_point.clone(),
            t: eval,
            normal: (&intersection_point - &self.center).norm(),
        });
    }
}
