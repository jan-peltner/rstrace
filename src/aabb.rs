use crate::{
    ray::{Hit, Hittable, Ray3},
    utils::Interval,
    vec::Point,
};

pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_points(a: &Point, b: &Point) -> Self {
        Self {
            x: if a.x <= b.x {
                Interval { min: a.x, max: b.x }
            } else {
                Interval { min: b.x, max: a.x }
            },
            y: if a.y <= b.y {
                Interval { min: a.y, max: b.y }
            } else {
                Interval { min: b.y, max: a.y }
            },
            z: if a.z <= b.z {
                Interval { min: a.z, max: b.z }
            } else {
                Interval { min: b.z, max: a.z }
            },
        }
    }
}

impl Hittable for AABB {
    fn hit(&self, ray: &Ray3, t_range: &mut Interval) -> Option<Hit> {
        todo!()
    }
}
