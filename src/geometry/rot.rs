use std::rc::Rc;

use crate::{
    aabb::AABB,
    ray::{Hit, Hittable, Ray3},
    utils::Interval,
};

#[derive(Debug)]
pub struct RotateY<H: Hittable> {
    object: H,
    cos_theta: f64,
    sin_theta: f64,
    bbox: AABB,
}

impl<H: Hittable> Hittable for RotateY<H> {
    fn hit(&self, ray: &Ray3, t_range: &mut Interval) -> Option<Hit> {
        todo!()
    }

    fn bbox(&self) -> AABB {
        self.bbox
    }
}

impl<H: Hittable> RotateY<H> {
    pub fn new(object: H, angle: f64) -> Self {
        todo!()
    }

    pub fn new_rc(object: H, angle: f64) -> Rc<Self> {
        Rc::from(Self::new(object, angle))
    }
}
