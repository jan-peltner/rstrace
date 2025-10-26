use std::rc::Rc;

use crate::{
    aabb::AABB,
    ray::{Hit, Hittable, Ray3},
    utils::Interval,
};

#[derive(Debug)]
pub struct RotateY {
    object: Rc<dyn Hittable>,
    cos_theta: f64,
    sin_theta: f64,
    bbox: AABB,
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray3, t_range: &mut Interval) -> Option<Hit> {
        todo!()
    }

    fn bbox(&self) -> AABB {
        self.bbox
    }
}

impl RotateY {
    pub fn new(object: Rc<dyn Hittable>, angle: f64) -> Self {
        todo!()
    }

    pub fn new_rc(object: Rc<dyn Hittable>, angle: f64) -> Rc<Self> {
        Rc::from(Self::new(object, angle))
    }
}
