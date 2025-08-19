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
