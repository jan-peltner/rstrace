use crate::{ray::Ray3, utils::Interval, vec::Point};

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

    pub fn axis_interval(&self, index: usize) -> &Interval {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("out of bounds AABB axis interval: {index}"),
        }
    }

    // We don't implement the `Hittable` trait because we can't (and don't need to) compute the
    // associated hit data. We only care about whether the ray has intersected the aabb or not.
    pub fn hit(&self, ray: &Ray3, t_range: &mut Interval) -> bool {
        for (idx, comp) in ray.dir.iter().enumerate() {
            let interval = self.axis_interval(idx);
        }
        todo!()
    }
}
