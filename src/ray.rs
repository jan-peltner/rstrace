use crate::{Point3, Vec3};

/// Ray in 3d space. A ray has an origin point `P3` and a direction `V3`.
pub struct Ray3 {
    origin: Point3,
    dir: Vec3,
}

impl Ray3 {
    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + &(&self.dir * t)
    }
}
