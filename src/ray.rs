use crate::vec::{Point3, Vec3};

/// Ray in 3d space. A ray has an origin point `P3` and a direction `V3`.
#[derive(Clone)]
pub struct Ray3 {
    pub origin: Point3,
    pub dir: Vec3,
}

impl Ray3 {
    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + &(&self.dir * t)
    }
}

pub struct Hit {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray3, tmin: f64, tmax: f64) -> Option<Hit>;
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
