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
    pub fn hit(&self, ray: &Ray3, mut t_range: Interval) -> bool {
        for (idx, (dir_comp, orig_comp)) in ray.dir.iter().zip(ray.origin.iter()).enumerate() {
            let interval = self.axis_interval(idx);

            let inv_dir_comp = 1.0 / dir_comp;

            let t0 = (interval.min - orig_comp) * inv_dir_comp;
            let t1 = (interval.max - orig_comp) * inv_dir_comp;

            if t0 < t1 {
                if t0 > t_range.min {
                    t_range.min = t0;
                }
                if t1 < t_range.max {
                    t_range.max = t1;
                }
            } else {
                if t1 > t_range.min {
                    t_range.min = t1;
                }
                if t0 < t_range.max {
                    t_range.max = t0;
                }
            }

            if t_range.max <= t_range.min {
                return false;
            }
        }
        true
    }
}
