use std::ops::Add;

use crate::{
    ray::Ray3,
    utils::Interval,
    vec::{Point, Vec3},
};

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn empty() -> Self {
        Self::new(Interval::empty(), Interval::empty(), Interval::empty())
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

    pub fn from_bboxes(a: &AABB, b: &AABB) -> Self {
        Self {
            x: Interval::union(&a.x, &b.x),
            y: Interval::union(&a.y, &b.y),
            z: Interval::union(&a.z, &b.z),
        }
    }

    pub fn maybe_pad(mut self) -> Self {
        let delta: f64 = 0.0001;
        if self.x.size() <= delta {
            self.x = self.x.expand(delta);
        }
        if self.y.size() <= delta {
            self.y = self.y.expand(delta);
        }
        if self.z.size() <= delta {
            self.z = self.z.expand(delta);
        }
        self
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

impl Add<Vec3> for AABB {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
